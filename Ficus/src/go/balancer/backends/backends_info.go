package backends

import (
	"balancer/result"
	"balancer/void"
	"fmt"
	"sync"
	"sync/atomic"
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
}

func NewBackendsInfo() BackendsInfo {
	return &backendsInfo{&sync.Mutex{}, &atomic.Bool{}, make(map[string][]string)}
}

func NewBackendsInfoWithPredefinedParts(pipelinePartsToBackendUrls map[string][]string) BackendsInfo {
	predefinedMapSet := &atomic.Bool{}
	predefinedMapSet.Store(true)

	return &backendsInfo{&sync.Mutex{}, predefinedMapSet, pipelinePartsToBackendUrls}
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

	descriptors := materializeBackendsPipelinePartsDescriptors(urls)
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
