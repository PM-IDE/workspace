package backends

type BackendsInfo struct {
	pipelinePartsToBackendUrls map[string][]string
}

func CreateBackendsInfo(urls []string) *BackendsInfo {
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

	return &BackendsInfo{pipelinePartsToBackendUrls}
}

func (this *BackendsInfo) GetPipelinePartsToBackendUrls() map[string][]string {
	return this.pipelinePartsToBackendUrls
}
