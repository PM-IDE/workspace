// Original file: ../../../../../protos/backend_service.proto

import type * as grpc from '@grpc/grpc-js'
import type { MethodDefinition } from '@grpc/proto-loader'
import type { Empty_DONTUSE as _google_protobuf_Empty_DONTUSE, Empty as _google_protobuf_Empty } from '../google/protobuf/Empty';
import type { GrpcFicusBackendInfo_DONTUSE as _ficus_GrpcFicusBackendInfo_DONTUSE, GrpcFicusBackendInfo as _ficus_GrpcFicusBackendInfo } from '../ficus/GrpcFicusBackendInfo';
import type { GrpcGetAllContextValuesResult_DONTUSE as _ficus_GrpcGetAllContextValuesResult_DONTUSE, GrpcGetAllContextValuesResult as _ficus_GrpcGetAllContextValuesResult } from '../ficus/GrpcGetAllContextValuesResult';
import type { GrpcGetContextValueRequest_DONTUSE as _ficus_GrpcGetContextValueRequest_DONTUSE, GrpcGetContextValueRequest as _ficus_GrpcGetContextValueRequest } from '../ficus/GrpcGetContextValueRequest';
import type { GrpcGetContextValueResult_DONTUSE as _ficus_GrpcGetContextValueResult_DONTUSE, GrpcGetContextValueResult as _ficus_GrpcGetContextValueResult } from '../ficus/GrpcGetContextValueResult';
import type { GrpcGuid_DONTUSE as _ficus_GrpcGuid_DONTUSE, GrpcGuid as _ficus_GrpcGuid } from '../ficus/GrpcGuid';
import type { GrpcPipelinePartExecutionResult_DONTUSE as _ficus_GrpcPipelinePartExecutionResult_DONTUSE, GrpcPipelinePartExecutionResult as _ficus_GrpcPipelinePartExecutionResult } from '../ficus/GrpcPipelinePartExecutionResult';
import type { GrpcProxyPipelineExecutionRequest_DONTUSE as _ficus_GrpcProxyPipelineExecutionRequest_DONTUSE, GrpcProxyPipelineExecutionRequest as _ficus_GrpcProxyPipelineExecutionRequest } from '../ficus/GrpcProxyPipelineExecutionRequest';

export interface GrpcBackendServiceClient extends grpc.Client {
  DropExecutionResult(argument: _ficus_GrpcGuid_DONTUSE, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_google_protobuf_Empty>): grpc.ClientUnaryCall;
  DropExecutionResult(argument: _ficus_GrpcGuid_DONTUSE, metadata: grpc.Metadata, callback: grpc.requestCallback<_google_protobuf_Empty>): grpc.ClientUnaryCall;
  DropExecutionResult(argument: _ficus_GrpcGuid_DONTUSE, options: grpc.CallOptions, callback: grpc.requestCallback<_google_protobuf_Empty>): grpc.ClientUnaryCall;
  DropExecutionResult(argument: _ficus_GrpcGuid_DONTUSE, callback: grpc.requestCallback<_google_protobuf_Empty>): grpc.ClientUnaryCall;
  dropExecutionResult(argument: _ficus_GrpcGuid_DONTUSE, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_google_protobuf_Empty>): grpc.ClientUnaryCall;
  dropExecutionResult(argument: _ficus_GrpcGuid_DONTUSE, metadata: grpc.Metadata, callback: grpc.requestCallback<_google_protobuf_Empty>): grpc.ClientUnaryCall;
  dropExecutionResult(argument: _ficus_GrpcGuid_DONTUSE, options: grpc.CallOptions, callback: grpc.requestCallback<_google_protobuf_Empty>): grpc.ClientUnaryCall;
  dropExecutionResult(argument: _ficus_GrpcGuid_DONTUSE, callback: grpc.requestCallback<_google_protobuf_Empty>): grpc.ClientUnaryCall;
  
