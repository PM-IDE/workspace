package contextvalues

import (
	"balancer/grpcmodels"

	"github.com/google/uuid"
	cmap "github.com/orcaman/concurrent-map/v2"
)

type Storage struct {
	storage cmap.ConcurrentMap[uuid.UUID, grpcmodels.GrpcContextKeyValue]
}

func NewContextValuesStorage() *Storage {
	return &Storage{cmap.NewStringer[uuid.UUID, grpcmodels.GrpcContextKeyValue]()}
}

func (this *Storage) AddContextValue(id uuid.UUID, key string, value *grpcmodels.GrpcContextValue) {
	this.storage.Set(id, grpcmodels.GrpcContextKeyValue{
		Key:   &grpcmodels.GrpcContextKey{Name: key},
		Value: value,
	})
}

func (this *Storage) GetContextValue(id uuid.UUID) (grpcmodels.GrpcContextKeyValue, bool) {
	return this.storage.Get(id)
}

func (this *Storage) Clear() {
	this.storage.Clear()
}

func (this *Storage) Remove(id uuid.UUID) {
	this.storage.Remove(id)
}
