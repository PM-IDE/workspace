from ..models.kafka_service_pb2 import *
from google.protobuf.empty_pb2 import Empty

from .default_pipeline import Pipeline


def create_events_timeout_configuration(events_timeout_ms: int) -> GrpcPipelineStreamingConfiguration:
    return GrpcPipelineStreamingConfiguration(
      t1Configuration=GrpcT1StreamingConfiguration(
        eventsTimeout=GrpcT1EventsTimeBasedCaching(
          eventsTimeoutMs=events_timeout_ms
        )
      )
    )

def create_traces_timeout_configuration(traces_timeout_ms: int) -> GrpcPipelineStreamingConfiguration:
    return GrpcPipelineStreamingConfiguration(
      t1Configuration=GrpcT1StreamingConfiguration(
        tracesTimeout=GrpcT1TraceTimeBasedCaching(
          tracesTimeoutMs=traces_timeout_ms
        )
      )
    )

def create_queue_traces_configuration(queue_capacity: int) -> GrpcPipelineStreamingConfiguration:
    return GrpcPipelineStreamingConfiguration(
      t1Configuration=GrpcT1StreamingConfiguration(
        tracesQueueConfiguration=GrpcT1TracesQueueConfiguration(
          queueCapacity=queue_capacity
        )
      )
    )

def create_not_specified_configuration() -> GrpcPipelineStreamingConfiguration:
    return GrpcPipelineStreamingConfiguration(
      notSpecified=Empty()
    )

def create_lossy_count_configuration(error: float, support: float, trace_preprocessing_pipeline: Pipeline = Pipeline()) -> GrpcPipelineStreamingConfiguration:
    return GrpcPipelineStreamingConfiguration(
      t2Configuration=GrpcT2StreamingConfiguration(
        lossyCount=GrpcT2LossyCountConfiguration(
          error=error,
          support=support
        ),
        incomingTracesFilteringPipeline=trace_preprocessing_pipeline.to_grpc_pipeline()
      )
    )

def create_timed_sliding_window_configuration(lifetime_ms: int, trace_preprocessing_pipeline: Pipeline = Pipeline()) -> GrpcPipelineStreamingConfiguration:
    return GrpcPipelineStreamingConfiguration(
      t2Configuration=GrpcT2StreamingConfiguration(
        timedSlidingWindow=GrpcT2TimedSlidingWindowConfiguration(
          lifespanMs=lifetime_ms
        ),
        incomingTracesFilteringPipeline=trace_preprocessing_pipeline.to_grpc_pipeline()
      )
    )