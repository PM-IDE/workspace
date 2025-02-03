from ..models.kafka_service_pb2 import *
from google.protobuf.empty_pb2 import Empty

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

def create_not_specified_configuration() -> GrpcPipelineStreamingConfiguration:
    return GrpcPipelineStreamingConfiguration(
      notSpecified=Empty()
    )

def create_lossy_count_configuration(error: float, support: float) -> GrpcPipelineStreamingConfiguration:
    return GrpcPipelineStreamingConfiguration(
      t2Configuration=GrpcT2StreamingConfiguration(
        lossyCount=GrpcT2LossyCountConfiguration(
          error=error,
          support=support
        )
      )
    )

def create_timed_sliding_window_configuration(lifetime_ms: int) -> GrpcPipelineStreamingConfiguration:
    return GrpcPipelineStreamingConfiguration(
      t2Configuration=GrpcT2StreamingConfiguration(
        timedSlidingWindow=GrpcT2TimedSlidingWindowConfiguration(
          lifespanMs=lifetime_ms
        )
      )
    )
