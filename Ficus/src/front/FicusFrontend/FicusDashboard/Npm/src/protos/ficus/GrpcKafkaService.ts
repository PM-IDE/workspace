// Original file: ../../../../../protos/kafka_service.proto

import type * as grpc from '@grpc/grpc-js'
import type { MethodDefinition } from '@grpc/proto-loader'
import type { Empty_DONTUSE as _google_protobuf_Empty_DONTUSE, Empty as _google_protobuf_Empty } from '../google/protobuf/Empty';
import type { GrpcAddPipelineRequest_DONTUSE as _ficus_GrpcAddPipelineRequest_DONTUSE, GrpcAddPipelineRequest as _ficus_GrpcAddPipelineRequest } from '../ficus/GrpcAddPipelineRequest';
import type { GrpcAddPipelineStreamRequest_DONTUSE as _ficus_GrpcAddPipelineStreamRequest_DONTUSE, GrpcAddPipelineStreamRequest as _ficus_GrpcAddPipelineStreamRequest } from '../ficus/GrpcAddPipelineStreamRequest';
import type { GrpcExecutePipelineAndProduceKafkaRequest_DONTUSE as _ficus_GrpcExecutePipelineAndProduceKafkaRequest_DONTUSE, GrpcExecutePipelineAndProduceKafkaRequest as _ficus_GrpcExecutePipelineAndProduceKafkaRequest } from '../ficus/GrpcExecutePipelineAndProduceKafkaRequest';
import type { GrpcGetAllSubscriptionsAndPipelinesResponse_DONTUSE as _ficus_GrpcGetAllSubscriptionsAndPipelinesResponse_DONTUSE, GrpcGetAllSubscriptionsAndPipelinesResponse as _ficus_GrpcGetAllSubscriptionsAndPipelinesResponse } from '../ficus/GrpcGetAllSubscriptionsAndPipelinesResponse';
import type { GrpcKafkaResult_DONTUSE as _ficus_GrpcKafkaResult_DONTUSE, GrpcKafkaResult as _ficus_GrpcKafkaResult } from '../ficus/GrpcKafkaResult';
import type { GrpcPipelinePartExecutionResult_DONTUSE as _ficus_GrpcPipelinePartExecutionResult_DONTUSE, GrpcPipelinePartExecutionResult as _ficus_GrpcPipelinePartExecutionResult } from '../ficus/GrpcPipelinePartExecutionResult';
import type { GrpcRemoveAllPipelinesRequest_DONTUSE as _ficus_GrpcRemoveAllPipelinesRequest_DONTUSE, GrpcRemoveAllPipelinesRequest as _ficus_GrpcRemoveAllPipelinesRequest } from '../ficus/GrpcRemoveAllPipelinesRequest';
import type { GrpcRemovePipelineRequest_DONTUSE as _ficus_GrpcRemovePipelineRequest_DONTUSE, GrpcRemovePipelineRequest as _ficus_GrpcRemovePipelineRequest } from '../ficus/GrpcRemovePipelineRequest';
import type { GrpcSubscribeToKafkaRequest_DONTUSE as _ficus_GrpcSubscribeToKafkaRequest_DONTUSE, GrpcSubscribeToKafkaRequest as _ficus_GrpcSubscribeToKafkaRequest } from '../ficus/GrpcSubscribeToKafkaRequest';
import type { GrpcUnsubscribeFromKafkaRequest_DONTUSE as _ficus_GrpcUnsubscribeFromKafkaRequest_DONTUSE, GrpcUnsubscribeFromKafkaRequest as _ficus_GrpcUnsubscribeFromKafkaRequest } from '../ficus/GrpcUnsubscribeFromKafkaRequest';

