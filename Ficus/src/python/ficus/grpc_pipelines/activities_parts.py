from .clustering import get_visualization_function
from .data_models import ActivitiesRepresentationSource, LogSerializationFormat
from .entry_points.default_pipeline import *
from .entry_points.default_pipeline import create_default_pipeline_part, create_complex_get_context_part
from .models.pipelines_and_context_pb2 import GrpcPipelinePartBase, GrpcPipelinePartConfiguration, \
  GrpcContextValue
from .patterns_parts import FindMaximalRepeats, \
  FindSuperMaximalRepeats, FindNearSuperMaximalRepeats, FindPrimitiveTandemArrays, FindMaximalTandemArrays
from ..legacy.analysis.event_log_analysis import NComponents, DatasetVisualizationMethod
from ..legacy.pipelines.analysis.patterns.models import AdjustingMode


class DiscoverActivities(PipelinePart):
  def __init__(self, activity_level: int):
    super().__init__()
    self.activity_level = activity_level

  def to_grpc_part(self) -> GrpcPipelinePartBase:
    config = GrpcPipelinePartConfiguration()
    append_uint32_value(config, const_activity_level, self.activity_level)
    return GrpcPipelinePartBase(defaultPart=create_default_pipeline_part(const_discover_activities, config))


class DiscoverActivitiesInstances(PipelinePart):
  def __init__(self,
               narrow_activities: NarrowActivityKind = NarrowActivityKind.NarrowDown,
               min_events_in_activity: int = 0,
               activity_filter_kind: ActivityFilterKind = ActivityFilterKind.DefaultFilter):
    super().__init__()
    self.narrow_activities = narrow_activities
    self.min_events_in_activity = min_events_in_activity
    self.activity_filter_kind = activity_filter_kind

  def to_grpc_part(self) -> GrpcPipelinePartBase:
    config = GrpcPipelinePartConfiguration()
    append_narrow_kind(config, const_narrow_activities, self.narrow_activities)
    append_uint32_value(config, const_min_events_in_activity, self.min_events_in_activity)
    append_activity_filter_kind(config, const_activity_filter_kind, self.activity_filter_kind)

    return GrpcPipelinePartBase(
      defaultPart=create_default_pipeline_part(const_discover_activities_instances, config))


class CreateLogFromActivitiesInstances(PipelinePart):
  def __init__(self,
               strategy: UndefinedActivityHandlingStrategy = UndefinedActivityHandlingStrategy.DontInsert):
    super().__init__()
    self.strategy = strategy

  def to_grpc_part(self) -> GrpcPipelinePartBase:
    config = GrpcPipelinePartConfiguration()
    append_undef_activity_handling_strat(config, const_undef_activity_handling_strategy, self.strategy)

    return GrpcPipelinePartBase(defaultPart=create_default_pipeline_part(const_create_log_from_activities, config))


class DiscoverActivitiesForSeveralLevels(PipelinePart):
  def __init__(self,
               event_classes: list[str],
               patterns_kind: PatternsKind,
               narrow_activities: NarrowActivityKind = NarrowActivityKind.NarrowDown,
               activity_level: int = 0,
               strategy: PatternsDiscoveryStrategy = PatternsDiscoveryStrategy.FromAllTraces,
               max_array_length: int = 20,
               adjusting_mode: AdjustingMode = AdjustingMode.FromAllLog,
               min_events_in_unattached_subtrace_count: int = 0,
               min_events_in_activity_count: int = 0,
               activity_filter_kind: ActivityFilterKind = ActivityFilterKind.DefaultFilter):
    super().__init__()
    self.event_classes = event_classes
    self.narrow_activities = narrow_activities
    self.patterns_kind = patterns_kind
    self.activity_level = activity_level
    self.strategy = strategy
    self.max_array_length = max_array_length
    self.adjusting_mode = adjusting_mode
    self.min_events_in_unattached_subtrace_count = min_events_in_unattached_subtrace_count
    self.min_events_in_activity_count = min_events_in_activity_count
    self.activity_filter_kind = activity_filter_kind

  def to_grpc_part(self) -> GrpcPipelinePartBase:
    config = GrpcPipelinePartConfiguration()

    append_narrow_kind(config, const_narrow_activities, self.narrow_activities)
    append_strings_context_value(config, const_event_classes_regexes, self.event_classes)
    append_adjusting_mode(config, const_adjusting_mode, self.adjusting_mode)
    append_uint32_value(config, const_activity_level, self.activity_level)
    append_uint32_value(config, const_events_count, self.min_events_in_unattached_subtrace_count)
    append_patterns_kind(config, const_patterns_kind, self.patterns_kind)
    append_patterns_discovery_strategy(config, const_patterns_discovery_strategy, self.strategy)
    append_uint32_value(config, const_min_events_in_activity, self.min_events_in_activity_count)
    append_activity_filter_kind(config, const_activity_filter_kind, self.activity_filter_kind)
    append_uint32_value(config, const_tandem_array_length, self.max_array_length)

    default_part = create_default_pipeline_part(const_discover_activities_for_several_levels, config)
    return GrpcPipelinePartBase(defaultPart=default_part)


