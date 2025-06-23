from ficus import *
import os
import time

def env_or_default(env_name: str, default: str):
    env = os.getenv(env_name)
    return env if env is not None else default

def execute_pipeline(sub_name: str, pipeline_name: str, pipeline_parts: list[PipelinePart], trace_filtering_pipeline = Pipeline()):
    consumer_servers = env_or_default('CONSUMER_BOOTSTRAP_SERVERS', 'localhost:9092')
    consumer_topic = env_or_default('CONSUMER_TOPIC', 'my-topic')
    consumer_group = env_or_default('CONSUMER_GROUP_ID', 'xd')

    kafka_consumer_metadata = KafkaPipelineMetadata(
        topic_name=consumer_topic,
        kafka_consumer_configuration={
            'bootstrap.servers': consumer_servers,
            'group.id': consumer_group,
            'auto.offset.reset': 'earliest',
            'message.max.bytes': '25728640',
        }
    )

    producer_servers = env_or_default('PRODUCER_BOOTSTRAP_SERVERS', 'localhost:9092')
    producer_topic = env_or_default('PRODUCER_TOPIC', 'ficus-topic')

    kafka_producer_metadata = KafkaPipelineMetadata(
        topic_name=producer_topic,
        kafka_consumer_configuration={
            'bootstrap.servers': producer_servers,
            'message.max.bytes': '25728640',
        }
    )

    ficus_backend = env_or_default('FICUS_BACKEND', 'localhost:8080')

    subscription_id = create_kafka_subscription(sub_name, kafka_consumer_metadata, ficus_backend)
    if subscription_id is None:
        return

    with open(os.path.join(os.path.abspath(os.curdir), 'software_data_config.json'), "r") as f:
        software_data_config = f.read()

    KafkaPipeline(
        pipeline_parts
    ).execute(ficus_backend, 
              subscription_id, 
              pipeline_name, 
              kafka_producer_metadata, 
              initial_context={
                'software_data_extraction_config': JsonContextValue(software_data_config)
              },
              streaming_configuration=create_queue_traces_configuration(3))

    if env_or_default('SLEEP', None) is not None:
        while True:
            time.sleep(10 ** 8)
