from ficus import KafkaPipeline, PrintEventLogInfo, KafkaPipelineMetadata, ficus_backend_addr_key


def test_kafka_pipeline():
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
    ).execute(kafka_metadata, {})


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