class DiscoverActivitiesFromPatterns(PipelinePart):
  def __init__(self,
               patterns_kind: PatternsKind,
               strategy: PatternsDiscoveryStrategy = PatternsDiscoveryStrategy.FromAllTraces,
               max_array_length: int = 20,
               activity_level: int = 0):
    super().__init__()
    self.patterns_kind = patterns_kind
    self.strategy = strategy
    self.max_array_length = max_array_length
    self.activity_level = activity_level

  def to_grpc_part(self) -> GrpcPipelinePartBase:
    match self.patterns_kind:
      case PatternsKind.MaximalRepeats:
        patterns_part = FindMaximalRepeats(strategy=self.strategy)
      case PatternsKind.SuperMaximalRepeats:
        patterns_part = FindSuperMaximalRepeats(strategy=self.strategy)
      case PatternsKind.NearSuperMaximalRepeats:
        patterns_part = FindNearSuperMaximalRepeats(strategy=self.strategy)
      case PatternsKind.PrimitiveTandemArrays:
        patterns_part = FindPrimitiveTandemArrays(max_array_length=self.max_array_length)
      case PatternsKind.MaximalTandemArrays:
        patterns_part = FindMaximalTandemArrays(max_array_length=self.max_array_length)
      case _:
        print(f"Unknown patterns_kind: {self.patterns_kind}")
        raise ValueError()

    pipeline = Pipeline(
      patterns_part,
      DiscoverActivities(activity_level=self.activity_level),
    )

    config = GrpcPipelinePartConfiguration()
    append_pipeline_value(config, const_pipeline, pipeline)

    default_part = create_default_pipeline_part(const_execute_frontend_part, config)
    return GrpcPipelinePartBase(defaultPart=default_part)


