from dataclasses import dataclass
from typing import Optional

from .util import *
from ..models.kafka_service_pb2 import *
from ..models.kafka_service_pb2_grpc import *
from ..models.util_pb2 import GrpcGuid
from ...grpc_pipelines.context_values import ContextValue
from ...grpc_pipelines.entry_points.default_pipeline import create_grpc_pipeline
from ...grpc_pipelines.models.backend_service_pb2 import *


@dataclass
class KafkaPipelineMetadata:
    topic_name: str
    kafka_consumer_configuration: dict[str, str]


class KafkaPipeline:
    def __init__(self, *parts):
        self.parts: list['PipelinePart'] = list(parts)
        self.pipeline_id = None

    def execute(self,
                ficus_backend: str,
                subscription_id: str,
                producer_connection_metadata: KafkaPipelineMetadata,
                initial_context: dict[str, ContextValue]):
        with create_ficus_grpc_channel(ficus_backend) as channel:
            stub = GrpcKafkaServiceStub(channel)

            request = self._create_pipeline_execution_request(initial_context)
            request = GrpcAddPipelineRequest(
                subscriptionId=GrpcGuid(guid=subscription_id),
                pipelineRequest=request,
                resultsToKafkaTopic=_create_kafka_connection_metadata(producer_connection_metadata)
            )

            response = stub.AddPipelineToSubscription(request)
            if response.HasField('success'):
                self.pipeline_id = response.success.id.guid
                print(f'Pipeline id: {self.pipeline_id}')
            else:
                print(response.failure.errorMessage)


    def _create_pipeline_execution_request(self,
                                           initial_context: dict[str, ContextValue]) -> GrpcPipelineExecutionRequest:

        return GrpcPipelineExecutionRequest(
            pipeline=create_grpc_pipeline(self.parts),
            initialContext=create_initial_context(initial_context)
        )


    def execute_stream(self, ficus_backend: str, subscription_id: str, initial_context: dict[str, ContextValue]):
        with create_ficus_grpc_channel(ficus_backend) as channel:
            stub = GrpcKafkaServiceStub(channel)
            callback_parts = []
            append_parts_with_callbacks(list(self.parts), callback_parts)

            request = self._create_pipeline_execution_request(initial_context)
            request = GrpcAddPipelineStreamRequest(
                pipelineRequest=request,
                subscriptionId=GrpcGuid(guid=subscription_id)
            )

            process_multiple_pipelines_output_stream(callback_parts, stub.AddPipelineToSubscriptionStream(request))


    def execute_and_send_to_kafka(self,
                                  ficus_backend: str,
                                  process_name: str,
                                  case_name: str,
                                  producer_metadata: KafkaPipelineMetadata,
                                  initial_context: dict[str, ContextValue]):
        with create_ficus_grpc_channel(ficus_backend) as channel:
            def action(ids: list[GrpcGuid]):
                stub = GrpcKafkaServiceStub(channel)

                pipeline_request = GrpcProxyPipelineExecutionRequest(
                    pipeline=create_grpc_pipeline(self.parts),
                    contextValuesIds=ids,
                )

                request = GrpcExecutePipelineAndProduceKafkaRequest(
                    pipelineRequest=pipeline_request,
                    producerMetadata=_create_kafka_connection_metadata(producer_metadata),
                    caseInfo=GrpcProcessInfo(caseName=case_name, processName=process_name),
                )

                callback_parts = []
                append_parts_with_callbacks(list(self.parts), callback_parts)

                process_pipeline_output_stream(callback_parts, stub.ExecutePipelineAndProduceToKafka(request))

            execute_with_context_values(channel, initial_context, action)


def _create_kafka_connection_metadata(kafka_metadata: KafkaPipelineMetadata) -> GrpcKafkaConnectionMetadata:
    metadata = list(map(
        lambda x: GrpcKafkaConsumerMetadata(key=x[0], value=x[1]),
        list(kafka_metadata.kafka_consumer_configuration.items())
    ))

    return GrpcKafkaConnectionMetadata(
        topicName=kafka_metadata.topic_name,
        metadata=metadata
    )


def create_kafka_subscription(kafka_connection_metadata: KafkaPipelineMetadata, ficus_addr: str) -> Optional[str]:
    with create_ficus_grpc_channel(ficus_addr) as channel:
        stub = GrpcKafkaServiceStub(channel)

        metadata = _create_kafka_connection_metadata(kafka_connection_metadata)
        response = stub.SubscribeForKafkaTopic(GrpcSubscribeToKafkaRequest(connectionMetadata=metadata))

        if response.HasField('success'):
            id = response.success.id.guid
            print(f'Created kafka subscription with id {id}')
            return id
        else:
            print(f'Failed to create kafka subscription')
            return None


def remove_kafka_subscription(subscription_id: str, ficus_addr: str):
    with create_ficus_grpc_channel(ficus_addr) as channel:
        stub = GrpcKafkaServiceStub(channel)

        request = GrpcUnsubscribeFromKafkaRequest(subscriptionId=GrpcGuid(guid=subscription_id))
        response = stub.UnsubscribeFromKafkaTopic(request)

        if response.HasField('success'):
            print(f'Unsubscribed from kafka subscription {subscription_id}')
        else:
            print(f'Failed to unsubscribe from kafka subscription {subscription_id}')
