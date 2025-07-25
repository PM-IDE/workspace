// Original file: /Users/aero/work/workspace/Ficus/protos/front_contract.proto

import type * as grpc from '@grpc/grpc-js'
import type { MethodDefinition } from '@grpc/proto-loader'
import type { Empty_DONTUSE as _google_protobuf_Empty_DONTUSE, Empty as _google_protobuf_Empty } from '../google/protobuf/Empty';
import type { GrpcCaseContextValues_DONTUSE as _ficus_GrpcCaseContextValues_DONTUSE, GrpcCaseContextValues as _ficus_GrpcCaseContextValues } from '../ficus/GrpcCaseContextValues';
import type { GrpcGetPipelineCaseContextValuesRequest_DONTUSE as _ficus_GrpcGetPipelineCaseContextValuesRequest_DONTUSE, GrpcGetPipelineCaseContextValuesRequest as _ficus_GrpcGetPipelineCaseContextValuesRequest } from '../ficus/GrpcGetPipelineCaseContextValuesRequest';
import type { GrpcSubscriptionAndPipelinesStateResponse_DONTUSE as _ficus_GrpcSubscriptionAndPipelinesStateResponse_DONTUSE, GrpcSubscriptionAndPipelinesStateResponse as _ficus_GrpcSubscriptionAndPipelinesStateResponse } from '../ficus/GrpcSubscriptionAndPipelinesStateResponse';

export interface GrpcPipelinePartsContextValuesServiceClient extends grpc.Client {
  GetPipelineCaseContextValue(argument: _ficus_GrpcGetPipelineCaseContextValuesRequest_DONTUSE, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_ficus_GrpcCaseContextValues>): grpc.ClientUnaryCall;
  GetPipelineCaseContextValue(argument: _ficus_GrpcGetPipelineCaseContextValuesRequest_DONTUSE, metadata: grpc.Metadata, callback: grpc.requestCallback<_ficus_GrpcCaseContextValues>): grpc.ClientUnaryCall;
  GetPipelineCaseContextValue(argument: _ficus_GrpcGetPipelineCaseContextValuesRequest_DONTUSE, options: grpc.CallOptions, callback: grpc.requestCallback<_ficus_GrpcCaseContextValues>): grpc.ClientUnaryCall;
  GetPipelineCaseContextValue(argument: _ficus_GrpcGetPipelineCaseContextValuesRequest_DONTUSE, callback: grpc.requestCallback<_ficus_GrpcCaseContextValues>): grpc.ClientUnaryCall;
  getPipelineCaseContextValue(argument: _ficus_GrpcGetPipelineCaseContextValuesRequest_DONTUSE, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_ficus_GrpcCaseContextValues>): grpc.ClientUnaryCall;
  getPipelineCaseContextValue(argument: _ficus_GrpcGetPipelineCaseContextValuesRequest_DONTUSE, metadata: grpc.Metadata, callback: grpc.requestCallback<_ficus_GrpcCaseContextValues>): grpc.ClientUnaryCall;
  getPipelineCaseContextValue(argument: _ficus_GrpcGetPipelineCaseContextValuesRequest_DONTUSE, options: grpc.CallOptions, callback: grpc.requestCallback<_ficus_GrpcCaseContextValues>): grpc.ClientUnaryCall;
  getPipelineCaseContextValue(argument: _ficus_GrpcGetPipelineCaseContextValuesRequest_DONTUSE, callback: grpc.requestCallback<_ficus_GrpcCaseContextValues>): grpc.ClientUnaryCall;
  
  GetSubscriptionAndPipelinesState(argument: _google_protobuf_Empty_DONTUSE, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_ficus_GrpcSubscriptionAndPipelinesStateResponse>): grpc.ClientUnaryCall;
  GetSubscriptionAndPipelinesState(argument: _google_protobuf_Empty_DONTUSE, metadata: grpc.Metadata, callback: grpc.requestCallback<_ficus_GrpcSubscriptionAndPipelinesStateResponse>): grpc.ClientUnaryCall;
  GetSubscriptionAndPipelinesState(argument: _google_protobuf_Empty_DONTUSE, options: grpc.CallOptions, callback: grpc.requestCallback<_ficus_GrpcSubscriptionAndPipelinesStateResponse>): grpc.ClientUnaryCall;
  GetSubscriptionAndPipelinesState(argument: _google_protobuf_Empty_DONTUSE, callback: grpc.requestCallback<_ficus_GrpcSubscriptionAndPipelinesStateResponse>): grpc.ClientUnaryCall;
  getSubscriptionAndPipelinesState(argument: _google_protobuf_Empty_DONTUSE, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_ficus_GrpcSubscriptionAndPipelinesStateResponse>): grpc.ClientUnaryCall;
  getSubscriptionAndPipelinesState(argument: _google_protobuf_Empty_DONTUSE, metadata: grpc.Metadata, callback: grpc.requestCallback<_ficus_GrpcSubscriptionAndPipelinesStateResponse>): grpc.ClientUnaryCall;
  getSubscriptionAndPipelinesState(argument: _google_protobuf_Empty_DONTUSE, options: grpc.CallOptions, callback: grpc.requestCallback<_ficus_GrpcSubscriptionAndPipelinesStateResponse>): grpc.ClientUnaryCall;
  getSubscriptionAndPipelinesState(argument: _google_protobuf_Empty_DONTUSE, callback: grpc.requestCallback<_ficus_GrpcSubscriptionAndPipelinesStateResponse>): grpc.ClientUnaryCall;
  
}

export interface GrpcPipelinePartsContextValuesServiceHandlers extends grpc.UntypedServiceImplementation {
  GetPipelineCaseContextValue: grpc.handleUnaryCall<_ficus_GrpcGetPipelineCaseContextValuesRequest, _ficus_GrpcCaseContextValues_DONTUSE>;
  
  GetSubscriptionAndPipelinesState: grpc.handleUnaryCall<_google_protobuf_Empty, _ficus_GrpcSubscriptionAndPipelinesStateResponse_DONTUSE>;
  
}

export interface GrpcPipelinePartsContextValuesServiceDefinition extends grpc.ServiceDefinition {
  GetPipelineCaseContextValue: MethodDefinition<_ficus_GrpcGetPipelineCaseContextValuesRequest_DONTUSE, _ficus_GrpcCaseContextValues_DONTUSE, _ficus_GrpcGetPipelineCaseContextValuesRequest, _ficus_GrpcCaseContextValues>
  GetSubscriptionAndPipelinesState: MethodDefinition<_google_protobuf_Empty_DONTUSE, _ficus_GrpcSubscriptionAndPipelinesStateResponse_DONTUSE, _google_protobuf_Empty, _ficus_GrpcSubscriptionAndPipelinesStateResponse>
}
