package services

import (
	"balancer/backends"
	"balancer/grpcmodels"
	"context"

	"google.golang.org/grpc"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
	"google.golang.org/protobuf/types/known/emptypb"
)

type BackendServiceServer struct {
	urls         []string
	backendsInfo *backends.BackendsInfo
	grpcmodels.UnsafeGrpcBackendServiceServer
}

func NewBackendServiceServer(urls []string) *BackendServiceServer {
	return &BackendServiceServer{urls: urls, backendsInfo: backends.NewBackendsInfo()}
}

func (this *BackendServiceServer) ExecutePipeline(
	request *grpcmodels.GrpcProxyPipelineExecutionRequest,
	server grpc.ServerStreamingServer[grpcmodels.GrpcPipelinePartExecutionResult],
) error {
	res := this.backendsInfo.UpdateBackendsInfo(this.urls)
	if res.IsErr() {
		return status.Errorf(codes.Internal, "failed to update backends information: %s", res.Err().Error())
	}

	return status.Errorf(codes.Unimplemented, "method ExecutePipeline not implemented")
}

func (this *BackendServiceServer) GetContextValue(
	context context.Context,
	request *grpcmodels.GrpcGetContextValueRequest,
) (*grpcmodels.GrpcGetContextValueResult, error) {
	return nil, status.Errorf(codes.Unimplemented, "method GetContextValue not implemented")
}

func (this *BackendServiceServer) DropExecutionResult(context context.Context, id *grpcmodels.GrpcGuid) (*emptypb.Empty, error) {
	return nil, status.Errorf(codes.Unimplemented, "method DropExecutionResult not implemented")
}

func (this *BackendServiceServer) GetBackendInfo(context context.Context, empty *emptypb.Empty) (*grpcmodels.GrpcFicusBackendInfo, error) {
	return nil, status.Errorf(codes.Unimplemented, "method GetBackendInfo not implemented")
}

func (this *BackendServiceServer) GetAllContextValues(context.Context, *grpcmodels.GrpcGuid) (*grpcmodels.GrpcGetAllContextValuesResult, error) {
	return nil, status.Errorf(codes.Unimplemented, "method GetAllContextValues not implemented")
}
