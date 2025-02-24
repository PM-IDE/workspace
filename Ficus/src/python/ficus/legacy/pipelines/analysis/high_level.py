from .patterns.models import AdjustingMode
from ...pipelines.analysis.patterns.patterns_parts import *
from ...pipelines.pipelines import *


class DiscoverActivitiesBase(InternalPipelinePart):
  def __init__(self,
               activity_level: int,
               activities_discovery_strategy=ActivitiesDiscoveryStrategy.DiscoverFromAllTraces,
               debug_prints: bool = False,
               class_extractor: Callable[[MyEvent], str] = default_class_extractor,
               activity_in_trace_filter: ActivityInTraceFilter = default_activity_in_trace_filter,
               activity_name_creator: ActivityNameCreator = default_activity_name_creator,
               should_narrow_activity: bool = True):
    self.activities_discovery_strategy = activities_discovery_strategy
    self.activity_level = activity_level
    self.debug_prints = debug_prints
    self.class_extractor = class_extractor
    self.activity_in_trace_filter = activity_in_trace_filter
    self.activity_name_creator = activity_name_creator
    self.should_narrow_activity = should_narrow_activity

  def execute(self, current_input: PipelinePartResult) -> PipelinePartResult:
    raise NotImplementedError()


class DiscoverActivitiesFromTandemArrays(DiscoverActivitiesBase):
  def __init__(self,
               array_kind: TandemArrayKind,
               max_loop_length: int = 20,
               activity_level: int = 0,
               debug_prints: bool = False,
               class_extractor: Callable[[MyEvent], str] = default_class_extractor,
               activity_in_trace_filter: ActivityInTraceFilter = default_activity_in_trace_filter,
               activity_name_creator: ActivityNameCreator = default_activity_name_creator,
               should_narrow_activity: bool = True):
    super().__init__(activity_level,
                     ActivitiesDiscoveryStrategy.DiscoverFromAllTraces,
                     debug_prints,
                     class_extractor,
                     activity_in_trace_filter,
                     activity_name_creator,
                     should_narrow_activity)

    self.max_loop_length = max_loop_length
    self.array_kind = array_kind

  def execute(self, current_input: PipelinePartResult) -> PipelinePartResult:
    return Pipeline(
      DiscoverActivitiesFromPatterns(
        DiscoverTandemArrays(self.array_kind, self.max_loop_length, self.class_extractor),
        class_extractor=self.class_extractor,
        activity_level=self.activity_level,
        activities_discovery_strategy=self.activities_discovery_strategy,
        activity_name_creator=self.activity_name_creator
      ),
      PrintActivities(self.debug_prints),
      DiscoverActivitiesInTraces(self.class_extractor,
                                 activity_in_trace_filter=self.activity_in_trace_filter),
    )(current_input)


class DiscoverActivitiesInstancesFromRepeats(DiscoverActivitiesBase):
  def __init__(self,
               repeat_kind: RepeatActivitiesSource,
               activity_level: int = 0,
               activities_discovery_strategy=ActivitiesDiscoveryStrategy.DiscoverFromAllTraces,
               debug_prints: bool = False,
               class_extractor: Callable[[MyEvent], str] = default_class_extractor,
               activity_in_trace_filter: ActivityInTraceFilter = default_activity_in_trace_filter,
               activity_name_creator: ActivityNameCreator = default_activity_name_creator,
               should_narrow_activity: bool = True):
    super().__init__(activity_level,
                     activities_discovery_strategy,
                     debug_prints,
                     class_extractor,
                     activity_in_trace_filter,
                     activity_name_creator,
                     should_narrow_activity)

    self.repeat_kind = repeat_kind

  def execute(self, current_input: PipelinePartResult) -> PipelinePartResult:
    return Pipeline(
      DiscoverActivitiesFromPatterns(
        DiscoverRepeats(self.repeat_kind, self.class_extractor),
        class_extractor=self.class_extractor,
        activity_level=self.activity_level,
        activities_discovery_strategy=self.activities_discovery_strategy,
        activity_name_creator=self.activity_name_creator,
      ),
      PrintActivities(self.debug_prints),
      DiscoverActivitiesInTraces(self.class_extractor,
                                 activity_in_trace_filter=self.activity_in_trace_filter),
    )(current_input)


class AdjustWithActivitiesFromUnattachedEvents(InternalPipelinePart):
  def __init__(self,
               class_extractor: Callable[[MyEvent], str],
               activity_level: int,
               activities_discovery_strategy=ActivitiesDiscoveryStrategy.DiscoverFromAllTraces,
               min_events_in_trace: int = 1,
               adjusting_mode: AdjustingMode = AdjustingMode.FromUnattachedSubTraces,
               activities_in_trace_filter: ActivityInTraceFilter = default_activity_in_trace_filter,
               should_narrow_activity: bool = True):
    self.activity_level = activity_level
    self.class_extractor = class_extractor
    self.min_events_in_trace = min_events_in_trace
    self.adjusting_mode = adjusting_mode
    self.activities_discovery_strategy = activities_discovery_strategy
    self.activites_in_trace_filter = activities_in_trace_filter
    self.should_narrow_activity = should_narrow_activity

  def execute(self, current_input: PipelinePartResult) -> PipelinePartResult:
    def input_transformer(initial_input: PipelinePartResult) -> PipelinePartResult:
      has_unattached_events = initial_input.has_value(traces_activities_key)
      if has_unattached_events and self.adjusting_mode == AdjustingMode.FromUnattachedSubTraces:
        current_log = log(initial_input)
        current_traces_activities = traces_activities(initial_input)
        unattached_events_log = create_unattached_events_log(current_log, current_traces_activities)
      elif self.adjusting_mode == AdjustingMode.FromAllLog:
        unattached_events_log = log(initial_input)
      else:
        raise ValueError()

      return PipelinePartResult().with_log(unattached_events_log)

    def output_merger(initial_input: PipelinePartResult, temp_output: PipelinePartResult) -> PipelinePartResult:
      old_activities = activities(initial_input) if initial_input.has_value(activities_key) else []
      old_activities.extend(activities(temp_output))
      return initial_input.with_activities(old_activities)

    if not current_input.has_value(traces_activities_key):
      current_input = current_input.with_trace_activities([[] for _ in log(current_input)])

    return Pipeline(
      WithTempInput(input_transformer, output_merger, Pipeline(
        DiscoverActivitiesFromPatterns(
          DiscoverRepeats(repeat_kind=RepeatActivitiesSource.MaximalRepeats,
                          class_extractor=self.class_extractor),
          class_extractor=self.class_extractor,
          activity_level=self.activity_level,
          activities_discovery_strategy=self.activities_discovery_strategy,
        )
      )),
      DiscoverActivitiesInUnattachedSubTraces(class_extractor=self.class_extractor,
                                              min_numbers_of_events=self.min_events_in_trace,
                                              activities_in_trace_filter=self.activites_in_trace_filter,
                                              should_narrow_activity=self.should_narrow_activity)
    )(current_input)


