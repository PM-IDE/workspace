package services

import (
	"balancer/contextvalues"
	"balancer/grpcmodels"
	"balancer/utils"
	"context"

	"github.com/google/uuid"
	"go.uber.org/zap"
	"google.golang.org/grpc"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
	"google.golang.org/protobuf/types/known/emptypb"
)

type ContextValuesServiceServer struct {
	storage contextvalues.Storage
	logger  *zap.SugaredLogger
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
