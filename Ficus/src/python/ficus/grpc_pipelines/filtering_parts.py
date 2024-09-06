from .constants import const_events_count, const_filter_traces_by_events_count, const_event_name, \
    const_filter_events_by_name, const_regex, const_filter_events_by_regex, const_filter_log_by_variants
from .entry_points.default_pipeline import PipelinePart, _create_default_pipeline_part, append_uint32_value, \
    append_string_value
from .models.pipelines_and_context_pb2 import GrpcPipelinePartBase, GrpcPipelinePartConfiguration


class FilterTracesByEventsCount(PipelinePart):
    def __init__(self, min_events_in_trace: int):
        super().__init__()
        self.min_events_in_trace = min_events_in_trace

    def to_grpc_part(self) -> GrpcPipelinePartBase:
        config = GrpcPipelinePartConfiguration()
        append_uint32_value(config, const_events_count, self.min_events_in_trace)
        part = _create_default_pipeline_part(const_filter_traces_by_events_count, config)
        return GrpcPipelinePartBase(defaultPart=part)


class FilterEventsByName(PipelinePart):
    def __init__(self, event_name: str):
        super().__init__()
        self.event_name = event_name

    def to_grpc_part(self) -> GrpcPipelinePartBase:
        config = GrpcPipelinePartConfiguration()
        append_string_value(config, const_event_name, self.event_name)
        part = _create_default_pipeline_part(const_filter_events_by_name, config)
        return GrpcPipelinePartBase(defaultPart=part)


class FilterEventsByRegex(PipelinePart):
    def __init__(self, regex: str):
        super().__init__()
        self.regex = regex

    def to_grpc_part(self) -> GrpcPipelinePartBase:
        config = GrpcPipelinePartConfiguration()
        append_string_value(config, const_regex, self.regex)
        part = _create_default_pipeline_part(const_filter_events_by_regex, config)
        return GrpcPipelinePartBase(defaultPart=part)


class FilterLogByVariants(PipelinePart):
    def to_grpc_part(self) -> GrpcPipelinePartBase:
        return GrpcPipelinePartBase(defaultPart=_create_default_pipeline_part(const_filter_log_by_variants))
