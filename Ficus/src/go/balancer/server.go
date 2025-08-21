package main

import (
	"balancer/backends"
	"balancer/grpcmodels"
	"balancer/result"
	"balancer/void"
	"context"
	"fmt"
	"net"

	"google.golang.org/grpc"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
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