class DiscoverActivitiesForSeveralLevels(InternalPipelinePart):
  def __init__(self,
               class_extractors: list[Callable[[MyEvent], str]],
               discovering_strategy=ActivitiesDiscoveryStrategy.DiscoverFromSingleMergedTrace,
               initial_activity_level: int = 0,
               min_events_in_traces: int = 1,
               adjusting_mode: AdjustingMode = AdjustingMode.FromAllLog,
               activity_in_trace_filter: ActivityInTraceFilter = default_activity_in_trace_filter,
               should_narrow_activity: bool = True):
    self.class_extractors = class_extractors
    self.initial_activity_level = initial_activity_level
    self.min_events_in_traces = min_events_in_traces
    self.adjusting_mode = adjusting_mode
    self.discovering_strategy = discovering_strategy
    self.activity_in_trace_filter = activity_in_trace_filter
    self.should_narrow_activity = should_narrow_activity

  def execute(self, current_input: PipelinePartResult) -> PipelinePartResult:
    result = current_input
    for index, class_extractor in enumerate(self.class_extractors):
      part = AdjustWithActivitiesFromUnattachedEvents(class_extractor=class_extractor,
                                                      activity_level=self.initial_activity_level + index,
                                                      min_events_in_trace=self.min_events_in_traces,
                                                      adjusting_mode=self.adjusting_mode,
                                                      activities_discovery_strategy=self.discovering_strategy,
                                                      activities_in_trace_filter=self.activity_in_trace_filter,
                                                      should_narrow_activity=self.should_narrow_activity)
      result = part(result)

    return result


class ExecuteWithEachActivityLog(InternalPipelinePart):
  def __init__(self, pipeline: Pipeline):
    self.pipeline = pipeline

  def execute(self, current_input: PipelinePartResult) -> PipelinePartResult:
    for activity_name, activity_log in activities_to_logs(current_input).items():
      self.pipeline(PipelinePartResult().with_log(activity_log).with_activity_name(activity_name))

    return current_input


class DiscoverActivitiesInstancesFromRepeatsUntilNoMore(InternalPipelinePart):
  def __init__(self,
               class_extractor: ClassExtractor,
               use_hashes_as_event_name: bool = True,
               discovery_strategy=ActivitiesDiscoveryStrategy.DiscoverFromAllTraces,
               min_events_in_trace: int = 1,
               activity_in_trace_filter: ActivityInTraceFilter = default_activity_in_trace_filter,
               should_narrow_activity: bool = True,
               initial_activity_level: int = 0,
               increase_activity_level: bool = False,
               before_log_creation_pipeline: Pipeline = None,
               after_log_creation_pipeline: Pipeline = None):
    self.after_log_creation_pipeline = after_log_creation_pipeline
    self.before_log_creation_pipeline = before_log_creation_pipeline
    self.initial_activity_level = initial_activity_level
    self.increase_activity_level = increase_activity_level
    self.should_narrow_activity = should_narrow_activity
    self.activity_in_trace_filter = activity_in_trace_filter
    self.min_events_in_trace = min_events_in_trace
    self.class_extractor = class_extractor
    self.use_hashes_as_event_names = use_hashes_as_event_name
    self.discovery_strategy = discovery_strategy

  def execute(self, current_input: PipelinePartResult) -> PipelinePartResult:
    log_size = calculate_events_count(log(current_input))
    activity_level = self.initial_activity_level

    while True:
      prev_input = current_input
      current_input = Pipeline(
        ClearActivities(),
        DiscoverActivitiesInstancesFromRepeats(RepeatActivitiesSource.MaximalRepeats,
                                               activity_level=activity_level,
                                               class_extractor=self.class_extractor,
                                               activity_in_trace_filter=self.activity_in_trace_filter,
                                               activities_discovery_strategy=self.discovery_strategy,
                                               should_narrow_activity=self.should_narrow_activity)
      )(current_input)

      if sum(map(len, traces_activities(current_input))) == 0:
        return prev_input

      if self.before_log_creation_pipeline is not None:
        self.before_log_creation_pipeline(current_input)

      current_input = Pipeline(
        CreateLogFromActivities(use_hashes_as_names=self.use_hashes_as_event_names)
      )(current_input)

      if self.after_log_creation_pipeline is not None:
        self.after_log_creation_pipeline(current_input)

      if self.increase_activity_level:
        activity_level += 1

      new_log_size = calculate_events_count(log(current_input))

      if new_log_size == log_size:
        return current_input

      log_size = new_log_size
