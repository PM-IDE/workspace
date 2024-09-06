from ...ficus.grpc_pipelines.constants import const_names_event_log, const_get_names_event_log
from ...ficus.grpc_pipelines.entry_points.default_pipeline import PipelinePartWithCallback, _create_complex_get_context_part
from ...ficus.grpc_pipelines.models.pipelines_and_context_pb2 import GrpcPipelinePartBase, GrpcPipelinePartConfiguration, \
    GrpcContextValue


class AssertNamesLogTestPart(PipelinePartWithCallback):
    def __init__(self, expected_names_log: list[list[str]]):
        super().__init__()
        self.expected_names_log = expected_names_log

    def to_grpc_part(self) -> GrpcPipelinePartBase:
        config = GrpcPipelinePartConfiguration()
        part = _create_complex_get_context_part(self.uuid, [const_names_event_log], const_get_names_event_log, config)
        return GrpcPipelinePartBase(complexContextRequestPart=part)

    def execute_callback(self, values: dict[str, GrpcContextValue]):
        names_log = []
        for trace in values[const_names_event_log].names_log.log.traces:
            names_log.append(list(trace.events))

        assert names_log == self.expected_names_log
