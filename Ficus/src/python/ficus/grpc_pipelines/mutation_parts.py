from .constants import *
from .entry_points.default_pipeline import PipelinePart, create_default_pipeline_part, append_strings_context_value
from .models.pipelines_and_context_pb2 import *


class AddStartEndArtificialEvents(PipelinePart):
    def to_grpc_part(self) -> GrpcPipelinePartBase:
        config = GrpcPipelinePartConfiguration()
        return GrpcPipelinePartBase(defaultPart=create_default_pipeline_part(const_add_start_end_artificial_events, config))


class AddStartArtificialEvents(PipelinePart):
    def to_grpc_part(self) -> GrpcPipelinePartBase:
        config = GrpcPipelinePartConfiguration()
        return GrpcPipelinePartBase(defaultPart=create_default_pipeline_part(const_add_start_artificial_events, config))


class AddEndArtificialEvents(PipelinePart):
    def to_grpc_part(self) -> GrpcPipelinePartBase:
        config = GrpcPipelinePartConfiguration()
        return GrpcPipelinePartBase(defaultPart=create_default_pipeline_part(const_add_end_artificial_events, config))


class AppendAttributesToName(PipelinePart):
    def __init__(self, attributes: list[str]):
        super().__init__()
        self.attributes = attributes

    def to_grpc_part(self) -> GrpcPipelinePartBase:
        config = GrpcPipelinePartConfiguration()
        append_strings_context_value(config, const_attributes, self.attributes)

        return GrpcPipelinePartBase(defaultPart=create_default_pipeline_part(const_append_attributes_to_name, config))
