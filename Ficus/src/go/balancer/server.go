package main

import (
	grpcmodels "balancer/models"
	"balancer/result"
	"balancer/void"
	"context"
	"fmt"
	"log"
	"net"

	"google.golang.org/grpc"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
	"google.golang.org/protobuf/types/known/emptypb"
)

func StartServer() result.Result[void.Void] {
	lis, err := net.Listen("tcp", fmt.Sprintf("localhost:%d", 8080))

	if err != nil {
		log.Fatalf("failed to listen: %v", err)
	}

	var opts []grpc.ServerOption
	grpcServer := grpc.NewServer(opts...)
	grpcmodels.RegisterGrpcBackendServiceServer(grpcServer, &backendServiceServer{})
	res := result.FromErr(grpcServer.Serve(lis))

	if res.IsErr() {
		return result.Err[void.Void](res.Err())
	}

	return result.Ok(void.Instance)
}

type backendServiceServer struct {
	grpcmodels.UnsafeGrpcBackendServiceServer
}

func (this backendServiceServer) ExecutePipeline(
	request *grpcmodels.GrpcProxyPipelineExecutionRequest,
	server grpc.ServerStreamingServer[grpcmodels.GrpcPipelinePartExecutionResult],
) error {
	return status.Errorf(codes.Unimplemented, "method ExecutePipeline not implemented")
}

func (this backendServiceServer) GetContextValue(
	context context.Context,
	request *grpcmodels.GrpcGetContextValueRequest,
) (*grpcmodels.GrpcGetContextValueResult, error) {
	return nil, status.Errorf(codes.Unimplemented, "method GetContextValue not implemented")
}

func (this backendServiceServer) DropExecutionResult(context context.Context, id *grpcmodels.GrpcGuid) (*emptypb.Empty, error) {
	return nil, status.Errorf(codes.Unimplemented, "method DropExecutionResult not implemented")
}

func (this backendServiceServer) GetBackendInfo(context context.Context, empty *emptypb.Empty) (*grpcmodels.GrpcFicusBackendInfo, error) {
	return nil, status.Errorf(codes.Unimplemented, "method GetBackendInfo not implemented")
}
