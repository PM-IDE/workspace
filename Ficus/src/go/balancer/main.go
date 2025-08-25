package main

import (
	"fmt"
	"grpcmodels"
	"net"

	"google.golang.org/grpc"
)

func main() {
	lis, err := net.Listen("tcp", ":8080")

	if err != nil {
		fmt.Printf("failed to listen: %v\n", err)
		return
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

	err = grpcServer.Serve(lis)
	if err != nil {
		fmt.Printf("error  %v\n", err)
	}
}
