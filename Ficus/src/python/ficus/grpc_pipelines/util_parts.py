from .constants import const_use_names_event_log, const_names_event_log, const_get_names_event_log, \
    const_pipeline
from .grpc_pipelines import PipelinePart, _create_default_pipeline_part, PipelinePartWithCallback, \
    _create_complex_get_context_part, Pipeline2, append_pipeline_value, PrintEventLogInfo2
from .models.pipelines_and_context_pb2 import GrpcPipelinePartBase, GrpcPipelinePartConfiguration, \
    GrpcContextValue


class UseNamesEventLog(PipelinePart):
    def to_grpc_part(self) -> GrpcPipelinePartBase:
        return GrpcPipelinePartBase(defaultPart=_create_default_pipeline_part(const_use_names_event_log))


class PrintEventLog2(PipelinePartWithCallback):
    def to_grpc_part(self) -> GrpcPipelinePartBase:
        config = GrpcPipelinePartConfiguration()
        part = _create_complex_get_context_part(self.uuid, [const_names_event_log], const_get_names_event_log, config)
        return GrpcPipelinePartBase(complexContextRequestPart=part)

    def execute_callback(self, values: dict[str, GrpcContextValue]):
        for trace in values[const_names_event_log].names_log.log.traces:
            print(list(trace.events))


class PrintEventlogInfoBeforeAfter(PipelinePart):
    def __init__(self, inner_pipeline: Pipeline2):
        super().__init__()
        self.inner_pipeline = inner_pipeline

    def to_grpc_part(self) -> GrpcPipelinePartBase:
        config = GrpcPipelinePartConfiguration()

        pipeline = Pipeline2(
            PrintEventLogInfo2(),
        )

        for part in self.inner_pipeline.parts:
            pipeline.parts.append(part)

        pipeline.parts.append(PrintEventLogInfo2())

        append_pipeline_value(config, const_pipeline, pipeline)

        return GrpcPipelinePartBase(defaultPart=_create_default_pipeline_part())
