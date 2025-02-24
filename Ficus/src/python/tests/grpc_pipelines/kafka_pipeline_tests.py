from ...ficus.grpc_pipelines.entry_points.default_pipeline import PrintEventLogInfo
from ...ficus.grpc_pipelines.entry_points.kafka import *


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

  ficus_backend = 'localhost:1234'
  sub_id = create_kafka_subscription('Subscription', kafka_metadata, ficus_backend)

  KafkaPipeline(
    PrintEventLogInfo()
  ).execute(ficus_backend, sub_id, 'Pipeline', kafka_producer_metadata, {})


def test_kafka_stream_pipeline():
  kafka_metadata = KafkaPipelineMetadata(
    topic_name="my-topic",
    kafka_consumer_configuration={
      'bootstrap.servers': 'localhost:9092',
      'group.id': 'xd',
      'auto.offset.reset': 'earliest'
    }
  )

  ficus_backend = 'localhost:1234'
  sub_id = create_kafka_subscription('Subscription', kafka_metadata, ficus_backend)

  KafkaPipeline(
    PrintEventLogInfo()
  ).execute_stream(ficus_backend, sub_id, 'Pipeline', {})
