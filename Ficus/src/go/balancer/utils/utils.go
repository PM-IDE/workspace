package utils

import (
	"balancer/grpcmodels"
	"balancer/result"
	"balancer/void"
	"io"
	"slices"

	"github.com/google/uuid"
	"go.uber.org/zap"
	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials/insecure"
	"google.golang.org/protobuf/proto"
)

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

type ContextValueWithKey struct {
	Key   string
	Value *grpcmodels.GrpcContextValue
}

type InputStream[T any] interface {
	Recv() (*T, error)
}

func UnmarshallContextValue(stream InputStream[grpcmodels.GrpcContextValuePart]) result.Result[ContextValueWithKey] {
	var buffer []byte
	var key string

	for {
		msg, err := stream.Recv()
		if err == io.EOF {
			break
		}

		if err != nil {
			return result.Err[ContextValueWithKey](err)
		}

		key = msg.Key
		buffer = append(buffer, msg.Bytes...)
	}

	cv := &grpcmodels.GrpcContextValue{}
	err := proto.Unmarshal(buffer, cv)
	if err != nil {
		return result.Err[ContextValueWithKey](err)
	}

	return result.Ok(&ContextValueWithKey{key, cv})
}

type OutputStream[T any] interface {
	Send(*T) error
}

func MarshallContextValue(
	cv ContextValueWithKey,
	stream OutputStream[grpcmodels.GrpcContextValuePart],
) result.Result[void.Void] {
	cvBytes, err := proto.Marshal(cv.Value)
	if err != nil {
		return result.Err[void.Void](err)
	}

	for chunk := range slices.Chunk(cvBytes, 1024) {
		err = stream.Send(&grpcmodels.GrpcContextValuePart{
			Key:   cv.Key,
			Bytes: chunk,
		})

		if err != nil {
			return result.Err[void.Void](err)
		}
	}

	return result.Ok(void.Instance)
}

func CreateLoggerAttachedToActivity(originalLogger *zap.SugaredLogger) result.Result[zap.SugaredLogger] {
	id, err := uuid.NewV7()
	if err != nil {
		return result.Err[zap.SugaredLogger](err)
	}

	return result.Ok(originalLogger.With("activity_id", id))
}
