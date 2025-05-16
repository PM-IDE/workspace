from .data_models import ActivitiesRepresentationSource, Distance, TracesRepresentationSource, FeatureCountKind
from .entry_points.default_pipeline import *
from .entry_points.default_pipeline import create_complex_get_context_part
from .models.pipelines_and_context_pb2 import GrpcPipelinePartBase, GrpcPipelinePartConfiguration, \
  GrpcContextValue
from ..legacy.analysis.event_log_analysis import NComponents, visualize_dataset_pca, \
  visualize_dataset_isomap, DatasetVisualizationMethod, visualize_dataset_mds, visualize_dataset_tsne


class ClusterizationPartWithVisualization(PipelinePartWithCallback):
  def __init__(self,
               show_visualization: bool,
               fig_size: (int, int),
               view_params: (int, int),
               font_size: int,
               save_path: Optional[str],
               n_components: NComponents,
               visualization_method: DatasetVisualizationMethod,
               legend_cols: int,
               labeled_dataset_key: str):
    super().__init__()
    self.show_visualization = show_visualization
    self.fig_size = fig_size
    self.view_params = view_params
    self.font_size = font_size
    self.save_path = save_path
    self.n_components = n_components
    self.n_components = n_components
    self.visualization_method = visualization_method
    self.legend_cols = legend_cols
    self.labeled_dataset_key = labeled_dataset_key

  def execute_callback(self, values: dict[str, GrpcContextValue]):
    if not self.show_visualization:
      return

    dataset = values[self.labeled_dataset_key].labeled_dataset
    df = from_grpc_labeled_dataset(dataset)

    colors = dict()
    for label, color in zip(dataset.labels, dataset.labelsColors):
      colors[label] = from_grpc_color(color)

    vis_func = get_visualization_function(self.visualization_method)
    vis_func(df, self.n_components, colors, self.fig_size, self.view_params,
             self.font_size, self.legend_cols, self.save_path, const_cluster_labels)


def get_visualization_function(method: DatasetVisualizationMethod):
  if method == DatasetVisualizationMethod.Pca:
    return visualize_dataset_pca

  if method == DatasetVisualizationMethod.Isomap:
    return visualize_dataset_isomap

  if method == DatasetVisualizationMethod.MDS:
    return visualize_dataset_mds

  if method == DatasetVisualizationMethod.TSNE:
    return visualize_dataset_tsne

  raise KeyError()


class ClusterizationPart(ClusterizationPartWithVisualization):
  def __init__(self,
               activity_level: int,
               tolerance: float,
               class_extractor: Optional[str],
               show_visualization: bool,
               fig_size: (int, int),
               view_params: (int, int),
               font_size: int,
               save_path: Optional[str],
               activities_repr_source: ActivitiesRepresentationSource,
               distance: Distance,
               n_components: NComponents,
               visualization_method: DatasetVisualizationMethod,
               legend_cols: int):
    super().__init__(show_visualization, fig_size, view_params, font_size,
                     save_path, n_components, visualization_method, legend_cols,
                     const_labeled_traces_activities_dataset)

    self.tolerance = tolerance
    self.activity_level = activity_level
    self.class_extractor = class_extractor
    self.activities_repr_source = activities_repr_source
    self.distance = distance

  def create_common_config(self) -> GrpcPipelinePartConfiguration:
    config = GrpcPipelinePartConfiguration()
    append_uint32_value(config, const_activity_level, self.activity_level)
    append_float_value(config, const_tolerance, self.tolerance)

    append_enum_value(config,
                      const_activities_representation_source,
                      const_activities_repr_source_enum_name,
                      self.activities_repr_source.name)

    append_enum_value(config,
                      const_distance,
                      const_distance_enum_name,
                      self.distance.name)

    if self.class_extractor is not None:
      append_string_value(config, const_event_class_regex, self.class_extractor)

    return config


