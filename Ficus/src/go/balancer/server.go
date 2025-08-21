package main

import (
	"balancer/backends"
	"balancer/grpcmodels"
	"balancer/result"
	"balancer/void"
	"context"
	"fmt"
	"io"
	"net"

	"github.com/google/uuid"
	cmap "github.com/orcaman/concurrent-map/v2"
	"google.golang.org/grpc"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
	"google.golang.org/protobuf/proto"
	"google.golang.org/protobuf/types/known/emptypb"
)

func StartServer(urls []string) result.Result[void.Void] {
	lis, err := net.Listen("tcp", fmt.Sprintf("localhost:%d", 8080))

	if err != nil {
		fmt.Printf("failed to listen: %v\n", err)
		return result.Err[void.Void](err)
	}

	var opts []grpc.ServerOption
	grpcServer := grpc.NewServer(opts...)

	grpcmodels.RegisterGrpcBackendServiceServer(grpcServer, newBackendService(urls))
	grpcmodels.RegisterGrpcContextValuesServiceServer(grpcServer, newContextValuesService())

	return result.FromErr(grpcServer.Serve(lis))
}

type backendServiceServer struct {
	urls         []string
	backendsInfo *backends.BackendsInfo
	grpcmodels.UnsafeGrpcBackendServiceServer
}

func newBackendService(urls []string) *backendServiceServer {
	return &backendServiceServer{urls: urls, backendsInfo: backends.NewBackendsInfo()}
}

func (this *backendServiceServer) ExecutePipeline(
	request *grpcmodels.GrpcProxyPipelineExecutionRequest,
	server grpc.ServerStreamingServer[grpcmodels.GrpcPipelinePartExecutionResult],
) error {
	res := this.backendsInfo.UpdateBackendsInfo(this.urls)
	if res.IsErr() {
		return status.Errorf(codes.Internal, "failed to update backends information: %s", res.Err().Error())
	}

	return status.Errorf(codes.Unimplemented, "method ExecutePipeline not implemented")
}

func (this *backendServiceServer) GetContextValue(
	context context.Context,
	request *grpcmodels.GrpcGetContextValueRequest,
) (*grpcmodels.GrpcGetContextValueResult, error) {
	return nil, status.Errorf(codes.Unimplemented, "method GetContextValue not implemented")
}

func (this *backendServiceServer) DropExecutionResult(context context.Context, id *grpcmodels.GrpcGuid) (*emptypb.Empty, error) {
	return nil, status.Errorf(codes.Unimplemented, "method DropExecutionResult not implemented")
}

func (this *backendServiceServer) GetBackendInfo(context context.Context, empty *emptypb.Empty) (*grpcmodels.GrpcFicusBackendInfo, error) {
	return nil, status.Errorf(codes.Unimplemented, "method GetBackendInfo not implemented")
}

func (this *backendServiceServer) GetAllContextValues(context.Context, *grpcmodels.GrpcGuid) (*grpcmodels.GrpcGetAllContextValuesResult, error) {
	return nil, status.Errorf(codes.Unimplemented, "method GetAllContextValues not implemented")
}

type ContextValuesStorage struct {
	storage cmap.ConcurrentMap[uuid.UUID, grpcmodels.GrpcContextKeyValue]
}

func NewContextValuesStorage() *ContextValuesStorage {
	return &ContextValuesStorage{cmap.NewStringer[uuid.UUID, grpcmodels.GrpcContextKeyValue]()}
}

func (this *ContextValuesStorage) AddContextValue(id uuid.UUID, key string, value *grpcmodels.GrpcContextValue) {
	this.storage.Set(id, grpcmodels.GrpcContextKeyValue{
		Key:   &grpcmodels.GrpcContextKey{Name: key},
		Value: value,
	})
}

type contextValuesService struct {
	storage *ContextValuesStorage
	grpcmodels.UnsafeGrpcContextValuesServiceServer
}

func newContextValuesService() *contextValuesService {
	return &contextValuesService{}
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
