package main

import (
	grpc_models "balancer/models"
	"context"
	"fmt"
	"os"
	"strings"
	"sync"

	cmap "github.com/orcaman/concurrent-map/v2"
	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials/insecure"
	"google.golang.org/protobuf/types/known/emptypb"
)

func main() {
	backendsUrls, err := getBackends()
	if err != nil {
		fmt.Println(err.Error())
		os.Exit(-1)
		return
	}

	fmt.Println(materializeBackendsPipelinePartsDescriptors(backendsUrls))
}

const BackendsEnvVar = "BALANCER_BACKENDS"

func getBackends() ([]string, error) {
	if rawBackends, ok := os.LookupEnv(BackendsEnvVar); ok {
		backends := strings.Split(rawBackends, ";")
		return backends, nil
	} else {
		return nil, fmt.Errorf("the %s environment variable is not set", BackendsEnvVar)
	}
}

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

func materializeBackendsPipelinePartsDescriptors(urls []string) map[string]BackendDescriptor {
	descriptors := cmap.New[BackendDescriptor]()

	wg := sync.WaitGroup{}

	for _, url := range urls {
		wg.Go(func() {
			desc, err := materializeBackendPipelinePartsDescriptors(url)
			if err != nil {
				fmt.Printf("Failed to get descriptor for backend %s %s\n", url, err.Error())
				return
			}

			descriptors.Set(url, desc)
		})
	}

	wg.Wait()

	return descriptors.Items()
}

func materializeBackendPipelinePartsDescriptors(url string) (BackendDescriptor, error) {
	conn, err := grpc.NewClient(url, grpc.WithTransportCredentials(insecure.NewCredentials()))
	if err != nil {
		return BackendDescriptor{}, err
	}

	client := grpc_models.NewGrpcBackendServiceClient(conn)
	info, err := client.GetBackendInfo(context.Background(), &emptypb.Empty{})
	if err != nil {
		return BackendDescriptor{}, err
	}

	pipelineParts := make([]PipelinePartDescriptor, 0, len(info.GetPipelineParts()))

	for _, descriptor := range info.GetPipelineParts() {
		pipelineParts = append(pipelineParts, NewPipelinePartDescriptor(descriptor.GetName()))
	}

	desc := NewBackendDescriptor(info.GetName(), pipelineParts)

	if err = conn.Close(); err != nil {
		return BackendDescriptor{}, err
	}

	return desc, nil
}
