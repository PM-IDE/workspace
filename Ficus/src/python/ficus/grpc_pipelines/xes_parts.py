from .constants import *
from .grpc_pipelines import PipelinePart, _create_default_pipeline_part, append_string_value
from .models.pipelines_and_context_pb2 import GrpcPipelinePartBase, GrpcPipelinePartConfiguration


class ReadLogFromXes(PipelinePart):
    def __init__(self, use_bytes: bool = False):
        super().__init__()
        self.use_bytes: bool = use_bytes

    def to_grpc_part(self) -> GrpcPipelinePartBase:
        if self.use_bytes:
            return GrpcPipelinePartBase(defaultPart=_create_default_pipeline_part(const_read_xes_from_bytes))

        return GrpcPipelinePartBase(defaultPart=_create_default_pipeline_part(const_read_log_from_xes))


class WriteLogToXes(PipelinePart):
    def __init__(self, save_path: str):
        super().__init__()
        self.save_path = save_path

    def to_grpc_part(self) -> GrpcPipelinePartBase:
        config = GrpcPipelinePartConfiguration()
        append_string_value(config, const_path, self.save_path)
        return GrpcPipelinePartBase(defaultPart=_create_default_pipeline_part(const_write_log_to_xes, config))


class ReadLogFromBxes(PipelinePart):
    def __init__(self, use_bytes: bool = False):
        super().__init__()
        self.use_bytes: bool = use_bytes

    def to_grpc_part(self) -> GrpcPipelinePartBase:
        if self.use_bytes:
            return GrpcPipelinePartBase(defaultPart=_create_default_pipeline_part(const_read_bxes_from_bytes))

        return GrpcPipelinePartBase(defaultPart=_create_default_pipeline_part(const_read_log_from_bxes))


class WriteLogToBxes(PipelinePart):
    def __init__(self, save_path: str):
        super().__init__()
        self.save_path = save_path

    def to_grpc_part(self) -> GrpcPipelinePartBase:
        config = GrpcPipelinePartConfiguration()
        append_string_value(config, const_path, self.save_path)
        return GrpcPipelinePartBase(defaultPart=_create_default_pipeline_part(const_write_log_to_bxes, config))
