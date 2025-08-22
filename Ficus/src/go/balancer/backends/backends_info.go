package backends

import (
	"balancer/result"
	"balancer/void"
	"fmt"
)

type BackendsInfo struct {
	pipelinePartsToBackendUrls map[string][]string
}

func NewBackendsInfo() *BackendsInfo {
	return &BackendsInfo{make(map[string][]string)}
}

func (this *BackendsInfo) GetPipelinePartsToBackendUrls() map[string][]string {
	return this.pipelinePartsToBackendUrls
}

func (this *BackendsInfo) UpdateBackendsInfo(urls []string) result.Result[void.Void] {
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

	this.pipelinePartsToBackendUrls = pipelinePartsToBackendUrls

	return result.Ok(void.Instance)
}

func (this *BackendsInfo) GetBackends(partName string) result.Result[[]string] {
	if backends, ok := this.pipelinePartsToBackendUrls[partName]; ok {
		return result.Ok(&backends)
	}

	return result.Err[[]string](fmt.Errorf("the are no backends for pipeline part %s", partName))
}