export interface GrpcKafkaServiceClient extends grpc.Client {
  AddPipelineToSubscription(argument: _ficus_GrpcAddPipelineRequest_DONTUSE, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_ficus_GrpcKafkaResult>): grpc.ClientUnaryCall;
  AddPipelineToSubscription(argument: _ficus_GrpcAddPipelineRequest_DONTUSE, metadata: grpc.Metadata, callback: grpc.requestCallback<_ficus_GrpcKafkaResult>): grpc.ClientUnaryCall;
  AddPipelineToSubscription(argument: _ficus_GrpcAddPipelineRequest_DONTUSE, options: grpc.CallOptions, callback: grpc.requestCallback<_ficus_GrpcKafkaResult>): grpc.ClientUnaryCall;
  AddPipelineToSubscription(argument: _ficus_GrpcAddPipelineRequest_DONTUSE, callback: grpc.requestCallback<_ficus_GrpcKafkaResult>): grpc.ClientUnaryCall;
  addPipelineToSubscription(argument: _ficus_GrpcAddPipelineRequest_DONTUSE, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_ficus_GrpcKafkaResult>): grpc.ClientUnaryCall;
  addPipelineToSubscription(argument: _ficus_GrpcAddPipelineRequest_DONTUSE, metadata: grpc.Metadata, callback: grpc.requestCallback<_ficus_GrpcKafkaResult>): grpc.ClientUnaryCall;
  addPipelineToSubscription(argument: _ficus_GrpcAddPipelineRequest_DONTUSE, options: grpc.CallOptions, callback: grpc.requestCallback<_ficus_GrpcKafkaResult>): grpc.ClientUnaryCall;
  addPipelineToSubscription(argument: _ficus_GrpcAddPipelineRequest_DONTUSE, callback: grpc.requestCallback<_ficus_GrpcKafkaResult>): grpc.ClientUnaryCall;
  
  AddPipelineToSubscriptionStream(argument: _ficus_GrpcAddPipelineStreamRequest_DONTUSE, metadata: grpc.Metadata, options?: grpc.CallOptions): grpc.ClientReadableStream<_ficus_GrpcPipelinePartExecutionResult>;
  AddPipelineToSubscriptionStream(argument: _ficus_GrpcAddPipelineStreamRequest_DONTUSE, options?: grpc.CallOptions): grpc.ClientReadableStream<_ficus_GrpcPipelinePartExecutionResult>;
  addPipelineToSubscriptionStream(argument: _ficus_GrpcAddPipelineStreamRequest_DONTUSE, metadata: grpc.Metadata, options?: grpc.CallOptions): grpc.ClientReadableStream<_ficus_GrpcPipelinePartExecutionResult>;
  addPipelineToSubscriptionStream(argument: _ficus_GrpcAddPipelineStreamRequest_DONTUSE, options?: grpc.CallOptions): grpc.ClientReadableStream<_ficus_GrpcPipelinePartExecutionResult>;
  
  ExecutePipelineAndProduceToKafka(argument: _ficus_GrpcExecutePipelineAndProduceKafkaRequest_DONTUSE, metadata: grpc.Metadata, options?: grpc.CallOptions): grpc.ClientReadableStream<_ficus_GrpcPipelinePartExecutionResult>;
  ExecutePipelineAndProduceToKafka(argument: _ficus_GrpcExecutePipelineAndProduceKafkaRequest_DONTUSE, options?: grpc.CallOptions): grpc.ClientReadableStream<_ficus_GrpcPipelinePartExecutionResult>;
  executePipelineAndProduceToKafka(argument: _ficus_GrpcExecutePipelineAndProduceKafkaRequest_DONTUSE, metadata: grpc.Metadata, options?: grpc.CallOptions): grpc.ClientReadableStream<_ficus_GrpcPipelinePartExecutionResult>;
  executePipelineAndProduceToKafka(argument: _ficus_GrpcExecutePipelineAndProduceKafkaRequest_DONTUSE, options?: grpc.CallOptions): grpc.ClientReadableStream<_ficus_GrpcPipelinePartExecutionResult>;
  
