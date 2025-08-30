package backends

import (
	"balancer/result"
	"fmt"
	"grpcmodels"
	"sync"

	cmap "github.com/orcaman/concurrent-map/v2"
	"golang.org/x/net/context"
	"google.golang.org/protobuf/types/known/emptypb"
)

type PipelinePartDescriptor struct {
	name string
}

func NewPipelinePartDescriptor(name string) PipelinePartDescriptor {
	return PipelinePartDescriptor{name}
}

type BackendDescriptor struct {
	name          string
	pipelineParts []PipelinePartDescriptor
}

func NewBackendDescriptor(name string, pipelineParts []PipelinePartDescriptor) BackendDescriptor {
	return BackendDescriptor{name, pipelineParts}
}

func materializeBackendsPipelinePartsDescriptors(urls []string) map[string]*BackendDescriptor {
	descriptors := cmap.New[*BackendDescriptor]()

	wg := sync.WaitGroup{}

	for _, url := range urls {
		wg.Go(func() {
			res := materializeBackendPipelinePartsDescriptors(url)
			if res.IsErr() {
				fmt.Printf("Failed to get descriptor for backend %s %s\n", url, res.Err().Error())
				return
			}

			descriptors.Set(url, res.Ok())
		})
	}

	wg.Wait()

	return descriptors.Items()
}

func materializeBackendPipelinePartsDescriptors(url string) result.Result[BackendDescriptor] {
	return grpcmodels.ExecuteWithBackendClient[BackendDescriptor](
		url,
		func(client grpcmodels.GrpcBackendServiceClient) result.Result[BackendDescriptor] {
			info, err := client.GetBackendInfo(context.Background(), &emptypb.Empty{})
			if err != nil {
				return result.Err[BackendDescriptor](err)
			}

			pipelineParts := make([]PipelinePartDescriptor, 0, len(info.GetPipelineParts()))

			for _, descriptor := range info.GetPipelineParts() {
				pipelineParts = append(pipelineParts, NewPipelinePartDescriptor(descriptor.GetName()))
			}

			desc := NewBackendDescriptor(info.GetName(), pipelineParts)
			return result.Ok[BackendDescriptor](&desc)
		},
	)
}
