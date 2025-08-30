package utils

import (
	"balancer/result"
	"grpcmodels"

	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials/insecure"
)

func ExecuteWithBalancerClient[T any](
	url string,
	action func(client grpcmodels.GrpcBackendBalancerServiceClient) result.Result[T],
) result.Result[T] {
	return ExecuteWithGrpcClient[grpcmodels.GrpcBackendBalancerServiceClient, T](
		url,
		func(conn *grpc.ClientConn) grpcmodels.GrpcBackendBalancerServiceClient {
			return grpcmodels.NewGrpcBackendBalancerServiceClient(conn)
		},
		action,
	)
}

func ExecuteWithBackendClient[T any](
	url string,
	action func(grpcmodels.GrpcBackendServiceClient) result.Result[T],
) result.Result[T] {
	return ExecuteWithGrpcClient[grpcmodels.GrpcBackendServiceClient, T](
		url,
		func(conn *grpc.ClientConn) grpcmodels.GrpcBackendServiceClient {
			return grpcmodels.NewGrpcBackendServiceClient(conn)
		},
		action,
	)
}

func ExecuteWithContextValuesClient[T any](
	url string,
	action func(client grpcmodels.GrpcContextValuesServiceClient) result.Result[T],
) result.Result[T] {
	return ExecuteWithGrpcClient[grpcmodels.GrpcContextValuesServiceClient, T](
		url,
		func(conn *grpc.ClientConn) grpcmodels.GrpcContextValuesServiceClient {
			return grpcmodels.NewGrpcContextValuesServiceClient(conn)
		},
		action,
	)
}

func ExecuteWithGrpcClient[TClient any, TReturn any](
	url string,
	factory func(*grpc.ClientConn) TClient,
	action func(TClient) result.Result[TReturn],
) result.Result[TReturn] {
	clientRes := CreateNewGrpcServiceClient(url, factory)
	if clientRes.IsErr() {
		return result.Err[TReturn](clientRes.Err())
	}

	valueResult := action(clientRes.Ok().client)

	if err := clientRes.Ok().connection.Close(); err != nil {
		return result.Err[TReturn](err)
	}

	return valueResult
}

type clientWithConnection[TClient any] struct {
	client     TClient
	connection *grpc.ClientConn
}

func CreateNewGrpcServiceClient[TClient any](
	url string,
	factory func(*grpc.ClientConn) TClient,
) result.Result[clientWithConnection[TClient]] {
	conn, err := grpc.NewClient(url, grpc.WithTransportCredentials(insecure.NewCredentials()))
	if err != nil {
		return result.Err[clientWithConnection[TClient]](err)
	}

	client := factory(conn)
	return result.Ok(&clientWithConnection[TClient]{client, conn})
}
