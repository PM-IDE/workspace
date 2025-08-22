package main

import (
	"balancer/contextvalues"
	"balancer/grpcmodels"
	"balancer/result"
	"balancer/services"
	"balancer/void"
	"fmt"
	"net"

	"google.golang.org/grpc"
)

func StartServer(urls []string) result.Result[void.Void] {
	lis, err := net.Listen("tcp", ":8080")

	if err != nil {
		fmt.Printf("failed to listen: %v\n", err)
		return result.Err[void.Void](err)
	}

	var opts []grpc.ServerOption
	grpcServer := grpc.NewServer(opts...)

	storage := contextvalues.NewContextValuesStorage()
	grpcmodels.RegisterGrpcBackendServiceServer(grpcServer, services.NewBackendServiceServer(urls, storage))
	grpcmodels.RegisterGrpcContextValuesServiceServer(grpcServer, services.NewContextValuesServiceServer(storage))

	return result.FromErr(grpcServer.Serve(lis))
}
