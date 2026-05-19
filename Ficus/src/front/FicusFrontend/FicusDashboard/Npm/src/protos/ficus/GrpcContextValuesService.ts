// Original file: ../../../../../protos/context_values_service.proto

import type * as grpc from '@grpc/grpc-js'
import type { MethodDefinition } from '@grpc/proto-loader'
import type { Empty_DONTUSE as _google_protobuf_Empty_DONTUSE, Empty as _google_protobuf_Empty } from '../google/protobuf/Empty';
import type { GrpcContextValuePart_DONTUSE as _ficus_GrpcContextValuePart_DONTUSE, GrpcContextValuePart as _ficus_GrpcContextValuePart } from '../ficus/GrpcContextValuePart';
import type { GrpcDropContextValuesRequest_DONTUSE as _ficus_GrpcDropContextValuesRequest_DONTUSE, GrpcDropContextValuesRequest as _ficus_GrpcDropContextValuesRequest } from '../ficus/GrpcDropContextValuesRequest';
import type { GrpcGetAllContextValuesResult_DONTUSE as _ficus_GrpcGetAllContextValuesResult_DONTUSE, GrpcGetAllContextValuesResult as _ficus_GrpcGetAllContextValuesResult } from '../ficus/GrpcGetAllContextValuesResult';
import type { GrpcGetContextValueRequest_DONTUSE as _ficus_GrpcGetContextValueRequest_DONTUSE, GrpcGetContextValueRequest as _ficus_GrpcGetContextValueRequest } from '../ficus/GrpcGetContextValueRequest';
import type { GrpcGuid_DONTUSE as _ficus_GrpcGuid_DONTUSE, GrpcGuid as _ficus_GrpcGuid } from '../ficus/GrpcGuid';

export interface GrpcContextValuesServiceClient extends grpc.Client {
  DropContextValues(argument: _ficus_GrpcDropContextValuesRequest_DONTUSE, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_google_protobuf_Empty>): grpc.ClientUnaryCall;
  DropContextValues(argument: _ficus_GrpcDropContextValuesRequest_DONTUSE, metadata: grpc.Metadata, callback: grpc.requestCallback<_google_protobuf_Empty>): grpc.ClientUnaryCall;
  DropContextValues(argument: _ficus_GrpcDropContextValuesRequest_DONTUSE, options: grpc.CallOptions, callback: grpc.requestCallback<_google_protobuf_Empty>): grpc.ClientUnaryCall;
  DropContextValues(argument: _ficus_GrpcDropContextValuesRequest_DONTUSE, callback: grpc.requestCallback<_google_protobuf_Empty>): grpc.ClientUnaryCall;
  dropContextValues(argument: _ficus_GrpcDropContextValuesRequest_DONTUSE, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_google_protobuf_Empty>): grpc.ClientUnaryCall;
  dropContextValues(argument: _ficus_GrpcDropContextValuesRequest_DONTUSE, metadata: grpc.Metadata, callback: grpc.requestCallback<_google_protobuf_Empty>): grpc.ClientUnaryCall;
  dropContextValues(argument: _ficus_GrpcDropContextValuesRequest_DONTUSE, options: grpc.CallOptions, callback: grpc.requestCallback<_google_protobuf_Empty>): grpc.ClientUnaryCall;
  dropContextValues(argument: _ficus_GrpcDropContextValuesRequest_DONTUSE, callback: grpc.requestCallback<_google_protobuf_Empty>): grpc.ClientUnaryCall;
  
  GetAllContextValuesIds(argument: _ficus_GrpcGuid_DONTUSE, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_ficus_GrpcGetAllContextValuesResult>): grpc.ClientUnaryCall;
  GetAllContextValuesIds(argument: _ficus_GrpcGuid_DONTUSE, metadata: grpc.Metadata, callback: grpc.requestCallback<_ficus_GrpcGetAllContextValuesResult>): grpc.ClientUnaryCall;
  GetAllContextValuesIds(argument: _ficus_GrpcGuid_DONTUSE, options: grpc.CallOptions, callback: grpc.requestCallback<_ficus_GrpcGetAllContextValuesResult>): grpc.ClientUnaryCall;
  GetAllContextValuesIds(argument: _ficus_GrpcGuid_DONTUSE, callback: grpc.requestCallback<_ficus_GrpcGetAllContextValuesResult>): grpc.ClientUnaryCall;
  getAllContextValuesIds(argument: _ficus_GrpcGuid_DONTUSE, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_ficus_GrpcGetAllContextValuesResult>): grpc.ClientUnaryCall;
  getAllContextValuesIds(argument: _ficus_GrpcGuid_DONTUSE, metadata: grpc.Metadata, callback: grpc.requestCallback<_ficus_GrpcGetAllContextValuesResult>): grpc.ClientUnaryCall;
  getAllContextValuesIds(argument: _ficus_GrpcGuid_DONTUSE, options: grpc.CallOptions, callback: grpc.requestCallback<_ficus_GrpcGetAllContextValuesResult>): grpc.ClientUnaryCall;
  getAllContextValuesIds(argument: _ficus_GrpcGuid_DONTUSE, callback: grpc.requestCallback<_ficus_GrpcGetAllContextValuesResult>): grpc.ClientUnaryCall;
  
  GetContextValue(argument: _ficus_GrpcGuid_DONTUSE, metadata: grpc.Metadata, options?: grpc.CallOptions): grpc.ClientReadableStream<_ficus_GrpcContextValuePart>;
  GetContextValue(argument: _ficus_GrpcGuid_DONTUSE, options?: grpc.CallOptions): grpc.ClientReadableStream<_ficus_GrpcContextValuePart>;
  getContextValue(argument: _ficus_GrpcGuid_DONTUSE, metadata: grpc.Metadata, options?: grpc.CallOptions): grpc.ClientReadableStream<_ficus_GrpcContextValuePart>;
  getContextValue(argument: _ficus_GrpcGuid_DONTUSE, options?: grpc.CallOptions): grpc.ClientReadableStream<_ficus_GrpcContextValuePart>;
  
