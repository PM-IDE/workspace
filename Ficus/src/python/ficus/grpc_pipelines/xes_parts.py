from .constants import *
from .grpc_pipelines import PipelinePart2, _create_default_pipeline_part, append_string_value
from .models.pipelines_and_context_pb2 import GrpcPipelinePartBase, GrpcPipelinePartConfiguration


class ReadLogFromXes2(PipelinePart2):
    def to_grpc_part(self) -> GrpcPipelinePartBase:
        return GrpcPipelinePartBase(defaultPart=_create_default_pipeline_part(const_read_log_from_xes))


class WriteLogToXes2(PipelinePart2):
    def __init__(self, save_path: str):
        super().__init__()
        self.save_path = save_path

    def to_grpc_part(self) -> GrpcPipelinePartBase:
        config = GrpcPipelinePartConfiguration()
        append_string_value(config, const_path, self.save_path)
        return GrpcPipelinePartBase(defaultPart=_create_default_pipeline_part(const_write_log_to_xes, config))


class ReadLogFromBxes2(PipelinePart2):
    def to_grpc_part(self) -> GrpcPipelinePartBase:
        return GrpcPipelinePartBase(defaultPart=_create_default_pipeline_part(const_read_log_from_bxes))


class WriteLogToBxes2(PipelinePart2):
    def __init__(self, save_path: str):
        super().__init__()
        self.save_path = save_path

    def to_grpc_part(self) -> GrpcPipelinePartBase:
        config = GrpcPipelinePartConfiguration()
        append_string_value(config, const_path, self.save_path)
        return GrpcPipelinePartBase(defaultPart=_create_default_pipeline_part(const_write_log_to_bxes, config))
