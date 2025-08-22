package services

import (
	"balancer/backends"
	"balancer/contextvalues"
	"balancer/executor"
	"balancer/grpcmodels"
	"balancer/result"
	"context"
	"maps"

	"github.com/google/uuid"
	"google.golang.org/grpc"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
	"google.golang.org/protobuf/types/known/emptypb"
)

type BackendServiceServer struct {
	urls         []string
	backendsInfo *backends.BackendsInfo
	executor     *executor.PipelineExecutor
	grpcmodels.UnsafeGrpcBackendServiceServer
}

func NewBackendServiceServer(urls []string, storage *contextvalues.Storage) *BackendServiceServer {
	backendsInfo := backends.NewBackendsInfo()
	return &BackendServiceServer{urls: urls, backendsInfo: backendsInfo, executor: executor.NewPipelineExecutor(backendsInfo, storage)}
}

func (this *BackendServiceServer) ExecutePipeline(
	request *grpcmodels.GrpcProxyPipelineExecutionRequest,
	server grpc.ServerStreamingServer[grpcmodels.GrpcPipelinePartExecutionResult],
) error {
	res := this.backendsInfo.UpdateBackendsInfo(this.urls)
	if res.IsErr() {
		return status.Errorf(codes.Internal, "failed to update backends information: %s", res.Err().Error())
	}

	return status.Errorf(codes.Unimplemented, "method ExecutePipeline not implemented")
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
		err := status.Errorf(codes.InvalidArgument, "failed to parse uuis %s", id.Guid)
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
		return nil, status.Errorf(codes.InvalidArgument, "failed to parse uuis %s", id.Guid)
	}

	this.executor.DropExecutionResult(executionId)

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
