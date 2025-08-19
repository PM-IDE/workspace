package backends

import (
	grpc_models "balancer/models"
	"balancer/result"
	"fmt"
	"sync"

	cmap "github.com/orcaman/concurrent-map/v2"
	"golang.org/x/net/context"
	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials/insecure"
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
	conn, err := grpc.NewClient(url, grpc.WithTransportCredentials(insecure.NewCredentials()))
	if err != nil {
		return result.Err[BackendDescriptor](err)
	}

	client := grpc_models.NewGrpcBackendServiceClient(conn)
	info, err := client.GetBackendInfo(context.Background(), &emptypb.Empty{})
	if err != nil {
		return result.Err[BackendDescriptor](err)
	}

	pipelineParts := make([]PipelinePartDescriptor, 0, len(info.GetPipelineParts()))

	for _, descriptor := range info.GetPipelineParts() {
		pipelineParts = append(pipelineParts, NewPipelinePartDescriptor(descriptor.GetName()))
	}

	desc := NewBackendDescriptor(info.GetName(), pipelineParts)

	if err = conn.Close(); err != nil {
		return result.Err[BackendDescriptor](err)
	}

	return result.Ok[BackendDescriptor](&desc)
}
