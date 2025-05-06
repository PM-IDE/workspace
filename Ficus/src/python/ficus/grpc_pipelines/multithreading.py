from .entry_points.default_pipeline import *
from .models.pipelines_and_context_pb2 import *
from ..legacy.analysis.event_log_analysis_canvas import draw_log_timeline_diagram_canvas

class DiscoverTimelineDiagramBase(PipelinePartWithCallback):
  def __init__(self,
               title: Optional[str] = None,
               save_path: Optional[str] = None,
               plot_legend: bool = False,
               height_scale: float = 1,
               width_scale: float = 1,
               distance_scale: float = 1,
               rect_width_scale: int = 1):
    super().__init__()
    self.title = title
    self.save_path = save_path
    self.plot_legend = plot_legend
    self.height_scale = height_scale
    self.width_scale = width_scale
    self.distance_scale = distance_scale
    self.rect_width_scale = rect_width_scale

  def execute_callback(self, values: dict[str, GrpcContextValue]):
    draw_log_timeline_diagram_canvas(values[const_log_timeline_diagram].logTimelineDiagram,
                                     self.rect_width_scale,
                                     self.distance_scale,
                                     self.title,
                                     self.save_path,
                                     self.plot_legend,
                                     self.width_scale,
                                     self.height_scale)


class DiscoverLogTimelineDiagram(DiscoverTimelineDiagramBase):
  def __init__(self,
               thread_attribute: str,
               time_attribute: Optional[str] = None,
               event_group_delta: Optional[int] = None,
               title: Optional[str] = None,
               save_path: Optional[str] = None,
               plot_legend: bool = False,
               height_scale: float = 1,
               width_scale: float = 1,
               distance_scale: float = 1,
               rect_width_scale: int = 1):
    super().__init__(title,
                     save_path,
                     plot_legend,
                     height_scale,
                     width_scale,
                     distance_scale,
                     rect_width_scale)

    self.thread_attribute = thread_attribute
    self.time_attribute = time_attribute
    self.event_group_delta = event_group_delta

  def to_grpc_part(self) -> GrpcPipelinePartBase:
    config = GrpcPipelinePartConfiguration()
    append_string_value(config, const_thread_attribute, self.thread_attribute)

    return _create_discover_log_timeline_diagram_grpc_part(self, config, const_discover_log_timeline_diagram)


def _create_discover_log_timeline_diagram_grpc_part(self, config, part_name):
  if self.time_attribute is not None:
    append_string_value(config, const_time_attribute, self.time_attribute)

  if self.event_group_delta is not None:
    append_uint32_value(config, const_time_delta_attribute, self.event_group_delta)

  part = create_complex_get_context_part(self.uuid,
                                         self.__class__.__name__,
                                         [const_log_timeline_diagram],
                                         part_name,
                                         config)

  return GrpcPipelinePartBase(complexContextRequestPart=part)


class CreateThreadsLog(PipelinePart):
  def __init__(self, thread_attribute: str):
    super().__init__()
    self.thread_attribute = thread_attribute

  def to_grpc_part(self) -> GrpcPipelinePartBase:
    config = GrpcPipelinePartConfiguration()
    append_string_value(config, const_thread_attribute, self.thread_attribute)

    return GrpcPipelinePartBase(defaultPart=create_default_pipeline_part(const_create_threads_log, config))


class DiscoverTracesTimelineDiagram(DiscoverTimelineDiagramBase):
  def __init__(self,
               discover_events_groups_in_each_trace: bool,
               time_attribute: Optional[str] = None,
               event_group_delta: Optional[int] = None,
               title: Optional[str] = None,
               save_path: Optional[str] = None,
               plot_legend: bool = False,
               height_scale: float = 1,
               width_scale: float = 1,
               distance_scale: float = 1,
               rect_width_scale: int = 1):
    super().__init__(title,
                     save_path,
                     plot_legend,
                     height_scale,
                     width_scale,
                     distance_scale,
                     rect_width_scale)

    self.discover_events_groups_in_each_trace = discover_events_groups_in_each_trace
    self.time_attribute = time_attribute
    self.event_group_delta = event_group_delta

  def to_grpc_part(self) -> GrpcPipelinePartBase:
    config = GrpcPipelinePartConfiguration()
    append_bool_value(config, const_discover_events_groups_in_each_trace, self.discover_events_groups_in_each_trace)

    return _create_discover_log_timeline_diagram_grpc_part(self, config, const_discover_traces_timeline_diagram)


class PrepareSoftwareLog(PipelinePart):
  def __init__(self, time_attribute: Optional[str]):
    super().__init__()
    self.time_attribute = time_attribute

  def to_grpc_part(self) -> GrpcPipelinePartBase:
    config = GrpcPipelinePartConfiguration()
    append_string_value(config, const_time_attribute, self.time_attribute)

    return GrpcPipelinePartBase(defaultPart=create_default_pipeline_part(const_prepare_software_log, config))


class ShortenAllocationType(PipelinePart):
  def to_grpc_part(self) -> GrpcPipelinePartBase:
    return GrpcPipelinePartBase(defaultPart=create_default_pipeline_part(const_shorten_allocation_type))


class ShortenMethodNames(PipelinePart):
  def to_grpc_part(self) -> GrpcPipelinePartBase:
    return GrpcPipelinePartBase(defaultPart=create_default_pipeline_part(const_shorten_method_names))


class SetMethodsDisplayName(PipelinePart):
  def to_grpc_part(self) -> GrpcPipelinePartBase:
    return GrpcPipelinePartBase(defaultPart=create_default_pipeline_part(const_set_methods_display_name))
