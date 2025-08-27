package integrationtests

import (
	"balancer/result"
	"balancer/utils"
	"balancer/void"
	"context"
	"grpcmodels"
	"os"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestSetContextValues(t *testing.T) {
	backend, ok := os.LookupEnv("BALANCER_BACKEND")
	if !ok {
		assert.Fail(t, "balancer backend is not specified")
		return
	}

	res := utils.ExecuteWithContextValuesClient(
		backend,
		func(client grpcmodels.GrpcContextValuesServiceClient) result.Result[void.Void] {
			outputStream, err := client.SetContextValue(context.Background())
			assert.Nil(t, err)

			rawStringValue := "asdasdasd"
			contextValue := &grpcmodels.GrpcContextValue{
				ContextValue: &grpcmodels.GrpcContextValue_String_{
					String_: rawStringValue,
				},
			}

			rawKey := "key"
			res := utils.MarshallContextValue(utils.ContextValueWithKey{Key: rawKey, Value: contextValue}, outputStream)
			assert.True(t, res.IsOk())

			cvId, err := outputStream.CloseAndRecv()
			assert.Nil(t, err)

			inputStream, err := client.GetContextValue(context.Background(), cvId)
			assert.Nil(t, err)

			cvRes := utils.UnmarshallContextValue(inputStream)
			assert.True(t, cvRes.IsOk())

			assert.Equal(t, cvRes.Ok().Key, rawKey)
			assert.Equal(t, cvRes.Ok().Value.GetString_(), rawStringValue)

			_, err = client.DropContextValues(context.Background(), &grpcmodels.GrpcDropContextValuesRequest{
				Ids: []*grpcmodels.GrpcGuid{
					cvId,
				},
			})

			assert.Nil(t, err)

			inputStream, err = client.GetContextValue(context.Background(), cvId)
			assert.Nil(t, err)

			cvRes = utils.UnmarshallContextValue(inputStream)
			assert.True(t, cvRes.IsErr())

			return result.Ok(void.Instance)
		},
	)

	assert.True(t, res.IsOk())
}
