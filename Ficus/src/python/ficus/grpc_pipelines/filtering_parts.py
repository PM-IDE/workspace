from .entry_points.default_pipeline import *
from .models.pipelines_and_context_pb2 import GrpcPipelinePartBase, GrpcPipelinePartConfiguration


class FilterTracesByEventsCount(PipelinePart):
  def __init__(self, min_events_in_trace: int):
    super().__init__()
    self.min_events_in_trace = min_events_in_trace

  def to_grpc_part(self) -> GrpcPipelinePartBase:
    config = GrpcPipelinePartConfiguration()
    append_uint32_value(config, const_events_count, self.min_events_in_trace)
    part = create_default_pipeline_part(const_filter_traces_by_events_count, config)
    return GrpcPipelinePartBase(defaultPart=part)


class FilterEventsByName(PipelinePart):
  def __init__(self, event_name: str):
    super().__init__()
    self.event_name = event_name

  def to_grpc_part(self) -> GrpcPipelinePartBase:
    config = GrpcPipelinePartConfiguration()
    append_string_value(config, const_event_name, self.event_name)
    part = create_default_pipeline_part(const_filter_events_by_name, config)
    return GrpcPipelinePartBase(defaultPart=part)


class RegexFilteringPartBase(PipelinePart):
  def __init__(self, regex: str, pipeline_part_name: str):
    super().__init__()
    self.regex = regex
    self.pipeline_part_name = pipeline_part_name

  def to_grpc_part(self) -> GrpcPipelinePartBase:
    config = GrpcPipelinePartConfiguration()
    append_string_value(config, const_regex, self.regex)
    part = create_default_pipeline_part(self.pipeline_part_name, config)
    return GrpcPipelinePartBase(defaultPart=part)


class FilterEventsByRegex(RegexFilteringPartBase):
  def __init__(self, regex: str):
    super().__init__(regex, const_filter_events_by_regex)


class RemainEventsByRegex(RegexFilteringPartBase):
  def __init__(self, regex: str):
    super().__init__(regex, const_remain_events_by_regex)


class FilterLogByVariants(PipelinePart):
  def to_grpc_part(self) -> GrpcPipelinePartBase:
    return GrpcPipelinePartBase(defaultPart=create_default_pipeline_part(const_filter_log_by_variants))
