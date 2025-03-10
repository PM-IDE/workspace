from .data_models import Distance, TracesRepresentationSource
from ..legacy.analysis.event_log_analysis import *
from .entry_points.default_pipeline import *
from .models.pipelines_and_context_pb2 import *
from .activities_parts import ClusterizeLogTracesDbscanBase
from ..legacy.analysis.event_log_analysis_canvas import draw_log_timeline_diagram_canvas

class DiscoverLogTimelineDiagram(PipelinePartWithCallback):
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
    super().__init__()
    self.thread_attribute = thread_attribute
    self.time_attribute = time_attribute
    self.event_group_delta = event_group_delta
    self.title = title
    self.save_path = save_path
    self.plot_legend = plot_legend
    self.height_scale = height_scale
    self.width_scale = width_scale
    self.distance_scale = distance_scale
    self.rect_width_scale = rect_width_scale

  def to_grpc_part(self) -> GrpcPipelinePartBase:
    config = GrpcPipelinePartConfiguration()
    append_string_value(config, const_thread_attribute, self.thread_attribute)

    if self.time_attribute is not None:
      append_string_value(config, const_time_attribute, self.time_attribute)

    if self.event_group_delta is not None:
      append_uint32_value(config, const_time_delta_attribute, self.event_group_delta)

    part = create_complex_get_context_part(self.uuid,
                                           self.__class__.__name__,
                                           [const_log_timeline_diagram],
                                           const_discover_log_timeline_diagram,
                                           config)

    return GrpcPipelinePartBase(complexContextRequestPart=part)

  def execute_callback(self, values: dict[str, GrpcContextValue]):
    draw_log_timeline_diagram_canvas(values[const_log_timeline_diagram].logTimelineDiagram,
                                     self.rect_width_scale,
                                     self.distance_scale,
                                     self.title,
                                     self.save_path,
                                     self.plot_legend,
                                     self.width_scale,
                                     self.height_scale)


class CreateThreadsLog(PipelinePart):
  def __init__(self, thread_attribute: str):
    super().__init__()
    self.thread_attribute = thread_attribute

  def to_grpc_part(self) -> GrpcPipelinePartBase:
    config = GrpcPipelinePartConfiguration()
    append_string_value(config, const_thread_attribute, self.thread_attribute)

    return GrpcPipelinePartBase(defaultPart=create_default_pipeline_part(const_create_threads_log, config))


class AbstractTimelineDiagram(ClusterizeLogTracesDbscanBase):
  def __init__(self,
               min_events_count_in_cluster: int = 1,
               tolerance: float = 1e-5,
               show_visualization: bool = True,
               fig_size: (int, int) = (7, 9),
               view_params: (int, int) = (-140, 60),
               font_size: int = 14,
               save_path: Optional[str] = None,
               distance: Distance = Distance.Cosine,
               n_components: NComponents = NComponents.Three,
               visualization_method: DatasetVisualizationMethod = DatasetVisualizationMethod.Pca,
               legend_cols: int = 2,
               traces_repr_source: TracesRepresentationSource = TracesRepresentationSource.Events,
               class_extractor: Optional[str] = None):
    super().__init__(const_abstract_timeline_diagram,
                     None,
                     min_events_count_in_cluster,
                     tolerance,
                     show_visualization,
                     fig_size,
                     view_params,
                     font_size,
                     save_path,
                     distance,
                     n_components,
                     visualization_method,
                     legend_cols,
                     traces_repr_source,
                     class_extractor)
