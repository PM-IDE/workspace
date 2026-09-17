from .entry_points.default_pipeline import *
from .models.pipelines_and_context_pb2 import GrpcPipelinePartBase, GrpcPipelinePartConfiguration, \
  GrpcContextValue
from .xes_parts import WriteBytesToFilePipelinePartBase


class UseNamesEventLog(PipelinePart):
  def to_grpc_part(self) -> GrpcPipelinePartBase:
    return GrpcPipelinePartBase(defaultPart=create_default_pipeline_part(const_use_names_event_log))


class PrintEventLog(PipelinePartWithCallback):
  def to_grpc_part(self) -> GrpcPipelinePartBase:
    config = GrpcPipelinePartConfiguration()
    part = create_complex_get_context_part(self.uuid,
                                           self.__class__.__name__,
                                           [const_names_event_log],
                                           const_get_names_event_log,
                                           config)

    return GrpcPipelinePartBase(complexContextRequestPart=part)

  def execute_callback(self, values: dict[str, GrpcContextValue]):
    for trace in values[const_names_event_log].names_log.log.traces:
      print(list(trace.events))


class PrintEventlogInfoBeforeAfter(PipelinePart):
  def __init__(self, inner_pipeline: Pipeline):
    super().__init__()
    self.inner_pipeline = inner_pipeline

  def to_grpc_part(self) -> GrpcPipelinePartBase:
    config = GrpcPipelinePartConfiguration()

    pipeline = Pipeline(
      PrintEventLogInfo(),
    )

    for part in self.inner_pipeline.parts:
      pipeline.parts.append(part)

    pipeline.parts.append(PrintEventLogInfo())

    append_pipeline_value(config, const_pipeline, pipeline)

    return GrpcPipelinePartBase(defaultPart=create_default_pipeline_part())


class MergeGraphs(PipelinePart):
  def to_grpc_part(self) -> GrpcPipelinePartBase:
    return GrpcPipelinePartBase(defaultPart=create_default_pipeline_part(const_merge_graphs))


class AddGraphToGraphs(PipelinePart):
  def to_grpc_part(self) -> GrpcPipelinePartBase:
    return GrpcPipelinePartBase(defaultPart=create_default_pipeline_part(const_add_graph_to_graphs))


class ClearGraphs(PipelinePart):
  def to_grpc_part(self) -> GrpcPipelinePartBase:
    return GrpcPipelinePartBase(defaultPart=create_default_pipeline_part(const_clear_graphs))


class TerminateIfEmptyLog(PipelinePart):
  def to_grpc_part(self) -> GrpcPipelinePartBase:
    return GrpcPipelinePartBase(defaultPart=create_default_pipeline_part(const_terminate_if_empty_log))


class SerializeGraphProm(PipelinePart):
  def __init__(self, save_path: str):
    super().__init__()
    self.save_path = save_path

  def to_grpc_part(self) -> GrpcPipelinePartBase:
    config = GrpcPipelinePartConfiguration()
    append_string_value(config, const_path, self.save_path)
    return GrpcPipelinePartBase(defaultPart=create_default_pipeline_part(const_serialize_graph_prom, config))


class SerializeGraphPromBytes(WriteBytesToFilePipelinePartBase):
  def __init__(self, save_path: str):
    super().__init__(save_path, const_serialize_graph_prom_bytes)


class GetGraphInfo(PipelinePartWithCallback):
  def to_grpc_part(self) -> GrpcPipelinePartBase:
    part = create_complex_get_context_part(self.uuid,
                                           self.__class__.__name__,
                                           [const_graph_info],
                                           const_get_graph_info,
                                           GrpcPipelinePartConfiguration())

    return GrpcPipelinePartBase(complexContextRequestPart=part)

  def execute_callback(self, values: dict[str, GrpcContextValue]):
    print(values[const_graph_info].graph_info)


class GetPetriNetInfo(PipelinePartWithCallback):
  def to_grpc_part(self) -> GrpcPipelinePartBase:
    part = create_complex_get_context_part(self.uuid,
                                           self.__class__.__name__,
                                           [const_petri_net_info],
                                           const_get_petri_net_info,
                                           GrpcPipelinePartConfiguration())

    return GrpcPipelinePartBase(complexContextRequestPart=part)

  def execute_callback(self, values: dict[str, GrpcContextValue]):
    print(values[const_petri_net_info].petri_net_info)