  GetAllSubscriptionsAndPipelines(argument: _google_protobuf_Empty_DONTUSE, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_ficus_GrpcGetAllSubscriptionsAndPipelinesResponse>): grpc.ClientUnaryCall;
  GetAllSubscriptionsAndPipelines(argument: _google_protobuf_Empty_DONTUSE, metadata: grpc.Metadata, callback: grpc.requestCallback<_ficus_GrpcGetAllSubscriptionsAndPipelinesResponse>): grpc.ClientUnaryCall;
  GetAllSubscriptionsAndPipelines(argument: _google_protobuf_Empty_DONTUSE, options: grpc.CallOptions, callback: grpc.requestCallback<_ficus_GrpcGetAllSubscriptionsAndPipelinesResponse>): grpc.ClientUnaryCall;
  GetAllSubscriptionsAndPipelines(argument: _google_protobuf_Empty_DONTUSE, callback: grpc.requestCallback<_ficus_GrpcGetAllSubscriptionsAndPipelinesResponse>): grpc.ClientUnaryCall;
  getAllSubscriptionsAndPipelines(argument: _google_protobuf_Empty_DONTUSE, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_ficus_GrpcGetAllSubscriptionsAndPipelinesResponse>): grpc.ClientUnaryCall;
  getAllSubscriptionsAndPipelines(argument: _google_protobuf_Empty_DONTUSE, metadata: grpc.Metadata, callback: grpc.requestCallback<_ficus_GrpcGetAllSubscriptionsAndPipelinesResponse>): grpc.ClientUnaryCall;
  getAllSubscriptionsAndPipelines(argument: _google_protobuf_Empty_DONTUSE, options: grpc.CallOptions, callback: grpc.requestCallback<_ficus_GrpcGetAllSubscriptionsAndPipelinesResponse>): grpc.ClientUnaryCall;
  getAllSubscriptionsAndPipelines(argument: _google_protobuf_Empty_DONTUSE, callback: grpc.requestCallback<_ficus_GrpcGetAllSubscriptionsAndPipelinesResponse>): grpc.ClientUnaryCall;
  
  RemoveAllPipelineSubscriptions(argument: _ficus_GrpcRemoveAllPipelinesRequest_DONTUSE, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_ficus_GrpcKafkaResult>): grpc.ClientUnaryCall;
  RemoveAllPipelineSubscriptions(argument: _ficus_GrpcRemoveAllPipelinesRequest_DONTUSE, metadata: grpc.Metadata, callback: grpc.requestCallback<_ficus_GrpcKafkaResult>): grpc.ClientUnaryCall;
  RemoveAllPipelineSubscriptions(argument: _ficus_GrpcRemoveAllPipelinesRequest_DONTUSE, options: grpc.CallOptions, callback: grpc.requestCallback<_ficus_GrpcKafkaResult>): grpc.ClientUnaryCall;
  RemoveAllPipelineSubscriptions(argument: _ficus_GrpcRemoveAllPipelinesRequest_DONTUSE, callback: grpc.requestCallback<_ficus_GrpcKafkaResult>): grpc.ClientUnaryCall;
  removeAllPipelineSubscriptions(argument: _ficus_GrpcRemoveAllPipelinesRequest_DONTUSE, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_ficus_GrpcKafkaResult>): grpc.ClientUnaryCall;
  removeAllPipelineSubscriptions(argument: _ficus_GrpcRemoveAllPipelinesRequest_DONTUSE, metadata: grpc.Metadata, callback: grpc.requestCallback<_ficus_GrpcKafkaResult>): grpc.ClientUnaryCall;
  removeAllPipelineSubscriptions(argument: _ficus_GrpcRemoveAllPipelinesRequest_DONTUSE, options: grpc.CallOptions, callback: grpc.requestCallback<_ficus_GrpcKafkaResult>): grpc.ClientUnaryCall;
  removeAllPipelineSubscriptions(argument: _ficus_GrpcRemoveAllPipelinesRequest_DONTUSE, callback: grpc.requestCallback<_ficus_GrpcKafkaResult>): grpc.ClientUnaryCall;
  
  RemovePipelineSubscription(argument: _ficus_GrpcRemovePipelineRequest_DONTUSE, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_ficus_GrpcKafkaResult>): grpc.ClientUnaryCall;
  RemovePipelineSubscription(argument: _ficus_GrpcRemovePipelineRequest_DONTUSE, metadata: grpc.Metadata, callback: grpc.requestCallback<_ficus_GrpcKafkaResult>): grpc.ClientUnaryCall;
  RemovePipelineSubscription(argument: _ficus_GrpcRemovePipelineRequest_DONTUSE, options: grpc.CallOptions, callback: grpc.requestCallback<_ficus_GrpcKafkaResult>): grpc.ClientUnaryCall;
  RemovePipelineSubscription(argument: _ficus_GrpcRemovePipelineRequest_DONTUSE, callback: grpc.requestCallback<_ficus_GrpcKafkaResult>): grpc.ClientUnaryCall;
  removePipelineSubscription(argument: _ficus_GrpcRemovePipelineRequest_DONTUSE, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_ficus_GrpcKafkaResult>): grpc.ClientUnaryCall;
  removePipelineSubscription(argument: _ficus_GrpcRemovePipelineRequest_DONTUSE, metadata: grpc.Metadata, callback: grpc.requestCallback<_ficus_GrpcKafkaResult>): grpc.ClientUnaryCall;
  removePipelineSubscription(argument: _ficus_GrpcRemovePipelineRequest_DONTUSE, options: grpc.CallOptions, callback: grpc.requestCallback<_ficus_GrpcKafkaResult>): grpc.ClientUnaryCall;
  removePipelineSubscription(argument: _ficus_GrpcRemovePipelineRequest_DONTUSE, callback: grpc.requestCallback<_ficus_GrpcKafkaResult>): grpc.ClientUnaryCall;
  
