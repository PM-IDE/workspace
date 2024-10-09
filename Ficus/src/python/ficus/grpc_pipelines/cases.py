from .entry_points.default_pipeline import *

class DiscoverCases(PipelinePart):
    def __init__(self, start_regex: str, end_regex: str, pipeline: Pipeline):
        super().__init__()
        self.start_regex = start_regex
        self.end_regex = end_regex
        self.pipeline = pipeline

    def to_grpc_part(self) -> GrpcPipelinePartBase:
        config = GrpcPipelinePartConfiguration()
        append_string_value(config, const_start_case_regex, self.start_regex)
        append_string_value(config, const_end_case_regex, self.end_regex)
        append_pipeline_value(config, const_pipeline, self.pipeline)

        return GrpcPipelinePartBase(defaultPart=create_default_pipeline_part(const_discover_cases, config))

    def append_parts_with_callbacks(self, parts: list['PipelinePartWithCallback']):
        super().append_parts_with_callbacks(parts)
        append_parts_with_callbacks(self.pipeline.parts, parts)
