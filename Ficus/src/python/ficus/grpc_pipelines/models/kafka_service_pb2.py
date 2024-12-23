# -*- coding: utf-8 -*-

# Generated by the protocol buffer compiler.  DO NOT EDIT!

# source: kafka_service.proto

"""Generated protocol buffer code."""

from google.protobuf import descriptor as _descriptor

from google.protobuf import descriptor_pool as _descriptor_pool

from google.protobuf import symbol_database as _symbol_database

from google.protobuf.internal import builder as _builder

# @@protoc_insertion_point(imports)



_sym_db = _symbol_database.Default()





import ficus.grpc_pipelines.models.pipelines_and_context_pb2 as pipelines__and__context__pb2

import ficus.grpc_pipelines.models.util_pb2 as util__pb2

import ficus.grpc_pipelines.models.backend_service_pb2 as backend__service__pb2

from google.protobuf import empty_pb2 as google_dot_protobuf_dot_empty__pb2





DESCRIPTOR = _descriptor_pool.Default().AddSerializedFile(b'\n\x13kafka_service.proto\x12\x05\x66icus\x1a\x1bpipelines_and_context.proto\x1a\nutil.proto\x1a\x15\x62\x61\x63kend_service.proto\x1a\x1bgoogle/protobuf/empty.proto\"\xd6\x01\n)GrpcExecutePipelineAndProduceKafkaRequest\x12\x41\n\x0fpipelineRequest\x18\x01 \x01(\x0b\x32(.ficus.GrpcProxyPipelineExecutionRequest\x12<\n\x10producerMetadata\x18\x02 \x01(\x0b\x32\".ficus.GrpcKafkaConnectionMetadata\x12(\n\x08\x63\x61seInfo\x18\x03 \x01(\x0b\x32\x16.ficus.GrpcProcessInfo\"8\n\x0fGrpcProcessInfo\x12\x13\n\x0bprocessName\x18\x01 \x01(\t\x12\x10\n\x08\x63\x61seName\x18\x02 \x01(\t\"\xa1\x01\n\x1bGrpcSubscribeToKafkaRequest\x12>\n\x12\x63onnectionMetadata\x18\x01 \x01(\x0b\x32\".ficus.GrpcKafkaConnectionMetadata\x12\x42\n\x14subscriptionMetadata\x18\x02 \x01(\x0b\x32$.ficus.GrpcKafkaSubscriptionMetadata\"9\n\x1dGrpcKafkaSubscriptionMetadata\x12\x18\n\x10subscriptionName\x18\x01 \x01(\t\"\xc1\x01\n!GrpcKafkaPipelineExecutionRequest\x12\'\n\x0esubscriptionId\x18\x01 \x01(\x0b\x32\x0f.ficus.GrpcGuid\x12<\n\x0fpipelineRequest\x18\x02 \x01(\x0b\x32#.ficus.GrpcPipelineExecutionRequest\x12\x35\n\x10pipelineMetadata\x18\x03 \x01(\x0b\x32\x1b.ficus.GrpcPipelineMetadata\"$\n\x14GrpcPipelineMetadata\x12\x0c\n\x04name\x18\x01 \x01(\t\"\x9e\x01\n\x16GrpcAddPipelineRequest\x12\x41\n\x0fpipelineRequest\x18\x01 \x01(\x0b\x32(.ficus.GrpcKafkaPipelineExecutionRequest\x12\x41\n\x15producerKafkaMetadata\x18\x02 \x01(\x0b\x32\".ficus.GrpcKafkaConnectionMetadata\"a\n\x1cGrpcAddPipelineStreamRequest\x12\x41\n\x0fpipelineRequest\x18\x01 \x01(\x0b\x32(.ficus.GrpcKafkaPipelineExecutionRequest\"i\n\x19GrpcRemovePipelineRequest\x12\'\n\x0esubscriptionId\x18\x01 \x01(\x0b\x32\x0f.ficus.GrpcGuid\x12#\n\npipelineId\x18\x02 \x01(\x0b\x32\x0f.ficus.GrpcGuid\"H\n\x1dGrpcRemoveAllPipelinesRequest\x12\'\n\x0esubscriptionId\x18\x01 \x01(\x0b\x32\x0f.ficus.GrpcGuid\"\\\n\x1bGrpcKafkaConnectionMetadata\x12\x11\n\ttopicName\x18\x01 \x01(\t\x12*\n\x08metadata\x18\x02 \x03(\x0b\x32\x18.ficus.GrpcKafkaMetadata\"/\n\x11GrpcKafkaMetadata\x12\x0b\n\x03key\x18\x01 \x01(\t\x12\r\n\x05value\x18\x02 \x01(\t\"~\n\x0fGrpcKafkaResult\x12\x30\n\x07success\x18\x01 \x01(\x0b\x32\x1d.ficus.GrpcKafkaSuccessResultH\x00\x12/\n\x07\x66\x61ilure\x18\x02 \x01(\x0b\x32\x1c.ficus.GrpcKafkaFailedResultH\x00\x42\x08\n\x06result\"5\n\x16GrpcKafkaSuccessResult\x12\x1b\n\x02id\x18\x01 \x01(\x0b\x32\x0f.ficus.GrpcGuid\"-\n\x15GrpcKafkaFailedResult\x12\x14\n\x0c\x65rrorMessage\x18\x01 \x01(\t\"J\n\x1fGrpcUnsubscribeFromKafkaRequest\x12\'\n\x0esubscriptionId\x18\x01 \x01(\x0b\x32\x0f.ficus.GrpcGuid\"b\n+GrpcGetAllSubscriptionsAndPipelinesResponse\x12\x33\n\rsubscriptions\x18\x01 \x03(\x0b\x32\x1c.ficus.GrpcKafkaSubscription\"\xa0\x01\n\x15GrpcKafkaSubscription\x12\x1b\n\x02id\x18\x01 \x01(\x0b\x32\x0f.ficus.GrpcGuid\x12\x36\n\x08metadata\x18\x02 \x01(\x0b\x32$.ficus.GrpcKafkaSubscriptionMetadata\x12\x32\n\tpipelines\x18\x03 \x03(\x0b\x32\x1f.ficus.GrpcSubscriptionPipeline\"f\n\x18GrpcSubscriptionPipeline\x12\x1b\n\x02id\x18\x01 \x01(\x0b\x32\x0f.ficus.GrpcGuid\x12-\n\x08metadata\x18\x02 \x01(\x0b\x32\x1b.ficus.GrpcPipelineMetadata2\xb2\x06\n\x10GrpcKafkaService\x12T\n\x16SubscribeForKafkaTopic\x12\".ficus.GrpcSubscribeToKafkaRequest\x1a\x16.ficus.GrpcKafkaResult\x12[\n\x19UnsubscribeFromKafkaTopic\x12&.ficus.GrpcUnsubscribeFromKafkaRequest\x1a\x16.ficus.GrpcKafkaResult\x12R\n\x19\x41\x64\x64PipelineToSubscription\x12\x1d.ficus.GrpcAddPipelineRequest\x1a\x16.ficus.GrpcKafkaResult\x12p\n\x1f\x41\x64\x64PipelineToSubscriptionStream\x12#.ficus.GrpcAddPipelineStreamRequest\x1a&.ficus.GrpcPipelinePartExecutionResult0\x01\x12V\n\x1aRemovePipelineSubscription\x12 .ficus.GrpcRemovePipelineRequest\x1a\x16.ficus.GrpcKafkaResult\x12^\n\x1eRemoveAllPipelineSubscriptions\x12$.ficus.GrpcRemoveAllPipelinesRequest\x1a\x16.ficus.GrpcKafkaResult\x12m\n\x1fGetAllSubscriptionsAndPipelines\x12\x16.google.protobuf.Empty\x1a\x32.ficus.GrpcGetAllSubscriptionsAndPipelinesResponse\x12~\n ExecutePipelineAndProduceToKafka\x12\x30.ficus.GrpcExecutePipelineAndProduceKafkaRequest\x1a&.ficus.GrpcPipelinePartExecutionResult0\x01\x62\x06proto3')



