// Original file: ../../../../../protos/backend_service.proto

import type * as grpc from '@grpc/grpc-js'
import type { MethodDefinition } from '@grpc/proto-loader'
import type { Empty_DONTUSE as _google_protobuf_Empty_DONTUSE, Empty as _google_protobuf_Empty } from '../google/protobuf/Empty';
import type { GrpcPredefinedPipelinePartsToBackendsMap_DONTUSE as _ficus_GrpcPredefinedPipelinePartsToBackendsMap_DONTUSE, GrpcPredefinedPipelinePartsToBackendsMap as _ficus_GrpcPredefinedPipelinePartsToBackendsMap } from '../ficus/GrpcPredefinedPipelinePartsToBackendsMap';

export interface GrpcBackendBalancerServiceClient extends grpc.Client {
  SetPipelinePartsToBackendsMap(argument: _ficus_GrpcPredefinedPipelinePartsToBackendsMap_DONTUSE, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_google_protobuf_Empty>): grpc.ClientUnaryCall;
  SetPipelinePartsToBackendsMap(argument: _ficus_GrpcPredefinedPipelinePartsToBackendsMap_DONTUSE, metadata: grpc.Metadata, callback: grpc.requestCallback<_google_protobuf_Empty>): grpc.ClientUnaryCall;
  SetPipelinePartsToBackendsMap(argument: _ficus_GrpcPredefinedPipelinePartsToBackendsMap_DONTUSE, options: grpc.CallOptions, callback: grpc.requestCallback<_google_protobuf_Empty>): grpc.ClientUnaryCall;
  SetPipelinePartsToBackendsMap(argument: _ficus_GrpcPredefinedPipelinePartsToBackendsMap_DONTUSE, callback: grpc.requestCallback<_google_protobuf_Empty>): grpc.ClientUnaryCall;
  setPipelinePartsToBackendsMap(argument: _ficus_GrpcPredefinedPipelinePartsToBackendsMap_DONTUSE, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_google_protobuf_Empty>): grpc.ClientUnaryCall;
  setPipelinePartsToBackendsMap(argument: _ficus_GrpcPredefinedPipelinePartsToBackendsMap_DONTUSE, metadata: grpc.Metadata, callback: grpc.requestCallback<_google_protobuf_Empty>): grpc.ClientUnaryCall;
  setPipelinePartsToBackendsMap(argument: _ficus_GrpcPredefinedPipelinePartsToBackendsMap_DONTUSE, options: grpc.CallOptions, callback: grpc.requestCallback<_google_protobuf_Empty>): grpc.ClientUnaryCall;
  setPipelinePartsToBackendsMap(argument: _ficus_GrpcPredefinedPipelinePartsToBackendsMap_DONTUSE, callback: grpc.requestCallback<_google_protobuf_Empty>): grpc.ClientUnaryCall;
  
}

export interface GrpcBackendBalancerServiceHandlers extends grpc.UntypedServiceImplementation {
  SetPipelinePartsToBackendsMap: grpc.handleUnaryCall<_ficus_GrpcPredefinedPipelinePartsToBackendsMap, _google_protobuf_Empty_DONTUSE>;
  
}

export interface GrpcBackendBalancerServiceDefinition extends grpc.ServiceDefinition {
  SetPipelinePartsToBackendsMap: MethodDefinition<_ficus_GrpcPredefinedPipelinePartsToBackendsMap_DONTUSE, _google_protobuf_Empty_DONTUSE, _ficus_GrpcPredefinedPipelinePartsToBackendsMap, _google_protobuf_Empty>
}
