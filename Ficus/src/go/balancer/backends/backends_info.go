package backends

import (
	"balancer/result"
	"balancer/void"
	"fmt"
	"sync"
	"sync/atomic"

	"go.uber.org/zap"
)

type BackendsInfo interface {
	UpdateBackendsInfo(urls []string) result.Result[void.Void]
	GetBackends(partName string) result.Result[[]string]
	GetPipelinePartsToBackendUrls() map[string][]string

	SetPredefinedMap(map[string][]string)
}

type backendsInfo struct {
	lock                       *sync.Mutex
	predefinedMapSet           *atomic.Bool
	pipelinePartsToBackendUrls map[string][]string
	logger                     *zap.SugaredLogger
}

func NewBackendsInfo(logger *zap.SugaredLogger) BackendsInfo {
	return &backendsInfo{
		&sync.Mutex{},
		&atomic.Bool{},
		make(map[string][]string),
		logger,
	}
}

func NewBackendsInfoWithPredefinedParts(pipelinePartsToBackendUrls map[string][]string, logger *zap.SugaredLogger) BackendsInfo {
	predefinedMapSet := &atomic.Bool{}
	predefinedMapSet.Store(true)

	return &backendsInfo{
		&sync.Mutex{},
		predefinedMapSet,
		pipelinePartsToBackendUrls,
		logger,
	}
}

func (this *backendsInfo) GetPipelinePartsToBackendUrls() map[string][]string {
	return this.pipelinePartsToBackendUrls
}

func (this *backendsInfo) SetPredefinedMap(predefinedMap map[string][]string) {
	this.lock.Lock()
	defer this.lock.Unlock()

	this.pipelinePartsToBackendUrls = predefinedMap
	this.predefinedMapSet.Store(true)
}

func (this *backendsInfo) UpdateBackendsInfo(urls []string) result.Result[void.Void] {
	if this.predefinedMapSet.Load() {
		return result.Ok(void.Instance)
	}

	descriptors := materializeBackendsPipelinePartsDescriptors(urls, this.logger)
	pipelinePartsToBackendUrls := make(map[string][]string)
	for backendUrl, descriptor := range descriptors {
		for _, part := range descriptor.pipelineParts {
			_, ok := pipelinePartsToBackendUrls[part.name]
			if !ok {
				pipelinePartsToBackendUrls[part.name] = []string{}
			}

			backends := pipelinePartsToBackendUrls[part.name]
			pipelinePartsToBackendUrls[part.name] = append(backends, backendUrl)
		}
	}

	this.lock.Lock()

	this.pipelinePartsToBackendUrls = pipelinePartsToBackendUrls

	this.lock.Unlock()

	return result.Ok(void.Instance)
}

func (this *backendsInfo) GetBackends(partName string) result.Result[[]string] {
	this.lock.Lock()
	defer this.lock.Unlock()

	if backends, ok := this.pipelinePartsToBackendUrls[partName]; ok {
		return result.Ok(&backends)
	}

	return result.Err[[]string](fmt.Errorf("the are no backends for pipeline part %s", partName))
}