  ExecutePipeline(argument: _ficus_GrpcProxyPipelineExecutionRequest_DONTUSE, metadata: grpc.Metadata, options?: grpc.CallOptions): grpc.ClientReadableStream<_ficus_GrpcPipelinePartExecutionResult>;
  ExecutePipeline(argument: _ficus_GrpcProxyPipelineExecutionRequest_DONTUSE, options?: grpc.CallOptions): grpc.ClientReadableStream<_ficus_GrpcPipelinePartExecutionResult>;
  executePipeline(argument: _ficus_GrpcProxyPipelineExecutionRequest_DONTUSE, metadata: grpc.Metadata, options?: grpc.CallOptions): grpc.ClientReadableStream<_ficus_GrpcPipelinePartExecutionResult>;
  executePipeline(argument: _ficus_GrpcProxyPipelineExecutionRequest_DONTUSE, options?: grpc.CallOptions): grpc.ClientReadableStream<_ficus_GrpcPipelinePartExecutionResult>;
  
  GetAllContextValues(argument: _ficus_GrpcGuid_DONTUSE, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_ficus_GrpcGetAllContextValuesResult>): grpc.ClientUnaryCall;
  GetAllContextValues(argument: _ficus_GrpcGuid_DONTUSE, metadata: grpc.Metadata, callback: grpc.requestCallback<_ficus_GrpcGetAllContextValuesResult>): grpc.ClientUnaryCall;
  GetAllContextValues(argument: _ficus_GrpcGuid_DONTUSE, options: grpc.CallOptions, callback: grpc.requestCallback<_ficus_GrpcGetAllContextValuesResult>): grpc.ClientUnaryCall;
  GetAllContextValues(argument: _ficus_GrpcGuid_DONTUSE, callback: grpc.requestCallback<_ficus_GrpcGetAllContextValuesResult>): grpc.ClientUnaryCall;
  getAllContextValues(argument: _ficus_GrpcGuid_DONTUSE, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_ficus_GrpcGetAllContextValuesResult>): grpc.ClientUnaryCall;
  getAllContextValues(argument: _ficus_GrpcGuid_DONTUSE, metadata: grpc.Metadata, callback: grpc.requestCallback<_ficus_GrpcGetAllContextValuesResult>): grpc.ClientUnaryCall;
  getAllContextValues(argument: _ficus_GrpcGuid_DONTUSE, options: grpc.CallOptions, callback: grpc.requestCallback<_ficus_GrpcGetAllContextValuesResult>): grpc.ClientUnaryCall;
  getAllContextValues(argument: _ficus_GrpcGuid_DONTUSE, callback: grpc.requestCallback<_ficus_GrpcGetAllContextValuesResult>): grpc.ClientUnaryCall;
  
  GetBackendInfo(argument: _google_protobuf_Empty_DONTUSE, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_ficus_GrpcFicusBackendInfo>): grpc.ClientUnaryCall;
  GetBackendInfo(argument: _google_protobuf_Empty_DONTUSE, metadata: grpc.Metadata, callback: grpc.requestCallback<_ficus_GrpcFicusBackendInfo>): grpc.ClientUnaryCall;
  GetBackendInfo(argument: _google_protobuf_Empty_DONTUSE, options: grpc.CallOptions, callback: grpc.requestCallback<_ficus_GrpcFicusBackendInfo>): grpc.ClientUnaryCall;
  GetBackendInfo(argument: _google_protobuf_Empty_DONTUSE, callback: grpc.requestCallback<_ficus_GrpcFicusBackendInfo>): grpc.ClientUnaryCall;
  getBackendInfo(argument: _google_protobuf_Empty_DONTUSE, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_ficus_GrpcFicusBackendInfo>): grpc.ClientUnaryCall;
  getBackendInfo(argument: _google_protobuf_Empty_DONTUSE, metadata: grpc.Metadata, callback: grpc.requestCallback<_ficus_GrpcFicusBackendInfo>): grpc.ClientUnaryCall;
  getBackendInfo(argument: _google_protobuf_Empty_DONTUSE, options: grpc.CallOptions, callback: grpc.requestCallback<_ficus_GrpcFicusBackendInfo>): grpc.ClientUnaryCall;
  getBackendInfo(argument: _google_protobuf_Empty_DONTUSE, callback: grpc.requestCallback<_ficus_GrpcFicusBackendInfo>): grpc.ClientUnaryCall;
  