class DiscoverActivitiesUntilNoMore(PipelinePart):
  def __init__(self,
               event_class: str = None,
               patterns_kind: PatternsKind = PatternsKind.MaximalRepeats,
               narrow_activities: NarrowActivityKind = NarrowActivityKind.NarrowDown,
               activity_level: int = 0,
               strategy: PatternsDiscoveryStrategy = PatternsDiscoveryStrategy.FromAllTraces,
               undef_strategy: UndefinedActivityHandlingStrategy = UndefinedActivityHandlingStrategy.DontInsert,
               max_array_length: int = 20,
               adjusting_mode: AdjustingMode = AdjustingMode.FromAllLog,
               min_events_in_unattached_subtrace_count: int = 0,
               min_events_in_activity_count: int = 0,
               activity_filter_kind: ActivityFilterKind = ActivityFilterKind.DefaultFilter,
               after_activities_extraction_pipeline: Optional[Pipeline] = None,
               execute_only_on_last_extraction: bool = False):
    super().__init__()
    self.event_class = event_class
    self.narrow_activities = narrow_activities
    self.patterns_kind = patterns_kind
    self.activity_level = activity_level
    self.strategy = strategy
    self.undef_strategy = undef_strategy
    self.max_array_length = max_array_length
    self.adjusting_mode = adjusting_mode
    self.min_events_count = min_events_in_unattached_subtrace_count
    self.min_events_in_activity_count = min_events_in_activity_count
    self.activity_filter_kind = activity_filter_kind
    self.after_activities_extraction_pipeline = after_activities_extraction_pipeline
    self.execute_only_on_last_extraction = execute_only_on_last_extraction

  def to_grpc_part(self) -> GrpcPipelinePartBase:
    config = GrpcPipelinePartConfiguration()

    append_narrow_kind(config, const_narrow_activities, self.narrow_activities)
    append_adjusting_mode(config, const_adjusting_mode, self.adjusting_mode)
    append_uint32_value(config, const_activity_level, self.activity_level)
    append_uint32_value(config, const_events_count, self.min_events_count)
    append_patterns_kind(config, const_patterns_kind, self.patterns_kind)
    append_patterns_discovery_strategy(config, const_patterns_discovery_strategy, self.strategy)
    append_uint32_value(config, const_min_events_in_activity, self.min_events_in_activity_count)
    append_undef_activity_handling_strat(config, const_undef_activity_handling_strategy, self.undef_strategy)
    append_activity_filter_kind(config, const_activity_filter_kind, self.activity_filter_kind)
    append_bool_value(config, const_execute_only_on_last_extraction, self.execute_only_on_last_extraction)

    if self.event_class is not None:
      append_string_value(config, const_event_class_regex, self.event_class)

    if self.after_activities_extraction_pipeline is not None:
      append_pipeline_value(config, const_pipeline, self.after_activities_extraction_pipeline)

    default_part = create_default_pipeline_part(const_discover_activities_until_no_more, config)
    return GrpcPipelinePartBase(defaultPart=default_part)

  def append_parts_with_callbacks(self, parts: list['PipelinePartWithCallback']):
    super().append_parts_with_callbacks(parts)

    if self.after_activities_extraction_pipeline is not None:
      append_parts_with_callbacks(self.after_activities_extraction_pipeline.parts, parts)


class ExecuteWithEachActivityLog(PipelinePart):
  def __init__(self, activities_logs_source: ActivitiesLogsSource, activity_level: int,
               activity_log_pipeline: Pipeline):
    super().__init__()
    self.activity_level = activity_level
    self.activity_log_pipeline = activity_log_pipeline
    self.activities_logs_source = activities_logs_source

  def to_grpc_part(self) -> GrpcPipelinePartBase:
    config = GrpcPipelinePartConfiguration()
    append_pipeline_value(config, const_pipeline, self.activity_log_pipeline)
    append_uint32_value(config, const_activity_level, self.activity_level)
    append_activities_logs_source(config, const_activities_logs_source, self.activities_logs_source)

    default_part = create_default_pipeline_part(const_execute_with_each_activity_log, config)
    return GrpcPipelinePartBase(defaultPart=default_part)

  def append_parts_with_callbacks(self, parts: list['PipelinePartWithCallback']):
    super().append_parts_with_callbacks(parts)
    append_parts_with_callbacks(self.activity_log_pipeline.parts, parts)


class SubstituteUnderlyingEvents(PipelinePart):
  def to_grpc_part(self) -> GrpcPipelinePartBase:
    return GrpcPipelinePartBase(defaultPart=create_default_pipeline_part(const_substitute_underlying_events))


class ClearActivitiesRelatedStuff(PipelinePart):
  def to_grpc_part(self) -> GrpcPipelinePartBase:
    return GrpcPipelinePartBase(defaultPart=create_default_pipeline_part(const_clear_activities_related_stuff))


class PrintNumberOfUnderlyingEvents(PipelinePartWithCallback):
  def to_grpc_part(self) -> GrpcPipelinePartBase:
    part = create_complex_get_context_part(self.uuid,
                                           self.__class__.__name__,
                                           [const_underlying_events_count],
                                           const_get_number_of_underlying_events,
                                           GrpcPipelinePartConfiguration())

    return GrpcPipelinePartBase(complexContextRequestPart=part)

  def execute_callback(self, values: dict[str, GrpcContextValue]):
    print(f'Underlying events count: {values[const_underlying_events_count].uint32}')


