package services

import (
	"balancer/backends"
	"balancer/executor"
	"balancer/grpcmodels"
	"balancer/plan"
	"balancer/result"
	"balancer/utils"
	"context"
	"maps"

	"github.com/google/uuid"
	"go.uber.org/zap"
	"google.golang.org/grpc"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
	"google.golang.org/protobuf/types/known/emptypb"
)

type BackendServiceServer struct {
	backendsInfo *backends.BackendsInfo
	executor     *executor.PipelineExecutor
	planner      *plan.ExecutionPlanner
	grpcmodels.UnsafeGrpcBackendServiceServer
	logger *zap.SugaredLogger
}

func NewBackendServiceServer(
	backendsInfo *backends.BackendsInfo,
	executor *executor.PipelineExecutor,
	planner *plan.ExecutionPlanner,
	logger *zap.SugaredLogger,
) *BackendServiceServer {
	return &BackendServiceServer{
		backendsInfo: backendsInfo,
		executor:     executor,
		planner:      planner,
		logger:       logger,
	}
}

func (this *BackendServiceServer) ExecutePipeline(
	request *grpcmodels.GrpcProxyPipelineExecutionRequest,
	server grpc.ServerStreamingServer[grpcmodels.GrpcPipelinePartExecutionResult],
) error {
	loggerRes := utils.CreateLoggerAttachedToActivity(this.logger)
	if loggerRes.IsErr() {
		return loggerRes.Err()
	}

	logger := loggerRes.Ok()

	urlsRes := backends.GetBackendsUrls()
	if urlsRes.IsErr() {
		return urlsRes.Err()
	}

	res := this.backendsInfo.UpdateBackendsInfo(*urlsRes.Ok())
	if res.IsErr() {
		return status.Errorf(codes.Internal, "failed to update backends information: %s", res.Err().Error())
	}

	logger.Infow("Update backends info", "backends", this.backendsInfo.GetPipelinePartsToBackendUrls())

	planRes := this.planner.CreatePlan(request.Pipeline)
	if planRes.IsErr() {
		return status.Errorf(codes.Internal, "failed to create an execution plan: %s", res.Err().Error())
	}

	logger.Infow("Created an execution plan", "plan", planRes.Ok().String())

	var initialContextValuesIds []uuid.UUID
	for _, id := range request.ContextValuesIds {
		contextValueId, err := uuid.Parse(id.Guid)
		if err != nil {
			return status.Errorf(codes.InvalidArgument, "failed to parse uuid %s", id.Guid)
		}

		initialContextValuesIds = append(initialContextValuesIds, contextValueId)
	}

	logger.Infow("Initial context values", "context_values", initialContextValuesIds)

	pipelinePartsResultsChannel := make(chan *grpcmodels.GrpcPipelinePartExecutionResult, 100)
	errorChannel := make(chan result.Result[uuid.UUID])

	go func() {
		errorChannel <- this.executor.Execute(planRes.Ok(), initialContextValuesIds, pipelinePartsResultsChannel)
		close(errorChannel)
	}()

	for pipelineExecutionResult := range pipelinePartsResultsChannel {
		err := server.Send(pipelineExecutionResult)
		if err != nil {
			return status.Errorf(codes.Internal, "error happened during sending pipeline part result: %s", err.Error())
		} else {
			logger.Infow("Sent a pipeline execution result")
		}
	}

	executionResult := <-errorChannel
	if executionResult.IsErr() {
		return status.Errorf(codes.Internal, "error happened during pipeline execution %s", executionResult.Err())
	}

	logger.Infow("Finished execution", "execution_id", executionResult.Ok())

	return nil
}

func (this *BackendServiceServer) GetContextValue(
	_ context.Context,
	request *grpcmodels.GrpcGetContextValueRequest,
) (*grpcmodels.GrpcGuid, error) {
	executionContextValues := this.getExecutionContextValues(request.ExecutionId)
	if executionContextValues.IsErr() {
		return nil, executionContextValues.Err()
	}

	id, ok := (*executionContextValues.Ok())[request.Key.Name]
	if !ok {
		msg := "context value for execution id %s and context value key %s are not found"
		return nil, status.Errorf(codes.NotFound, msg, request.ExecutionId.Guid, request.Key.Name)
	}

	return &grpcmodels.GrpcGuid{Guid: id.String()}, nil
}

func (this *BackendServiceServer) getExecutionContextValues(id *grpcmodels.GrpcGuid) result.Result[map[string]uuid.UUID] {
	executionId, err := uuid.Parse(id.Guid)
	if err != nil {
		err := status.Errorf(codes.InvalidArgument, "failed to parse uuid %s", id.Guid)
		return result.Err[map[string]uuid.UUID](err)
	}

	executionContextValues, ok := this.executor.GetContextValues(executionId)
	if !ok {
		err := status.Errorf(codes.NotFound, "context values for execution id %s are not found", executionId)
		return result.Err[map[string]uuid.UUID](err)
	}

	return result.Ok(&executionContextValues)
}

func (this *BackendServiceServer) DropExecutionResult(context context.Context, id *grpcmodels.GrpcGuid) (*emptypb.Empty, error) {
	executionId, err := uuid.Parse(id.Guid)
	if err != nil {
		return nil, status.Errorf(codes.InvalidArgument, "failed to parse uuid %s", id.Guid)
	}

	this.executor.DropExecutionResult(executionId)
	this.logger.Infow("Dropped execution id", "execution_id", executionId)

	return &emptypb.Empty{}, nil
}

func (this *BackendServiceServer) GetBackendInfo(context.Context, *emptypb.Empty) (*grpcmodels.GrpcFicusBackendInfo, error) {
	var pipelineParts []*grpcmodels.GrpcPipelinePartDescriptor

	for partName := range maps.Keys(this.backendsInfo.GetPipelinePartsToBackendUrls()) {
		pipelineParts = append(pipelineParts, &grpcmodels.GrpcPipelinePartDescriptor{Name: partName})
	}

	return &grpcmodels.GrpcFicusBackendInfo{Name: "GO_BALANCER", PipelineParts: pipelineParts}, nil
}

func (this *BackendServiceServer) GetAllContextValues(
	_ context.Context,
	id *grpcmodels.GrpcGuid,
) (*grpcmodels.GrpcGetAllContextValuesResult, error) {
	executionContextValues := this.getExecutionContextValues(id)
	if executionContextValues.IsErr() {
		return nil, executionContextValues.Err()
	}

	var ids []*grpcmodels.GrpcGuid

	for _, id := range *executionContextValues.Ok() {
		ids = append(ids, &grpcmodels.GrpcGuid{Guid: id.String()})
	}

	return &grpcmodels.GrpcGetAllContextValuesResult{ContextValues: ids}, nil
}
