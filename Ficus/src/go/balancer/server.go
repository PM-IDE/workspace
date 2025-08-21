package main

import (
	"balancer/grpcmodels"
	"balancer/result"
	"balancer/void"
	"fmt"
	"net"

	"google.golang.org/grpc"
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