  GetContextValueId(argument: _ficus_GrpcGetContextValueRequest_DONTUSE, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_ficus_GrpcGuid>): grpc.ClientUnaryCall;
  GetContextValueId(argument: _ficus_GrpcGetContextValueRequest_DONTUSE, metadata: grpc.Metadata, callback: grpc.requestCallback<_ficus_GrpcGuid>): grpc.ClientUnaryCall;
  GetContextValueId(argument: _ficus_GrpcGetContextValueRequest_DONTUSE, options: grpc.CallOptions, callback: grpc.requestCallback<_ficus_GrpcGuid>): grpc.ClientUnaryCall;
  GetContextValueId(argument: _ficus_GrpcGetContextValueRequest_DONTUSE, callback: grpc.requestCallback<_ficus_GrpcGuid>): grpc.ClientUnaryCall;
  getContextValueId(argument: _ficus_GrpcGetContextValueRequest_DONTUSE, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_ficus_GrpcGuid>): grpc.ClientUnaryCall;
  getContextValueId(argument: _ficus_GrpcGetContextValueRequest_DONTUSE, metadata: grpc.Metadata, callback: grpc.requestCallback<_ficus_GrpcGuid>): grpc.ClientUnaryCall;
  getContextValueId(argument: _ficus_GrpcGetContextValueRequest_DONTUSE, options: grpc.CallOptions, callback: grpc.requestCallback<_ficus_GrpcGuid>): grpc.ClientUnaryCall;
  getContextValueId(argument: _ficus_GrpcGetContextValueRequest_DONTUSE, callback: grpc.requestCallback<_ficus_GrpcGuid>): grpc.ClientUnaryCall;
  
  SetContextValue(metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_ficus_GrpcGuid>): grpc.ClientWritableStream<_ficus_GrpcContextValuePart_DONTUSE>;
  SetContextValue(metadata: grpc.Metadata, callback: grpc.requestCallback<_ficus_GrpcGuid>): grpc.ClientWritableStream<_ficus_GrpcContextValuePart_DONTUSE>;
  SetContextValue(options: grpc.CallOptions, callback: grpc.requestCallback<_ficus_GrpcGuid>): grpc.ClientWritableStream<_ficus_GrpcContextValuePart_DONTUSE>;
  SetContextValue(callback: grpc.requestCallback<_ficus_GrpcGuid>): grpc.ClientWritableStream<_ficus_GrpcContextValuePart_DONTUSE>;
  setContextValue(metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_ficus_GrpcGuid>): grpc.ClientWritableStream<_ficus_GrpcContextValuePart_DONTUSE>;
  setContextValue(metadata: grpc.Metadata, callback: grpc.requestCallback<_ficus_GrpcGuid>): grpc.ClientWritableStream<_ficus_GrpcContextValuePart_DONTUSE>;
  setContextValue(options: grpc.CallOptions, callback: grpc.requestCallback<_ficus_GrpcGuid>): grpc.ClientWritableStream<_ficus_GrpcContextValuePart_DONTUSE>;
  setContextValue(callback: grpc.requestCallback<_ficus_GrpcGuid>): grpc.ClientWritableStream<_ficus_GrpcContextValuePart_DONTUSE>;
  
}

export interface GrpcContextValuesServiceHandlers extends grpc.UntypedServiceImplementation {
  DropContextValues: grpc.handleUnaryCall<_ficus_GrpcDropContextValuesRequest, _google_protobuf_Empty_DONTUSE>;
  
  GetAllContextValuesIds: grpc.handleUnaryCall<_ficus_GrpcGuid, _ficus_GrpcGetAllContextValuesResult_DONTUSE>;
  
  GetContextValue: grpc.handleServerStreamingCall<_ficus_GrpcGuid, _ficus_GrpcContextValuePart_DONTUSE>;
  
  GetContextValueId: grpc.handleUnaryCall<_ficus_GrpcGetContextValueRequest, _ficus_GrpcGuid_DONTUSE>;
  
  SetContextValue: grpc.handleClientStreamingCall<_ficus_GrpcContextValuePart, _ficus_GrpcGuid_DONTUSE>;
  
}

export interface GrpcContextValuesServiceDefinition extends grpc.ServiceDefinition {
  DropContextValues: MethodDefinition<_ficus_GrpcDropContextValuesRequest_DONTUSE, _google_protobuf_Empty_DONTUSE, _ficus_GrpcDropContextValuesRequest, _google_protobuf_Empty>
  GetAllContextValuesIds: MethodDefinition<_ficus_GrpcGuid_DONTUSE, _ficus_GrpcGetAllContextValuesResult_DONTUSE, _ficus_GrpcGuid, _ficus_GrpcGetAllContextValuesResult>
  GetContextValue: MethodDefinition<_ficus_GrpcGuid_DONTUSE, _ficus_GrpcContextValuePart_DONTUSE, _ficus_GrpcGuid, _ficus_GrpcContextValuePart>
  GetContextValueId: MethodDefinition<_ficus_GrpcGetContextValueRequest_DONTUSE, _ficus_GrpcGuid_DONTUSE, _ficus_GrpcGetContextValueRequest, _ficus_GrpcGuid>
  SetContextValue: MethodDefinition<_ficus_GrpcContextValuePart_DONTUSE, _ficus_GrpcGuid_DONTUSE, _ficus_GrpcContextValuePart, _ficus_GrpcGuid>
}
