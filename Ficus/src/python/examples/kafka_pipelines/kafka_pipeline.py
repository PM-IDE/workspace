from ficus import *
import os

def env_or_default(env_name: str, default: str):
    env = os.getenv(env_name)
    return env if env is not None else default

consumer_servers = env_or_default('CONSUMER_BOOTSTRAP_SERVERS', 'localhost:9092')
consumer_topic = env_or_default('CONSUMER_TOPIC', 'my-topic')
consumer_group = env_or_default('CONSUMER_GROUP_ID', 'xd')

kafka_consumer_metadata = KafkaPipelineMetadata(
    topic_name=consumer_topic,
    kafka_consumer_configuration={
        'bootstrap.servers': consumer_servers,
        'group.id': consumer_group,
        'auto.offset.reset': 'earliest'
    }
)

producer_servers = env_or_default('PRODUCER_BOOTSTRAP_SERVERS', 'localhost:9092')
producer_topic = env_or_default('PRODUCER_TOPIC', 'ficus-topic')

kafka_producer_metadata = KafkaPipelineMetadata(
    topic_name=producer_topic,
    kafka_consumer_configuration={
        'bootstrap.servers': producer_servers
    }
)

ficus_backend = env_or_default('FICUS_BACKEND', 'localhost:8080')

KafkaPipeline(
    PrintEventLogInfo(),
    TracesDiversityDiagramCanvas(),
    DiscoverPetriNetHeuristic(),
    EnsureInitialMarking(),
    AnnotatePetriNetWithFrequency(),
    DiscoverFuzzyGraph(),
    ViewGraph()
).execute(kafka_consumer_metadata, kafka_producer_metadata, {
    ficus_backend_addr_key: ficus_backend
})
