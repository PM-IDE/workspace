package services

import (
	"balancer/backends"
	"context"
	"grpcmodels"

	"google.golang.org/protobuf/types/known/emptypb"
)

type BalancerServiceServer struct {
	backendsInfo backends.BackendsInfo
	grpcmodels.UnsafeGrpcBackendBalancerServiceServer
}

func NewBalancerServiceServer(backendsInfo backends.BackendsInfo) *BalancerServiceServer {
	return &BalancerServiceServer{backendsInfo: backendsInfo}
}

func (this *BalancerServiceServer) SetPipelinePartsToBackendsMap(
	ctx context.Context,
	backendsMap *grpcmodels.GrpcPredefinedPipelinePartsToBackendsMap,
) (*emptypb.Empty, error) {
	newMap := make(map[string][]string)

	for _, partToBackends := range backendsMap.GetPartsToBackends() {
		newMap[partToBackends.GetPartName()] = partToBackends.GetBackends()
	}

	this.backendsInfo.SetPredefinedMap(newMap)

	return &emptypb.Empty{}, nil
}
