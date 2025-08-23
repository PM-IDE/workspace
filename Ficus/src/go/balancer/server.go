package main

import (
	"balancer/grpcmodels"
	"balancer/result"
	"balancer/void"
	"fmt"
	"net"

	"google.golang.org/grpc"
)

func StartServer() result.Result[void.Void] {
	lis, err := net.Listen("tcp", ":8081")

	if err != nil {
		fmt.Printf("failed to listen: %v\n", err)
		return result.Err[void.Void](err)
	}

	var opts []grpc.ServerOption
	grpcServer := grpc.NewServer(opts...)

	container := BuildContainer()
	defer func() {
		err := container.Logger.Sync()
		if err != nil {
			fmt.Println(err.Error())
		}
	}()

	grpcmodels.RegisterGrpcBackendServiceServer(grpcServer, container.BackendService)
	grpcmodels.RegisterGrpcContextValuesServiceServer(grpcServer, container.ContextValuesService)

	return result.FromErr(grpcServer.Serve(lis))
}
