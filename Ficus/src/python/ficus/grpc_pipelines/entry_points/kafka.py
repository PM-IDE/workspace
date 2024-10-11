from dataclasses import dataclass
from typing import Optional

from .util import *
from ..models.kafka_service_pb2 import *
from ..models.kafka_service_pb2_grpc import *
from ...grpc_pipelines.entry_points.default_pipeline import create_grpc_pipeline
from ...grpc_pipelines.context_values import ContextValue
from ...grpc_pipelines.models.backend_service_pb2 import *


@dataclass
class KafkaPipelineMetadata:
    topic_name: str
    kafka_consumer_configuration: dict[str, str]


class KafkaPipeline:
    def __init__(self, *parts):
        self.parts: list['PipelinePart'] = list(parts)
        self.consumer_uuid: Optional[str] = None

    def execute(self,
                consumer_connection_metadata: KafkaPipelineMetadata,
                producer_connection_metadata: KafkaPipelineMetadata,
                initial_context: dict[str, ContextValue]):
        with create_ficus_grpc_channel(initial_context) as channel:
            stub = GrpcKafkaServiceStub(channel)

            request = self._create_subscribe_to_kafka_request(consumer_connection_metadata, initial_context)
            request = GrpcSubscribeToKafkaAndProduceToKafka(
                request=request,
                producerMetadata=self._create_kafka_connection_metadata(producer_connection_metadata)
            )

            response = stub.SubscribeForKafkaTopicExternal(request)
            if response.HasField('success'):
                self.consumer_uuid = response.success.subscriptionId.guid
                print(f'Consumer id: {self.consumer_uuid}')
            else:
                print(response.failure.errorMessage)

    def _create_subscribe_to_kafka_request(self,
                                           kafka_metadata: KafkaPipelineMetadata,
                                           initial_context: dict[str, ContextValue]) -> GrpcSubscribeForKafkaTopicRequest:

        pipeline_request = GrpcPipelineExecutionRequest(
            pipeline=create_grpc_pipeline(self.parts),
            initialContext=create_initial_context(initial_context)
        )

        kafka_connection_metadata = self._create_kafka_connection_metadata(kafka_metadata)

        return GrpcSubscribeForKafkaTopicRequest(
            kafkaConnectionMetadata=kafka_connection_metadata,
            pipelineRequest=pipeline_request
        )

    @staticmethod
    def _create_kafka_connection_metadata(kafka_metadata: KafkaPipelineMetadata) -> GrpcKafkaConnectionMetadata:
        metadata = list(map(
            lambda x: GrpcKafkaConsumerMetadata(key=x[0], value=x[1]),
            list(kafka_metadata.kafka_consumer_configuration.items())
        ))

        return GrpcKafkaConnectionMetadata(
            topicName=kafka_metadata.topic_name,
            metadata=metadata
        )

    def execute_stream(self, kafka_metadata: KafkaPipelineMetadata, initial_context: dict[str, ContextValue]):
        with create_ficus_grpc_channel(initial_context) as channel:
            stub = GrpcKafkaServiceStub(channel)
            callback_parts = []
            append_parts_with_callbacks(list(self.parts), callback_parts)

            request = self._create_subscribe_to_kafka_request(kafka_metadata, initial_context)
            process_multiple_pipelines_output_stream(callback_parts, stub.SubscribeForKafkaTopicStream(request))

    def execute_and_send_to_kafka(self,
                                  case_name: str,
                                  producer_metadata: KafkaPipelineMetadata,
                                  initial_context: dict[str, ContextValue]):
        with create_ficus_grpc_channel(initial_context) as channel:
            stub = GrpcKafkaServiceStub(channel)

            pipeline_request = GrpcPipelineExecutionRequest(
                pipeline=create_grpc_pipeline(self.parts),
                initialContext=create_initial_context(initial_context)
            )

            request = GrpcExecutePipelineAndProduceKafkaRequest(
                pipelineRequest=pipeline_request,
                producerMetadata=self._create_kafka_connection_metadata(producer_metadata),
                caseInfo=GrpcCaseInfo(caseName=case_name),
            )

            result = stub.ExecutePipelineAndProduceToKafka(request)
            print(result)