  SubscribeForKafkaTopic(argument: _ficus_GrpcSubscribeToKafkaRequest_DONTUSE, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_ficus_GrpcKafkaResult>): grpc.ClientUnaryCall;
  SubscribeForKafkaTopic(argument: _ficus_GrpcSubscribeToKafkaRequest_DONTUSE, metadata: grpc.Metadata, callback: grpc.requestCallback<_ficus_GrpcKafkaResult>): grpc.ClientUnaryCall;
  SubscribeForKafkaTopic(argument: _ficus_GrpcSubscribeToKafkaRequest_DONTUSE, options: grpc.CallOptions, callback: grpc.requestCallback<_ficus_GrpcKafkaResult>): grpc.ClientUnaryCall;
  SubscribeForKafkaTopic(argument: _ficus_GrpcSubscribeToKafkaRequest_DONTUSE, callback: grpc.requestCallback<_ficus_GrpcKafkaResult>): grpc.ClientUnaryCall;
  subscribeForKafkaTopic(argument: _ficus_GrpcSubscribeToKafkaRequest_DONTUSE, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_ficus_GrpcKafkaResult>): grpc.ClientUnaryCall;
  subscribeForKafkaTopic(argument: _ficus_GrpcSubscribeToKafkaRequest_DONTUSE, metadata: grpc.Metadata, callback: grpc.requestCallback<_ficus_GrpcKafkaResult>): grpc.ClientUnaryCall;
  subscribeForKafkaTopic(argument: _ficus_GrpcSubscribeToKafkaRequest_DONTUSE, options: grpc.CallOptions, callback: grpc.requestCallback<_ficus_GrpcKafkaResult>): grpc.ClientUnaryCall;
  subscribeForKafkaTopic(argument: _ficus_GrpcSubscribeToKafkaRequest_DONTUSE, callback: grpc.requestCallback<_ficus_GrpcKafkaResult>): grpc.ClientUnaryCall;
  
  UnsubscribeFromKafkaTopic(argument: _ficus_GrpcUnsubscribeFromKafkaRequest_DONTUSE, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_ficus_GrpcKafkaResult>): grpc.ClientUnaryCall;
  UnsubscribeFromKafkaTopic(argument: _ficus_GrpcUnsubscribeFromKafkaRequest_DONTUSE, metadata: grpc.Metadata, callback: grpc.requestCallback<_ficus_GrpcKafkaResult>): grpc.ClientUnaryCall;
  UnsubscribeFromKafkaTopic(argument: _ficus_GrpcUnsubscribeFromKafkaRequest_DONTUSE, options: grpc.CallOptions, callback: grpc.requestCallback<_ficus_GrpcKafkaResult>): grpc.ClientUnaryCall;
  UnsubscribeFromKafkaTopic(argument: _ficus_GrpcUnsubscribeFromKafkaRequest_DONTUSE, callback: grpc.requestCallback<_ficus_GrpcKafkaResult>): grpc.ClientUnaryCall;
  unsubscribeFromKafkaTopic(argument: _ficus_GrpcUnsubscribeFromKafkaRequest_DONTUSE, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_ficus_GrpcKafkaResult>): grpc.ClientUnaryCall;
  unsubscribeFromKafkaTopic(argument: _ficus_GrpcUnsubscribeFromKafkaRequest_DONTUSE, metadata: grpc.Metadata, callback: grpc.requestCallback<_ficus_GrpcKafkaResult>): grpc.ClientUnaryCall;
  unsubscribeFromKafkaTopic(argument: _ficus_GrpcUnsubscribeFromKafkaRequest_DONTUSE, options: grpc.CallOptions, callback: grpc.requestCallback<_ficus_GrpcKafkaResult>): grpc.ClientUnaryCall;
  unsubscribeFromKafkaTopic(argument: _ficus_GrpcUnsubscribeFromKafkaRequest_DONTUSE, callback: grpc.requestCallback<_ficus_GrpcKafkaResult>): grpc.ClientUnaryCall;
  
}

