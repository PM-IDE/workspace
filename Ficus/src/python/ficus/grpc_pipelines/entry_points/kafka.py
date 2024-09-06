from dataclasses import dataclass
from typing import Optional

from .util import *
from ..models.kafka_service_pb2 import GrpcSubscribeForKafkaTopicRequest, GrpcKafkaConsumerMetadata
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

    def execute(self, kafka_metadata: KafkaPipelineMetadata, initial_context: dict[str, ContextValue]):
        with create_ficus_grpc_channel(initial_context) as channel:
            stub = GrpcKafkaServiceStub(channel)

            request = self._create_subscribe_to_kafka_request(kafka_metadata, initial_context)
            response = stub.SubscribeForKafkaTopic(request)
            if response.HasField('success'):
                self.consumer_uuid = response.success.subscriptionId.guid
                print(f'Consumer id: {self.consumer_uuid}')
            else:
                print(response.failure.errorMessage)

    def _create_subscribe_to_kafka_request(self,
                                           kafka_metadata: KafkaPipelineMetadata,
                                           initial_context: dict[str, ContextValue]) -> GrpcSubscribeForKafkaTopicRequest:
        metadata = list(map(
            lambda x: GrpcKafkaConsumerMetadata(key=x[0], value=x[1]),
            list(kafka_metadata.kafka_consumer_configuration.items())
        ))

        pipeline_request = GrpcPipelineExecutionRequest(
            pipeline=create_grpc_pipeline(self.parts),
            initialContext=create_initial_context(initial_context)
        )

        return GrpcSubscribeForKafkaTopicRequest(
            topicName=kafka_metadata.topic_name,
            metadata=metadata,
            pipelineRequest=pipeline_request
        )

    def execute_stream(self, kafka_metadata: KafkaPipelineMetadata, initial_context: dict[str, ContextValue]):
        with create_ficus_grpc_channel(initial_context) as channel:
            stub = GrpcKafkaServiceStub(channel)
            callback_parts = []
            append_parts_with_callbacks(list(self.parts), callback_parts)

            request = self._create_subscribe_to_kafka_request(kafka_metadata, initial_context)
            process_multiple_pipelines_output_stream(callback_parts, stub.SubscribeForKafkaTopicStream(request))