  GetContextValue(argument: _ficus_GrpcGetContextValueRequest_DONTUSE, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_ficus_GrpcGetContextValueResult>): grpc.ClientUnaryCall;
  GetContextValue(argument: _ficus_GrpcGetContextValueRequest_DONTUSE, metadata: grpc.Metadata, callback: grpc.requestCallback<_ficus_GrpcGetContextValueResult>): grpc.ClientUnaryCall;
  GetContextValue(argument: _ficus_GrpcGetContextValueRequest_DONTUSE, options: grpc.CallOptions, callback: grpc.requestCallback<_ficus_GrpcGetContextValueResult>): grpc.ClientUnaryCall;
  GetContextValue(argument: _ficus_GrpcGetContextValueRequest_DONTUSE, callback: grpc.requestCallback<_ficus_GrpcGetContextValueResult>): grpc.ClientUnaryCall;
  getContextValue(argument: _ficus_GrpcGetContextValueRequest_DONTUSE, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_ficus_GrpcGetContextValueResult>): grpc.ClientUnaryCall;
  getContextValue(argument: _ficus_GrpcGetContextValueRequest_DONTUSE, metadata: grpc.Metadata, callback: grpc.requestCallback<_ficus_GrpcGetContextValueResult>): grpc.ClientUnaryCall;
  getContextValue(argument: _ficus_GrpcGetContextValueRequest_DONTUSE, options: grpc.CallOptions, callback: grpc.requestCallback<_ficus_GrpcGetContextValueResult>): grpc.ClientUnaryCall;
  getContextValue(argument: _ficus_GrpcGetContextValueRequest_DONTUSE, callback: grpc.requestCallback<_ficus_GrpcGetContextValueResult>): grpc.ClientUnaryCall;
  
}

export interface GrpcBackendServiceHandlers extends grpc.UntypedServiceImplementation {
  DropExecutionResult: grpc.handleUnaryCall<_ficus_GrpcGuid, _google_protobuf_Empty_DONTUSE>;
  
  ExecutePipeline: grpc.handleServerStreamingCall<_ficus_GrpcProxyPipelineExecutionRequest, _ficus_GrpcPipelinePartExecutionResult_DONTUSE>;
  
  GetAllContextValues: grpc.handleUnaryCall<_ficus_GrpcGuid, _ficus_GrpcGetAllContextValuesResult_DONTUSE>;
  
  GetBackendInfo: grpc.handleUnaryCall<_google_protobuf_Empty, _ficus_GrpcFicusBackendInfo_DONTUSE>;
  
  GetContextValue: grpc.handleUnaryCall<_ficus_GrpcGetContextValueRequest, _ficus_GrpcGetContextValueResult_DONTUSE>;
  
}

export interface GrpcBackendServiceDefinition extends grpc.ServiceDefinition {
  DropExecutionResult: MethodDefinition<_ficus_GrpcGuid_DONTUSE, _google_protobuf_Empty_DONTUSE, _ficus_GrpcGuid, _google_protobuf_Empty>
  ExecutePipeline: MethodDefinition<_ficus_GrpcProxyPipelineExecutionRequest_DONTUSE, _ficus_GrpcPipelinePartExecutionResult_DONTUSE, _ficus_GrpcProxyPipelineExecutionRequest, _ficus_GrpcPipelinePartExecutionResult>
  GetAllContextValues: MethodDefinition<_ficus_GrpcGuid_DONTUSE, _ficus_GrpcGetAllContextValuesResult_DONTUSE, _ficus_GrpcGuid, _ficus_GrpcGetAllContextValuesResult>
  GetBackendInfo: MethodDefinition<_google_protobuf_Empty_DONTUSE, _ficus_GrpcFicusBackendInfo_DONTUSE, _google_protobuf_Empty, _ficus_GrpcFicusBackendInfo>
  GetContextValue: MethodDefinition<_ficus_GrpcGetContextValueRequest_DONTUSE, _ficus_GrpcGetContextValueResult_DONTUSE, _ficus_GrpcGetContextValueRequest, _ficus_GrpcGetContextValueResult>
}
