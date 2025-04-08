// Original file: /Users/aero/work/workspace/Ficus/protos/front_contract.proto

import type * as grpc from '@grpc/grpc-js'
import type { MethodDefinition } from '@grpc/proto-loader'
import type { Empty_DONTUSE as _google_protobuf_Empty_DONTUSE, Empty as _google_protobuf_Empty } from '../google/protobuf/Empty';
import type { GrpcPipelinePartUpdate_DONTUSE as _ficus_GrpcPipelinePartUpdate_DONTUSE, GrpcPipelinePartUpdate as _ficus_GrpcPipelinePartUpdate } from '../ficus/GrpcPipelinePartUpdate';

export interface GrpcPipelinePartsContextValuesServiceClient extends grpc.Client {
  StartUpdatesStream(argument: _google_protobuf_Empty_DONTUSE, metadata: grpc.Metadata, options?: grpc.CallOptions): grpc.ClientReadableStream<_ficus_GrpcPipelinePartUpdate>;
  StartUpdatesStream(argument: _google_protobuf_Empty_DONTUSE, options?: grpc.CallOptions): grpc.ClientReadableStream<_ficus_GrpcPipelinePartUpdate>;
  startUpdatesStream(argument: _google_protobuf_Empty_DONTUSE, metadata: grpc.Metadata, options?: grpc.CallOptions): grpc.ClientReadableStream<_ficus_GrpcPipelinePartUpdate>;
  startUpdatesStream(argument: _google_protobuf_Empty_DONTUSE, options?: grpc.CallOptions): grpc.ClientReadableStream<_ficus_GrpcPipelinePartUpdate>;
  
}

export interface GrpcPipelinePartsContextValuesServiceHandlers extends grpc.UntypedServiceImplementation {
  StartUpdatesStream: grpc.handleServerStreamingCall<_google_protobuf_Empty, _ficus_GrpcPipelinePartUpdate_DONTUSE>;
  
}

export interface GrpcPipelinePartsContextValuesServiceDefinition extends grpc.ServiceDefinition {
  StartUpdatesStream: MethodDefinition<_google_protobuf_Empty_DONTUSE, _ficus_GrpcPipelinePartUpdate_DONTUSE, _google_protobuf_Empty, _ficus_GrpcPipelinePartUpdate>
}