_globals = globals()

_builder.BuildMessageAndEnumDescriptors(DESCRIPTOR, _globals)

_builder.BuildTopDescriptorsAndMessages(DESCRIPTOR, 'kafka_service_pb2', _globals)

if _descriptor._USE_C_DESCRIPTORS == False:

  DESCRIPTOR._options = None

  _globals['_GRPCEXECUTEPIPELINEANDPRODUCEKAFKAREQUEST']._serialized_start=124

  _globals['_GRPCEXECUTEPIPELINEANDPRODUCEKAFKAREQUEST']._serialized_end=338

  _globals['_GRPCPROCESSINFO']._serialized_start=340

  _globals['_GRPCPROCESSINFO']._serialized_end=396

  _globals['_GRPCSUBSCRIBETOKAFKAREQUEST']._serialized_start=399

  _globals['_GRPCSUBSCRIBETOKAFKAREQUEST']._serialized_end=560

  _globals['_GRPCKAFKASUBSCRIPTIONMETADATA']._serialized_start=562

  _globals['_GRPCKAFKASUBSCRIPTIONMETADATA']._serialized_end=619

  _globals['_GRPCKAFKAPIPELINEEXECUTIONREQUEST']._serialized_start=622

  _globals['_GRPCKAFKAPIPELINEEXECUTIONREQUEST']._serialized_end=815

  _globals['_GRPCPIPELINEMETADATA']._serialized_start=817

  _globals['_GRPCPIPELINEMETADATA']._serialized_end=853

  _globals['_GRPCADDPIPELINEREQUEST']._serialized_start=856

  _globals['_GRPCADDPIPELINEREQUEST']._serialized_end=1014

  _globals['_GRPCADDPIPELINESTREAMREQUEST']._serialized_start=1016

  _globals['_GRPCADDPIPELINESTREAMREQUEST']._serialized_end=1113

  _globals['_GRPCREMOVEPIPELINEREQUEST']._serialized_start=1115

  _globals['_GRPCREMOVEPIPELINEREQUEST']._serialized_end=1220

  _globals['_GRPCREMOVEALLPIPELINESREQUEST']._serialized_start=1222

  _globals['_GRPCREMOVEALLPIPELINESREQUEST']._serialized_end=1294

  _globals['_GRPCKAFKACONNECTIONMETADATA']._serialized_start=1296

  _globals['_GRPCKAFKACONNECTIONMETADATA']._serialized_end=1388

  _globals['_GRPCKAFKAMETADATA']._serialized_start=1390

  _globals['_GRPCKAFKAMETADATA']._serialized_end=1437

  _globals['_GRPCKAFKARESULT']._serialized_start=1439

  _globals['_GRPCKAFKARESULT']._serialized_end=1565

  _globals['_GRPCKAFKASUCCESSRESULT']._serialized_start=1567

  _globals['_GRPCKAFKASUCCESSRESULT']._serialized_end=1620

  _globals['_GRPCKAFKAFAILEDRESULT']._serialized_start=1622

  _globals['_GRPCKAFKAFAILEDRESULT']._serialized_end=1667

  _globals['_GRPCUNSUBSCRIBEFROMKAFKAREQUEST']._serialized_start=1669

  _globals['_GRPCUNSUBSCRIBEFROMKAFKAREQUEST']._serialized_end=1743

  _globals['_GRPCGETALLSUBSCRIPTIONSANDPIPELINESRESPONSE']._serialized_start=1745

  _globals['_GRPCGETALLSUBSCRIPTIONSANDPIPELINESRESPONSE']._serialized_end=1843

  _globals['_GRPCKAFKASUBSCRIPTION']._serialized_start=1846

  _globals['_GRPCKAFKASUBSCRIPTION']._serialized_end=2006

  _globals['_GRPCSUBSCRIPTIONPIPELINE']._serialized_start=2008

  _globals['_GRPCSUBSCRIPTIONPIPELINE']._serialized_end=2110

  _globals['_GRPCKAFKASERVICE']._serialized_start=2113

  _globals['_GRPCKAFKASERVICE']._serialized_end=2931

# @@protoc_insertion_point(module_scope)
