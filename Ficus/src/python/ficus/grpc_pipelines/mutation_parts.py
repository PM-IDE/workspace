from .constants import *
from .grpc_pipelines import PipelinePart, _create_default_pipeline_part
from .models.pipelines_and_context_pb2 import *


class AddStartEndArtificialEvents(PipelinePart):
    def to_grpc_part(self) -> GrpcPipelinePartBase:
        config = GrpcPipelinePartConfiguration()
        return GrpcPipelinePartBase(defaultPart=_create_default_pipeline_part(const_add_start_end_artificial_events, config))


class AddStartArtificialEvents(PipelinePart):
    def to_grpc_part(self) -> GrpcPipelinePartBase:
        config = GrpcPipelinePartConfiguration()
        return GrpcPipelinePartBase(defaultPart=_create_default_pipeline_part(const_add_start_artificial_events, config))


class AddEndArtificialEvents(PipelinePart):
    def to_grpc_part(self) -> GrpcPipelinePartBase:
        config = GrpcPipelinePartConfiguration()
        return GrpcPipelinePartBase(defaultPart=_create_default_pipeline_part(const_add_end_artificial_events, config))