class ClusterizeActivitiesFromTracesKMeans(ClusterizationPart):
  def __init__(self,
               activity_level: int = 0,
               clusters_count: int = 10,
               learning_iterations_count: int = 200,
               tolerance: float = 1e-5,
               class_extractor: Optional[str] = None,
               show_visualization: bool = True,
               fig_size: (int, int) = (7, 9),
               view_params: (int, int) = (-140, 60),
               font_size: int = 14,
               save_path: Optional[str] = None,
               activities_repr_source: ActivitiesRepresentationSource = ActivitiesRepresentationSource.EventClasses,
               distance: Distance = Distance.Cosine,
               n_components: NComponents = NComponents.Three,
               visualization_method: DatasetVisualizationMethod = DatasetVisualizationMethod.Pca,
               legend_cols: int = 2):
    super().__init__(activity_level, tolerance, class_extractor, show_visualization,
                     fig_size, view_params, font_size, save_path, activities_repr_source, distance, n_components,
                     visualization_method, legend_cols)

    self.clusters_count = clusters_count
    self.learning_iterations_count = learning_iterations_count

  def to_grpc_part(self) -> GrpcPipelinePartBase:
    config = self.create_common_config()
    append_uint32_value(config, const_clusters_count, self.clusters_count)
    append_uint32_value(config, const_learning_iterations_count, self.learning_iterations_count)

    part = create_complex_get_context_part(self.uuid,
                                           self.__class__.__name__,
                                           [const_labeled_traces_activities_dataset],
                                           const_clusterize_activities_from_traces_k_means,
                                           config)

    return GrpcPipelinePartBase(complexContextRequestPart=part)


class ClusterizeActivitiesFromTracesKMeansGridSearch(ClusterizationPart):
  def __init__(self,
               activity_level: int = 0,
               learning_iterations_count: int = 200,
               tolerance: float = 1e-5,
               class_extractor: Optional[str] = None,
               show_visualization: bool = True,
               fig_size: (int, int) = (7, 9),
               view_params: (int, int) = (-140, 60),
               font_size: int = 14,
               activities_repr_source: ActivitiesRepresentationSource = ActivitiesRepresentationSource.EventClasses,
               save_path: Optional[str] = None,
               distance: Distance = Distance.Cosine,
               n_components: NComponents = NComponents.Three,
               visualization_method: DatasetVisualizationMethod = DatasetVisualizationMethod.Pca,
               legend_cols: int = 2):
    super().__init__(activity_level, tolerance, class_extractor, show_visualization,
                     fig_size, view_params, font_size, save_path, activities_repr_source, distance, n_components,
                     visualization_method, legend_cols)

    self.learning_iterations_count = learning_iterations_count

  def to_grpc_part(self) -> GrpcPipelinePartBase:
    config = self.create_common_config()
    append_uint32_value(config, const_learning_iterations_count, self.learning_iterations_count)

    part = create_complex_get_context_part(self.uuid,
                                           self.__class__.__name__,
                                           [const_labeled_traces_activities_dataset],
                                           const_clusterize_activities_from_traces_k_means_grid_search,
                                           config)

    return GrpcPipelinePartBase(complexContextRequestPart=part)


class ClusterizeActivitiesFromTracesDbscan(ClusterizationPart):
  def __init__(self,
               activity_level: int = 0,
               min_events_count_in_cluster: int = 1,
               put_noise_events_in_one_cluster: bool = True,
               tolerance: float = 1e-5,
               class_extractor: Optional[str] = None,
               show_visualization: bool = True,
               fig_size: (int, int) = (7, 9),
               view_params: (int, int) = (-140, 60),
               font_size: int = 14,
               activities_repr_source: ActivitiesRepresentationSource = ActivitiesRepresentationSource.EventClasses,
               save_path: Optional[str] = None,
               distance: Distance = Distance.Cosine,
               n_components: NComponents = NComponents.Three,
               visualization_method: DatasetVisualizationMethod = DatasetVisualizationMethod.Pca,
               legend_cols: int = 2):
    super().__init__(activity_level, tolerance, class_extractor, show_visualization,
                     fig_size, view_params, font_size, save_path, activities_repr_source, distance, n_components,
                     visualization_method, legend_cols)

    self.min_events_count_in_cluster = min_events_count_in_cluster
    self.put_noise_events_in_one_cluster = put_noise_events_in_one_cluster

  def to_grpc_part(self) -> GrpcPipelinePartBase:
    config = self.create_common_config()

    append_uint32_value(config, const_min_events_in_cluster_count, self.min_events_count_in_cluster)
    append_bool_value(config, const_put_noise_events_in_one_cluster, self.put_noise_events_in_one_cluster)

    part = create_complex_get_context_part(self.uuid,
                                           self.__class__.__name__,
                                           [const_labeled_traces_activities_dataset],
                                           const_clusterize_activities_from_traces_dbscan,
                                           config)

    return GrpcPipelinePartBase(complexContextRequestPart=part)


