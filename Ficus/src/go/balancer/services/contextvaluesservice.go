package services

import (
	"balancer/contextvalues"
	"balancer/grpcmodels"
	"balancer/utils"
	"context"

	"github.com/google/uuid"
	"google.golang.org/grpc"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
	"google.golang.org/protobuf/types/known/emptypb"
)

type ContextValuesServiceServer struct {
	storage *contextvalues.Storage
	grpcmodels.UnsafeGrpcContextValuesServiceServer
}

func NewContextValuesServiceServer(storage *contextvalues.Storage) *ContextValuesServiceServer {
	return &ContextValuesServiceServer{storage: storage}
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

	return nil
}

func (this *ContextValuesServiceServer) DropContextValues(
	context context.Context,
	request *grpcmodels.GrpcDropContextValuesRequest,
) (*emptypb.Empty, error) {
	return nil, status.Errorf(codes.Unimplemented, "")
}

func (this *ContextValuesServiceServer) GetContextValue(
	id *grpcmodels.GrpcGuid,
	stream grpc.ServerStreamingServer[grpcmodels.GrpcContextValuePart],
) error {
	return status.Errorf(codes.Unimplemented, "method GetContextValue not implemented")
}
