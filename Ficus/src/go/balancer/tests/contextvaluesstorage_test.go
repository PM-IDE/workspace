package tests

import (
	"balancer/contextvalues"
	"balancer/grpcmodels"
	"testing"

	"github.com/google/uuid"
	"github.com/stretchr/testify/assert"
)

func TestStorageAdd(t *testing.T) {
	storage := contextvalues.NewContextValuesStorage()

	id, err := uuid.NewV7()
	assert.Nil(t, err)

	key := "xd"
	value := &grpcmodels.GrpcContextValue{
		ContextValue: nil,
	}

	storage.AddContextValue(id, key, value)

	retrievedValue, ok := storage.GetContextValue(id)

	assert.True(t, ok)
	assert.Equal(t, retrievedValue.Value, value)
	assert.Equal(t, retrievedValue.Key.Name, key)

	storage.Remove(id)

	retrievedValue, ok = storage.GetContextValue(id)
	assert.False(t, ok)
}

func TestStorageClear(t *testing.T) {
	storage := contextvalues.NewContextValuesStorage()

	id, err := uuid.NewV7()
	assert.Nil(t, err)

	key := "xd"
	value := &grpcmodels.GrpcContextValue{
		ContextValue: nil,
	}

	storage.AddContextValue(id, key, value)

	retrievedValue, ok := storage.GetContextValue(id)
	assert.True(t, ok)
	assert.Equal(t, retrievedValue.Value, value)
	assert.Equal(t, retrievedValue.Key.Name, key)

	storage.Clear()

	retrievedValue, ok = storage.GetContextValue(id)
	assert.False(t, ok)
}