class ClusterizeLogTracesBase(ClusterizationPartWithVisualization):
  def __init__(self,
               pipeline_part_name: str,
               after_clusterization_pipeline: Optional[Pipeline] = None,
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
               class_extractor: Optional[str] = None,
               feature_count_kind: FeatureCountKind = FeatureCountKind.Count,
               percentage_from_max_value: float = 0):
    super().__init__(show_visualization, fig_size, view_params, font_size,
                     save_path, n_components, visualization_method, legend_cols,
                     const_labeled_log_traces_dataset)

    self.pipeline_part_name = pipeline_part_name
    self.after_clusterization_pipeline = after_clusterization_pipeline
    self.distance = distance
    self.traces_repr_source = traces_repr_source
    self.class_extractor = class_extractor
    self.feature_count_kind = feature_count_kind
    self.percentage_from_max_value = percentage_from_max_value

  def fill_config_values(self, config):
    pass

  def to_grpc_part(self) -> GrpcPipelinePartBase:
    config = GrpcPipelinePartConfiguration()
    self.fill_config_values(config)

    append_enum_value(config,
                      const_distance,
                      const_distance_enum_name,
                      self.distance.name)

    append_enum_value(config,
                      const_traces_repr_source,
                      const_traces_repr_source_enum_name,
                      self.traces_repr_source.name)

    append_enum_value(config,
                      const_feature_count_kind,
                      const_feature_count_kind_enum_name,
                      self.feature_count_kind.name)

    append_float_value(config, const_percentage_from_max_value, self.percentage_from_max_value)

    if self.after_clusterization_pipeline is not None:
      append_pipeline_value(config, const_pipeline, self.after_clusterization_pipeline)

    if self.class_extractor is not None:
      append_string_value(config, const_event_class_regex, self.class_extractor)

    part = create_complex_get_context_part(self.uuid,
                                           self.__class__.__name__,
                                           [const_labeled_log_traces_dataset],
                                           self.pipeline_part_name,
                                           config)

    return GrpcPipelinePartBase(complexContextRequestPart=part)

  def append_parts_with_callbacks(self, parts: list['PipelinePartWithCallback']):
    super().append_parts_with_callbacks(parts)

    if self.after_clusterization_pipeline is not None:
      append_parts_with_callbacks(self.after_clusterization_pipeline.parts, parts)


class ClusterizeLogTracesDbscan(ClusterizeLogTracesBase):
  def __init__(self,
               after_clusterization_pipeline: Pipeline,
               min_events_count_in_cluster: int = 1,
               put_noise_events_in_one_cluster: bool = True,
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
               class_extractor: Optional[str] = None,
               feature_count_kind: FeatureCountKind = FeatureCountKind.Count,
               percentage_from_max_value: float = 0):
    super().__init__(const_clusterize_log_traces,
                     after_clusterization_pipeline,
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
                     class_extractor,
                     feature_count_kind,
                     percentage_from_max_value)

    self.put_noise_events_in_one_cluster = put_noise_events_in_one_cluster
    self.min_events_count_in_cluster = min_events_count_in_cluster
    self.tolerance = tolerance

  def fill_config_values(self, config):
    append_float_value(config, const_tolerance, self.tolerance)
    append_uint32_value(config, const_min_events_in_cluster_count, self.min_events_count_in_cluster)
    append_bool_value(config, const_put_noise_events_in_one_cluster, self.put_noise_events_in_one_cluster)


class ClusterizeLogTracesKMeansGridSearch(ClusterizeLogTracesBase):
  def __init__(self,
               learning_iterations_count: int,
               after_clusterization_pipeline: Pipeline,
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
               class_extractor: Optional[str] = None,
               feature_count_kind: FeatureCountKind = FeatureCountKind.Count,
               percentage_from_max_value: float = 0):
    super().__init__(const_clusterize_traces_k_means_grid_search,
                     after_clusterization_pipeline,
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
                     class_extractor,
                     feature_count_kind,
                     percentage_from_max_value)

    self.tolerance = tolerance
    self.min_events_count_in_cluster = min_events_count_in_cluster
    self.learning_iterations_count = learning_iterations_count

  def fill_config_values(self, config):
    append_uint32_value(config, const_learning_iterations_count, self.learning_iterations_count)
    append_float_value(config, const_tolerance, self.tolerance)
    append_uint32_value(config, const_min_events_in_cluster_count, self.min_events_count_in_cluster)


