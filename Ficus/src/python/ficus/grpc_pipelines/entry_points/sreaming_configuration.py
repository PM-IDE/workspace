from ..models.kafka_service_pb2 import *
import google.protobuf.empty_pb2

def create_time_caching_configuration(trace_timeout_ms: int):
  return GrpcPipelineStreamingConfiguration(
    t1Configuration=GrpcT1StreamingConfiguration(
      timeBasedConfiguration=GrpcT1TimeBasedCachingConfiguration(
        tracesTimeoutMs=trace_timeout_ms
      )
    )
  )

def create_not_specified_configuration():
  return GrpcPipelineStreamingConfiguration(
    notSpecified=google.protobuf.empty_pb2.Empty()
  )
