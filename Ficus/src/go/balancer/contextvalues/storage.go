package contextvalues

import (
	"grpcmodels"

	"github.com/google/uuid"
	cmap "github.com/orcaman/concurrent-map/v2"
)

type Storage interface {
	AddContextValue(id uuid.UUID, key string, value *grpcmodels.GrpcContextValue)
	GetContextValue(id uuid.UUID) (grpcmodels.GrpcContextKeyValue, bool)
	Clear()
	Remove(id uuid.UUID)
}

type storage struct {
	storage cmap.ConcurrentMap[uuid.UUID, grpcmodels.GrpcContextKeyValue]
}

func NewContextValuesStorage() Storage {
	return &storage{cmap.NewStringer[uuid.UUID, grpcmodels.GrpcContextKeyValue]()}
}

func (this *storage) AddContextValue(id uuid.UUID, key string, value *grpcmodels.GrpcContextValue) {
	this.storage.Set(id, grpcmodels.GrpcContextKeyValue{
		Key:   &grpcmodels.GrpcContextKey{Name: key},
		Value: value,
	})
}

func (this *storage) GetContextValue(id uuid.UUID) (grpcmodels.GrpcContextKeyValue, bool) {
	return this.storage.Get(id)
}

func (this *storage) Clear() {
	this.storage.Clear()
}

func (this *storage) Remove(id uuid.UUID) {
	this.storage.Remove(id)
}
