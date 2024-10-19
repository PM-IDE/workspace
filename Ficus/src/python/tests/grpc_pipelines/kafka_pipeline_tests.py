from ficus import ficus_backend_addr_key
from ...ficus.grpc_pipelines.entry_points.default_pipeline import PrintEventLogInfo
from ...ficus.grpc_pipelines.entry_points.kafka import KafkaPipeline, KafkaPipelineMetadata


def test_kafka_pipeline():
    kafka_metadata = KafkaPipelineMetadata(
        topic_name="my-topic",
        kafka_consumer_configuration={
            'bootstrap.servers': 'kafka:29092',
            'group.id': 'xd',
            'auto.offset.reset': 'earliest'
        }
    )

    kafka_producer_metadata = KafkaPipelineMetadata(
        topic_name='ficus-topic',
        kafka_consumer_configuration={
            'bootstrap.servers': 'kafka:29092',
        }
    )

    KafkaPipeline(
        PrintEventLogInfo()
    ).execute(kafka_metadata, kafka_producer_metadata, {
        ficus_backend_addr_key: 'localhost:1234'
    })


def test_kafka_stream_pipeline():
    kafka_metadata = KafkaPipelineMetadata(
        topic_name="my-topic",
        kafka_consumer_configuration={
            'bootstrap.servers': 'localhost:9092',
            'group.id': 'xd',
            'auto.offset.reset': 'earliest'
        }
    )

    KafkaPipeline(
        PrintEventLogInfo()
    ).execute_stream(kafka_metadata, {})