package utils

import (
	"balancer/grpcmodels"
	"balancer/result"

	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials/insecure"
)

func ExecuteWithBackendClient[T any](
	url string,
	action func(grpcmodels.GrpcBackendServiceClient) result.Result[T],
) result.Result[T] {
	clientRes := CreateNewBackendServiceClient(url)
	if clientRes.IsErr() {
		return result.Err[T](clientRes.Err())
	}

	valueResult := action(clientRes.Ok().client)

	if err := clientRes.Ok().connection.Close(); err != nil {
		return result.Err[T](err)
	}

	return valueResult
}

type clientWithConnection[TClient any] struct {
	client     TClient
	connection *grpc.ClientConn
}

func CreateNewBackendServiceClient(url string) result.Result[clientWithConnection[grpcmodels.GrpcBackendServiceClient]] {
	conn, err := grpc.NewClient(url, grpc.WithTransportCredentials(insecure.NewCredentials()))
	if err != nil {
		return result.Err[clientWithConnection[grpcmodels.GrpcBackendServiceClient]](err)
	}

	client := grpcmodels.NewGrpcBackendServiceClient(conn)
	return result.Ok(&clientWithConnection[grpcmodels.GrpcBackendServiceClient]{client, conn})
}
