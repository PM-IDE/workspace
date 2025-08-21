package main

import (
	"balancer/contextvalues"
	"balancer/grpcmodels"
	"context"
	"io"

	"github.com/google/uuid"
	"google.golang.org/grpc"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
	"google.golang.org/protobuf/proto"
	"google.golang.org/protobuf/types/known/emptypb"
)

type contextValuesService struct {
	storage *contextvalues.Storage
	grpcmodels.UnsafeGrpcContextValuesServiceServer
}

func newContextValuesService() *contextValuesService {
	return &contextValuesService{storage: contextvalues.NewContextValuesStorage()}
}

func (this *contextValuesService) SetContextValue(
	stream grpc.ClientStreamingServer[grpcmodels.GrpcContextValuePart, grpcmodels.GrpcGuid],
) error {
	cvId, err := uuid.NewV7()
	if err != nil {
		return status.Errorf(codes.Internal, "failed to generate uuid")
	}

	var buffer []byte
	var key string

	for {
		msg, err := stream.Recv()
		if err == io.EOF {
			break
		}

		if err != nil {
			return status.Errorf(codes.Internal, "failed to read next part of the context value %s", err.Error())
		}

		key = msg.Key
		buffer = append(buffer, msg.Bytes...)
	}

	cv := &grpcmodels.GrpcContextValue{}
	err = proto.Unmarshal(buffer, cv)
	if err != nil {
		return status.Errorf(codes.Internal, "failed to unmarshall proto message %s", err.Error())
	}

	this.storage.AddContextValue(cvId, key, cv)

	return nil
}

func (this *contextValuesService) DropContextValues(
	context context.Context,
	request *grpcmodels.GrpcDropContextValuesRequest,
) (*emptypb.Empty, error) {
	return nil, status.Errorf(codes.Unimplemented, "")
}
