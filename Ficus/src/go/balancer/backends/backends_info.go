package backends

import (
	"balancer/result"
	"balancer/void"
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
	this.pipelinePartsToBackendUrls = make(map[string][]string)

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

	return result.Ok(void.Instance)
}
