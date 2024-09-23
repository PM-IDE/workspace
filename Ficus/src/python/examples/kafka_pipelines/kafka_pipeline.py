from ficus import *
import os

consumer_servers = os.getenv('CONSUMER_BOOTSTRAP_SERVERS')
consumer_topic = os.getenv('CONSUMER_TOPIC')
consumer_group = os.getenv('CONSUMER_GROUP_ID')

kafka_consumer_metadata = KafkaPipelineMetadata(
    topic_name=consumer_topic,
    kafka_consumer_configuration={
        'bootstrap.servers': consumer_servers,
        'group.id': consumer_group,
        'auto.offset.reset': 'earliest'
    }
)

producer_servers = os.getenv('PRODUCER_BOOTSTRAP_SERVERS')
producer_topic = os.getenv('PRODUCER_TOPIC')

kafka_producer_metadata = KafkaPipelineMetadata(
    topic_name=producer_topic,
    kafka_consumer_configuration={
        'bootstrap.servers': producer_servers
    }
)

ficus_backend = os.getenv('FICUS_BACKEND')

KafkaPipeline(
    PrintEventLogInfo(),
    TracesDiversityDiagramCanvas(),
).execute(kafka_consumer_metadata, kafka_producer_metadata, {
    ficus_backend_addr_key: ficus_backend
})