export interface GrpcKafkaServiceHandlers extends grpc.UntypedServiceImplementation {
  AddPipelineToSubscription: grpc.handleUnaryCall<_ficus_GrpcAddPipelineRequest, _ficus_GrpcKafkaResult_DONTUSE>;
  
  AddPipelineToSubscriptionStream: grpc.handleServerStreamingCall<_ficus_GrpcAddPipelineStreamRequest, _ficus_GrpcPipelinePartExecutionResult_DONTUSE>;
  
  ExecutePipelineAndProduceToKafka: grpc.handleServerStreamingCall<_ficus_GrpcExecutePipelineAndProduceKafkaRequest, _ficus_GrpcPipelinePartExecutionResult_DONTUSE>;
  
  GetAllSubscriptionsAndPipelines: grpc.handleUnaryCall<_google_protobuf_Empty, _ficus_GrpcGetAllSubscriptionsAndPipelinesResponse_DONTUSE>;
  
  RemoveAllPipelineSubscriptions: grpc.handleUnaryCall<_ficus_GrpcRemoveAllPipelinesRequest, _ficus_GrpcKafkaResult_DONTUSE>;
  
  RemovePipelineSubscription: grpc.handleUnaryCall<_ficus_GrpcRemovePipelineRequest, _ficus_GrpcKafkaResult_DONTUSE>;
  
  SubscribeForKafkaTopic: grpc.handleUnaryCall<_ficus_GrpcSubscribeToKafkaRequest, _ficus_GrpcKafkaResult_DONTUSE>;
  
  UnsubscribeFromKafkaTopic: grpc.handleUnaryCall<_ficus_GrpcUnsubscribeFromKafkaRequest, _ficus_GrpcKafkaResult_DONTUSE>;
  
}

export interface GrpcKafkaServiceDefinition extends grpc.ServiceDefinition {
  AddPipelineToSubscription: MethodDefinition<_ficus_GrpcAddPipelineRequest_DONTUSE, _ficus_GrpcKafkaResult_DONTUSE, _ficus_GrpcAddPipelineRequest, _ficus_GrpcKafkaResult>
  AddPipelineToSubscriptionStream: MethodDefinition<_ficus_GrpcAddPipelineStreamRequest_DONTUSE, _ficus_GrpcPipelinePartExecutionResult_DONTUSE, _ficus_GrpcAddPipelineStreamRequest, _ficus_GrpcPipelinePartExecutionResult>
  ExecutePipelineAndProduceToKafka: MethodDefinition<_ficus_GrpcExecutePipelineAndProduceKafkaRequest_DONTUSE, _ficus_GrpcPipelinePartExecutionResult_DONTUSE, _ficus_GrpcExecutePipelineAndProduceKafkaRequest, _ficus_GrpcPipelinePartExecutionResult>
  GetAllSubscriptionsAndPipelines: MethodDefinition<_google_protobuf_Empty_DONTUSE, _ficus_GrpcGetAllSubscriptionsAndPipelinesResponse_DONTUSE, _google_protobuf_Empty, _ficus_GrpcGetAllSubscriptionsAndPipelinesResponse>
  RemoveAllPipelineSubscriptions: MethodDefinition<_ficus_GrpcRemoveAllPipelinesRequest_DONTUSE, _ficus_GrpcKafkaResult_DONTUSE, _ficus_GrpcRemoveAllPipelinesRequest, _ficus_GrpcKafkaResult>
  RemovePipelineSubscription: MethodDefinition<_ficus_GrpcRemovePipelineRequest_DONTUSE, _ficus_GrpcKafkaResult_DONTUSE, _ficus_GrpcRemovePipelineRequest, _ficus_GrpcKafkaResult>
  SubscribeForKafkaTopic: MethodDefinition<_ficus_GrpcSubscribeToKafkaRequest_DONTUSE, _ficus_GrpcKafkaResult_DONTUSE, _ficus_GrpcSubscribeToKafkaRequest, _ficus_GrpcKafkaResult>
  UnsubscribeFromKafkaTopic: MethodDefinition<_ficus_GrpcUnsubscribeFromKafkaRequest_DONTUSE, _ficus_GrpcKafkaResult_DONTUSE, _ficus_GrpcUnsubscribeFromKafkaRequest, _ficus_GrpcKafkaResult>
}