class ClusterizeLogTracesDbscanGridSearch(ClusterizeLogTracesBase):
  def __init__(self,
               after_clusterization_pipeline: Pipeline,
               min_points_in_cluster_vec: list[int] = [1],
               tolerances: list[float] = [1e-5],
               put_noise_events_in_one_cluster: bool = True,
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
               class_extractor: Optional[str] = None,
               feature_count_kind: FeatureCountKind = FeatureCountKind.Count,
               percentage_from_max_value: float = 0):
    super().__init__(const_clusterize_traces_dbscan_grid_search,
                     after_clusterization_pipeline,
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
                     class_extractor,
                     feature_count_kind,
                     percentage_from_max_value)

    self.tolerances = tolerances
    self.min_points_in_cluster_vec = min_points_in_cluster_vec
    self.put_noise_events_in_one_cluster = put_noise_events_in_one_cluster

  def fill_config_values(self, config):
    append_float_array_value(config, const_tolerances, self.tolerances)
    append_uint_array_value(config, const_min_points_in_cluster_array, self.min_points_in_cluster_vec)
    append_bool_value(config, const_put_noise_events_in_one_cluster, self.put_noise_events_in_one_cluster)


class AbstractTimelineDiagram(ClusterizeLogTracesBase):
  def __init__(self,
               min_events_count_in_cluster: int = 1,
               put_noise_events_in_one_cluster: bool = True,
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
               class_extractor: Optional[str] = None,
               feature_count_kind: FeatureCountKind = FeatureCountKind.Count,
               after_clusterization_pipeline: Optional[Pipeline] = None,
               percent_from_max_value: float = 0):
    super().__init__(const_abstract_timeline_diagram,
                     after_clusterization_pipeline,
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
                     class_extractor,
                     feature_count_kind,
                     percent_from_max_value)

    self.tolerance = tolerance
    self.min_events_count_in_cluster = min_events_count_in_cluster
    self.put_noise_events_in_one_cluster = put_noise_events_in_one_cluster

  def fill_config_values(self, config):
    append_float_value(config, const_tolerance, self.tolerance)
    append_uint32_value(config, const_min_events_in_cluster_count, self.min_events_count_in_cluster)
    append_bool_value(config, const_put_noise_events_in_one_cluster, self.put_noise_events_in_one_cluster)


class AbstractMultithreadedEventsGroups(ClusterizeLogTracesBase):
  def __init__(self,
               thread_attribute: str,
               time_attribute: Optional[str],
               sequential_regexes: Optional[list[str]] = None,
               min_events_count_in_cluster: int = 1,
               put_noise_events_in_one_cluster: bool = True,
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
               class_extractor: Optional[str] = None,
               feature_count_kind: FeatureCountKind = FeatureCountKind.Count,
               after_clusterization_pipeline: Optional[Pipeline] = None,
               percent_from_max_value: float = 0):
    super().__init__(const_abstract_multithreaded_events_groups,
                     after_clusterization_pipeline,
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
                     class_extractor,
                     feature_count_kind,
                     percent_from_max_value)

    self.thread_attribute = thread_attribute
    self.time_attribute = time_attribute
    self.tolerance = tolerance
    self.min_events_count_in_cluster = min_events_count_in_cluster
    self.sequential_regexes = sequential_regexes
    self.put_noise_events_in_one_cluster = put_noise_events_in_one_cluster

  def fill_config_values(self, config: GrpcPipelinePartConfiguration):
    append_float_value(config, const_tolerance, self.tolerance)
    append_uint32_value(config, const_min_events_in_cluster_count, self.min_events_count_in_cluster)
    append_string_value(config, const_thread_attribute, self.thread_attribute)
    append_bool_value(config, const_put_noise_events_in_one_cluster, self.put_noise_events_in_one_cluster)

    if self.sequential_regexes is not None:
      append_strings_context_value(config, const_regexes, self.sequential_regexes)

    if self.time_attribute is not None:
      append_string_value(config, const_time_attribute, self.time_attribute)