class ApplyClassExtractor(PipelinePart):
  def __init__(self, class_extractor_regex: str, filter_regex: str = ".*"):
    super().__init__()
    self.class_extractor_regex = class_extractor_regex
    self.filter_regex = filter_regex

  def to_grpc_part(self) -> GrpcPipelinePartBase:
    config = GrpcPipelinePartConfiguration()
    append_string_value(config, const_event_class_regex, self.class_extractor_regex)
    append_string_value(config, const_regex, self.filter_regex)

    part = create_default_pipeline_part(const_apply_class_extractor, config)
    return GrpcPipelinePartBase(defaultPart=part)

class VisualizeTracesActivities(PipelinePartWithCallback):
  def __init__(self,
               activity_level: int = 0,
               class_extractor: Optional[str] = None,
               fig_size: (int, int) = (7, 9),
               view_params: (int, int) = (-140, 60),
               font_size: int = 14,
               save_path: Optional[str] = None,
               activities_repr_source: ActivitiesRepresentationSource = ActivitiesRepresentationSource.EventClasses,
               n_components: NComponents = NComponents.Three,
               visualization_method: DatasetVisualizationMethod = DatasetVisualizationMethod.Pca,
               legend_cols: int = 2):
    super().__init__()
    self.activity_level = activity_level
    self.class_extractor = class_extractor
    self.fig_size = fig_size
    self.font_size = font_size
    self.save_path = save_path
    self.n_components = n_components
    self.visualization_method = visualization_method
    self.activities_repr_source = activities_repr_source
    self.view_params = view_params
    self.legend_cols = legend_cols

  def to_grpc_part(self) -> GrpcPipelinePartBase:
    config = GrpcPipelinePartConfiguration()
    append_uint32_value(config, const_activity_level, self.activity_level)

    if self.class_extractor is not None:
      append_string_value(config, const_event_class_regex, self.class_extractor)

    append_enum_value(config,
                      const_activities_representation_source,
                      const_activities_repr_source_enum_name,
                      self.activities_repr_source.name)

    part = create_complex_get_context_part(self.uuid,
                                           self.__class__.__name__,
                                           [const_traces_activities_dataset],
                                           const_create_traces_activities_dataset,
                                           config)

    return GrpcPipelinePartBase(complexContextRequestPart=part)

  def execute_callback(self, values: dict[str, GrpcContextValue]):
    dataset = values[const_traces_activities_dataset].dataset
    df = from_grpc_ficus_dataset(dataset)
    vis_func = get_visualization_function(self.visualization_method)

    vis_func(df, self.n_components, dict(), self.fig_size, self.view_params,
             self.font_size, self.legend_cols, self.save_path, None)


class SerializeActivitiesLogs(PipelinePart):
  def __init__(self,
               directory_path: str,
               serialization_format: LogSerializationFormat,
               activity_level: int = 0,
               activities_source: ActivitiesLogsSource = ActivitiesLogsSource.TracesActivities):
    super().__init__()
    self.activities_source = activities_source
    self.path = directory_path
    self.serialization_format = serialization_format
    self.activity_level = activity_level

  def to_grpc_part(self) -> GrpcPipelinePartBase:
    config = GrpcPipelinePartConfiguration()
    append_string_value(config, const_path, self.path)
    append_uint32_value(config, const_activity_level, self.activity_level)

    append_enum_value(config,
                      const_log_serialization_format,
                      const_log_serialization_format_enum_name,
                      self.serialization_format.name)

    append_enum_value(config,
                      const_activities_logs_source,
                      const_activities_logs_source_enum_name,
                      self.activities_source.name)

    return GrpcPipelinePartBase(defaultPart=create_default_pipeline_part(const_serialize_activities_logs, config))


class ReverseHierarchyIndices(PipelinePart):
  def to_grpc_part(self) -> GrpcPipelinePartBase:
    return GrpcPipelinePartBase(defaultPart=create_default_pipeline_part(const_reverse_hierarchy_indices))


class DiscoverLoopsStrict(PipelinePart):
  def __init__(self, max_loop_length = 20):
    super().__init__()
    self.max_loop_length = max_loop_length

  def to_grpc_part(self) -> GrpcPipelinePartBase:
    config = GrpcPipelinePartConfiguration()
    append_uint32_value(config, const_tandem_array_length, self.max_loop_length)

    return GrpcPipelinePartBase(defaultPart=create_default_pipeline_part(const_discover_loops_strict, config))
