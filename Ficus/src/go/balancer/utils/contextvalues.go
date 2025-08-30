package utils

import (
	"balancer/result"
	"balancer/void"
	"grpcmodels"
	"io"
	"slices"

	"google.golang.org/protobuf/proto"
)

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
