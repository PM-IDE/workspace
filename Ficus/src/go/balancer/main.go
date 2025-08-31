package main

import (
	"fmt"
	"grpcmodels"
	"net"

	"google.golang.org/grpc"
)

func main() {
	container := BuildContainer()
	defer func() {
		err := container.Logger.Sync()
		if err != nil {
			fmt.Println(err.Error())
		}
	}()

	lis, err := net.Listen("tcp", ":8080")

	if err != nil {
		container.Logger.Errorw("Failed to listen", "error", err)
		return
	}

	var opts []grpc.ServerOption
	grpcServer := grpc.NewServer(opts...)

	grpcmodels.RegisterGrpcBackendServiceServer(grpcServer, container.BackendService)
	grpcmodels.RegisterGrpcContextValuesServiceServer(grpcServer, container.ContextValuesService)
	grpcmodels.RegisterGrpcBackendBalancerServiceServer(grpcServer, container.BalancerService)

	err = grpcServer.Serve(lis)
	if err != nil {
		container.Logger.Errorw("Error happened with grpc server serving", "error", err)
	}
}
