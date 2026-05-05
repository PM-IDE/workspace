package services

import (
	"balancer/contextvalues"
	"balancer/executor"
	"balancer/utils"
	"context"
	"grpcmodels"

	"github.com/google/uuid"
	"go.uber.org/zap"
	"google.golang.org/grpc"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
	"google.golang.org/protobuf/types/known/emptypb"
)

type ContextValuesServiceServer struct {
	storage  contextvalues.Storage
	executor executor.PipelineExecutor
	logger   *zap.SugaredLogger
	grpcmodels.UnsafeGrpcContextValuesServiceServer
}

func NewContextValuesServiceServer(storage contextvalues.Storage, logger *zap.SugaredLogger) *ContextValuesServiceServer {
	return &ContextValuesServiceServer{storage: storage, logger: logger}
}

func (this *ContextValuesServiceServer) SetContextValue(
	stream grpc.ClientStreamingServer[grpcmodels.GrpcContextValuePart, grpcmodels.GrpcGuid],
) error {
	cvId, err := uuid.NewV7()
	if err != nil {
		return status.Errorf(codes.Internal, "failed to generate uuid")
	}

	cvRes := utils.UnmarshallContextValue(stream)
	if cvRes.IsErr() {
		return status.Errorf(codes.Internal, cvRes.Err().Error())
	}

	this.storage.AddContextValue(cvId, cvRes.Ok().Key, cvRes.Ok().Value)

	this.logger.Infow("Added context value with id", "context_value_id", cvId)

	return stream.SendAndClose(&grpcmodels.GrpcGuid{Guid: cvId.String()})
}

func (this *ContextValuesServiceServer) DropContextValues(
	context context.Context,
	request *grpcmodels.GrpcDropContextValuesRequest,
) (*emptypb.Empty, error) {
	for _, id := range request.Ids {
		parsedId, err := uuid.Parse(id.GetGuid())
		if err != nil {
			return nil, status.Errorf(codes.InvalidArgument, "provided id is invalid %s", id.GetGuid())
		}

		this.logger.Infow("Removed context value with id", "context_value_id", parsedId)
		this.storage.Remove(parsedId)
	}

	return &emptypb.Empty{}, nil
}

func (this *ContextValuesServiceServer) GetContextValue(
	id *grpcmodels.GrpcGuid,
	stream grpc.ServerStreamingServer[grpcmodels.GrpcContextValuePart],
) error {
	parsedId, err := uuid.Parse(id.GetGuid())
	if err != nil {
		return status.Errorf(codes.InvalidArgument, "provided id is invalid %s", id.GetGuid())
	}

	cv, ok := this.storage.GetContextValue(parsedId)
	if !ok {
		return status.Errorf(codes.NotFound, "failed to find context value for %s", id.GetGuid())
	}

	res := utils.MarshallContextValue(utils.ContextValueWithKey{Key: cv.Key.Name, Value: cv.Value}, stream)
	if res.IsErr() {
		return status.Errorf(codes.Internal, res.Err().Error())
	}

	this.logger.Infow("Returned context value with id", "context_value_id", parsedId)

	return nil
}

func (this *ContextValuesServiceServer) GetAllContextValuesIds(
	_ context.Context,
	id *grpcmodels.GrpcGuid,
) (*grpcmodels.GrpcGetAllContextValuesResult, error) {
	executionContextValues := this.executor.GetExecutionContextValues(id)
	if executionContextValues.IsErr() {
		return nil, executionContextValues.Err()
	}

	var ids []*grpcmodels.GrpcGuid

	for _, id := range *executionContextValues.Ok() {
		ids = append(ids, &grpcmodels.GrpcGuid{Guid: id.String()})
	}

	return &grpcmodels.GrpcGetAllContextValuesResult{ContextValues: ids}, nil
}

func (this *ContextValuesServiceServer) GetContextValueId(
	_ context.Context,
	request *grpcmodels.GrpcGetContextValueRequest,
) (*grpcmodels.GrpcGuid, error) {
	executionContextValues := this.executor.GetExecutionContextValues(request.ExecutionId)
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
