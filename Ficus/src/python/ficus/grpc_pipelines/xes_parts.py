from .constants import *
from .context_values import from_grpc_bytes, write_file_bytes
from .entry_points.default_pipeline import PipelinePart, _create_default_pipeline_part, append_string_value, PipelinePartWithCallback, \
    _create_complex_get_context_part
from .models.pipelines_and_context_pb2 import GrpcPipelinePartBase, GrpcPipelinePartConfiguration, GrpcContextValue


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


class WriteBytesToFilePipelinePartBase(PipelinePartWithCallback):
    def __init__(self, save_path: str, before_pipeline_part: str):
        super().__init__()
        self.save_path = save_path
        self.before_pipeline_part = before_pipeline_part

    def to_grpc_part(self) -> GrpcPipelinePartBase:
        pipeline_part = _create_complex_get_context_part(self.uuid,
                                                         [const_bytes],
                                                         self.before_pipeline_part,
                                                         GrpcPipelinePartConfiguration())

        return GrpcPipelinePartBase(complexContextRequestPart=pipeline_part)

    def execute_callback(self, values: dict[str, GrpcContextValue]):
        file_bytes = from_grpc_bytes(values[const_bytes].bytes)
        write_file_bytes(self.save_path, file_bytes.bytes)


class WriteLogToBxesBytes(WriteBytesToFilePipelinePartBase):
    def __init__(self, save_path: str):
        super().__init__(save_path, const_write_bxes_to_bytes)


class WriteLogToXesBytes(WriteBytesToFilePipelinePartBase):
    def __init__(self, save_path: str):
        super().__init__(save_path, const_write_xes_to_bytes)
