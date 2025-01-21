from ..models.kafka_service_pb2 import *
from google.protobuf.empty_pb2 import Empty

def create_time_caching_configuration(trace_timeout_ms: int) -> GrpcPipelineStreamingConfiguration:
  return GrpcPipelineStreamingConfiguration(
    t1Configuration=GrpcT1StreamingConfiguration(
      timeBasedConfiguration=GrpcT1TimeBasedCachingConfiguration(
        tracesTimeoutMs=trace_timeout_ms
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
      lossyCountConfiguration=GrpcT2LossyCountConfiguration(
        error=error,
        support=support
      )
    )
  )
