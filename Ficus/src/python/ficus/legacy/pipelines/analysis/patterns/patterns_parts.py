from ....analysis.patterns.util import _do_create_hashes_traces
from ....pipelines.serialization.pipeline_parts import SavePathCreator

from .type_aliases import GraphAttributesSetter
from ...common import InternalDrawingPipelinePart
from ....analysis.event_log_info import calculate_events_count
from ....analysis.event_log_split import merge_all_traces_in_one_log
from ....analysis.patterns.patterns_graphs import draw_short_activity_diagram, draw_full_activity_diagram, \
  default_graph_attr_setter, draw_activity_graph, draw_activity_placement_diagram, draw_patterns
from ....analysis.patterns.util import *
from ....pipelines.analysis.patterns.models import TandemArrayKind
from ....pipelines.contexts.accessors import *
from ....pipelines.contexts.part_results import *
from ....pipelines.pipelines import *


class DiscoverTandemArrays(InternalPipelinePart):
  def __init__(self,
               array_kind: TandemArrayKind,
               max_loop_length: int = 20,
               class_extractor: Callable[[MyEvent], str] = None):
    self.array_kind = array_kind
    self.max_loop_length = max_loop_length
    self.class_extractor = class_extractor if class_extractor is not None else default_class_extractor

  def execute(self, current_input: PipelinePartResult) -> PipelinePartResult:
    arrays_kinds = {
      TandemArrayKind.PrimitiveArray: find_primitive_tandem_arrays,
      TandemArrayKind.MaximalArray: find_maximal_tandem_arrays
    }

    arrays = arrays_kinds[self.array_kind](log(current_input),
                                           max_tandem_array_length=self.max_loop_length,
                                           event_class_extractor=self.class_extractor)

    return current_input.with_patterns(arrays)


class AddAllPatterns(InternalPipelinePart):
  def __init__(self, class_extractor: Callable[[MyEvent], str] = None):
    self.class_extractor = class_extractor

  def execute(self, current_input: PipelinePartResult) -> PipelinePartResult:
    hashed_log = _do_create_hashes_traces(log(current_input))
    all_patterns = add_all_patterns(hashed_log, patterns(current_input))

    return current_input.with_patterns(all_patterns)


class DiscoverRepeatsSets(InternalPipelinePart):
  def __init__(self, class_extractor: Callable[[MyEvent], str] = None):
    self.class_extractor = class_extractor if class_extractor is not None else default_class_extractor

  def execute(self, current_input: PipelinePartResult) -> PipelinePartResult:
    repeat_sets = create_repeat_sets(log(current_input),
                                     patterns(current_input),
                                     event_class_extractor=self.class_extractor)

    return current_input.with_repeat_sets(repeat_sets)


ActivityNameCreator = Callable[[SubArrayWithTraceIndex, MyEventLog, ClassExtractor], str]


def default_activity_name_creator(sub_array: SubArrayWithTraceIndex,
                                  current_log: MyEventLog,
                                  class_extractor: ClassExtractor) -> str:
  return create_activity_name_from_log(current_log, sub_array, class_extractor)


class DiscoverActivities(InternalPipelinePart):
  def __init__(self,
               activity_level: int,
               class_extractor: ClassExtractor = None,
               activity_name_creator: ActivityNameCreator = default_activity_name_creator):
    self.activity_level = activity_level
    self.class_extractor = class_extractor if class_extractor is not None else default_class_extractor
    self.activity_name_creator = activity_name_creator

  def execute(self, current_input: PipelinePartResult) -> PipelinePartResult:
    current_log = log(current_input)

    def create_activity_name(sub_array: SubArrayWithTraceIndex) -> str:
      return self.activity_name_creator(sub_array, current_log, self.class_extractor)

    activities = build_repeat_set_tree(current_log,
                                       repeat_sets(current_input),
                                       activity_level=self.activity_level,
                                       activity_names_creator=create_activity_name,
                                       event_class_extractor=self.class_extractor)

    return current_input.with_activities(activities)


class DiscoverActivitiesInTraces(InternalPipelinePart):
  def __init__(self,
               class_extractor: Callable[[MyEvent], str] = default_class_extractor,
               activity_in_trace_filter: ActivityInTraceFilter = default_activity_in_trace_filter,
               should_narrow_activity: bool = True):
    self.class_extractor = class_extractor
    self.activity_in_trace_filter = activity_in_trace_filter
    self.should_narrow_activity = should_narrow_activity

  def execute(self, current_input: PipelinePartResult) -> PipelinePartResult:
    traces_activities = extract_activities_from_log(log(current_input),
                                                    activities(current_input),
                                                    event_class_extractor=self.class_extractor,
                                                    activity_filter=self.activity_in_trace_filter,
                                                    should_narrow_activity=self.should_narrow_activity)

    print(f"Discovered {sum(map(len, traces_activities))} activities instances")
    return current_input.with_trace_activities(traces_activities)


class CreateLogFromActivities(InternalPipelinePart):
  def __init__(self,
               strategy: UndefinedActivityHandlingStrategy = UndefinedActivityHandlingStrategy.InsertAllEvents,
               use_hashes_as_names: bool = True):
    self.strategy = strategy
    self.use_hashes_as_names = use_hashes_as_names

  def execute(self, current_input: PipelinePartResult) -> PipelinePartResult:
    new_log = create_new_log_from_activities(log(current_input),
                                             traces_activities(current_input),
                                             use_hashes_as_names=self.use_hashes_as_names,
                                             strategy=self.strategy)

    return current_input.with_log(new_log)


class DiscoverRepeats(InternalPipelinePart):
  def __init__(self,
               repeat_kind: RepeatActivitiesSource,
               class_extractor: Callable[[MyEvent], str] = None):
    self.repeat_kind = repeat_kind
    self.class_extractor = class_extractor if class_extractor is not None else default_class_extractor

  def execute(self, current_input: PipelinePartResult) -> PipelinePartResult:
    repeat_functions = {
      RepeatActivitiesSource.MaximalRepeats: find_maximal_repeats,
      RepeatActivitiesSource.SuperMaximalRepeats: find_super_maximal_repeats,
      RepeatActivitiesSource.NearSuperMaximalRepeats: find_near_super_maximal_repeats
    }

    repeats = repeat_functions[self.repeat_kind](log(current_input), self.class_extractor)
    return current_input.with_patterns(repeats)


class PrintActivities(InternalPipelinePart):
  def __init__(self, do_print: bool = True):
    self.do_print = do_print

  def execute(self, current_input: PipelinePartResult) -> PipelinePartResult:
    if not self.do_print:
      return current_input

    print('Found activities:')
    for activity_name in list(map(lambda x: x.name, activities(current_input))):
      print(activity_name)
      print()

    return current_input


class PrintUnattachedEvents(InternalPipelinePart):
  def execute(self, current_input: PipelinePartResult) -> PipelinePartResult:
    unattached_events_log = create_unattached_events_log(log(current_input), traces_activities(current_input))
    names = set()
    for trace in unattached_events_log:
      for event in trace:
        names.add(event[concept_name])

    print(list(names))
    return current_input


class CalculatePercentageOfUnattachedEvents(InternalPipelinePart):
  def execute(self, current_input: PipelinePartResult) -> PipelinePartResult:
    events_log = create_unattached_events_log(log(current_input), traces_activities(current_input))
    unattached_events_count = sum(map(len, events_log))
    events_count = calculate_events_count(log(current_input))
    if events_count == 0:
      print('Log is empty, can not calculate unattached events percentage')
      return current_input

    percentage = unattached_events_count / events_count
    print(f'Percentage of unattached events: {percentage}')
    return current_input


class DrawActivitiesDiagram(InternalDrawingPipelinePart):
  def __init__(self,
               short_diagram: bool,
               title: str = None,
               plot_legend: bool = False,
               height_scale: int = 1,
               width_scale: int = 1,
               save_path: Optional[Union[str, SavePathCreator]] = None):
    super().__init__(title, plot_legend, height_scale, width_scale, save_path)
    self.short_diagram = short_diagram

  def execute(self, current_input: PipelinePartResult) -> PipelinePartResult:
    draw_func = draw_short_activity_diagram if self.short_diagram else draw_full_activity_diagram
    draw_func(log(current_input),
              traces_activities(current_input),
              cached_colors(current_input),
              title=self.title,
              plot_legend=self.plot_legend,
              save_path=self._get_save_path(current_input),
              height_scale=self.height_scale,
              width_scale=self.width_scale)

    return current_input


class DrawPatterns(InternalDrawingPipelinePart):
  def __init__(self,
               short_diagram: bool,
               title: str = None,
               plot_legend: bool = False,
               height_scale: int = 1,
               width_scale: int = 1,
               save_path: Optional[Union[str, SavePathCreator]] = None):
    super().__init__(title, plot_legend, height_scale, width_scale, save_path)
    self.short_diagram = short_diagram

  def execute(self, current_input: PipelinePartResult) -> PipelinePartResult:
    draw_patterns(log(current_input),
                  patterns(current_input),
                  cached_colors(current_input),
                  title=self.title,
                  plot_legend=self.plot_legend,
                  save_path=self._get_save_path(current_input),
                  height_scale=self.height_scale,
                  width_scale=self.width_scale,
                  short_diagram=self.short_diagram)

    return current_input


class DrawShortActivitiesDiagram(DrawActivitiesDiagram):
  def __init__(self,
               title: str = None,
               plot_legend: bool = False,
               height_scale: int = 1,
               width_scale: int = 1,
               save_path: Optional[Union[str, SavePathCreator]] = None):
    super().__init__(True,
                     title=title,
                     plot_legend=plot_legend,
                     height_scale=height_scale,
                     width_scale=width_scale,
                     save_path=save_path)


class DrawFullActivitiesDiagram(DrawActivitiesDiagram):
  def __init__(self,
               title: str = None,
               plot_legend: bool = False,
               height_scale: int = 1,
               width_scale: int = 1,
               save_path: Optional[Union[str, SavePathCreator]] = None):
    super().__init__(False,
                     title=title,
                     plot_legend=plot_legend,
                     height_scale=height_scale,
                     width_scale=width_scale,
                     save_path=save_path)


class DiscoverActivitiesInUnattachedSubTraces(InternalPipelinePart):
  def __init__(self,
               class_extractor: Callable[[MyEvent], str],
               min_numbers_of_events: int = 1,
               activities_in_trace_filter: ActivityInTraceFilter = default_activity_in_trace_filter,
               should_narrow_activity: bool = True):
    self.class_extractor = class_extractor
    self.min_numbers_of_events = min_numbers_of_events
    self.activities_in_trace_filter = activities_in_trace_filter
    self.should_narrow_activity = should_narrow_activity

  def execute(self, current_input: PipelinePartResult) -> PipelinePartResult:
    activities_in_traces = add_unattached_activities(log(current_input),
                                                     activities(current_input),
                                                     traces_activities(current_input),
                                                     class_extractor=self.class_extractor,
                                                     min_number_of_events=self.min_numbers_of_events,
                                                     activities_in_trace_filter=self.activities_in_trace_filter,
                                                     should_narrow_activity=self.should_narrow_activity)

    print(f"Discovered {sum(map(len, activities_in_traces))} activities instances")
    return current_input.with_trace_activities(activities_in_traces)


class InternalGraphDrawingPart(InternalPipelinePart):
  def __init__(self,
               save_path: str = None,
               add_root_node: bool = True,
               graph_attributes_setter: GraphAttributesSetter = default_graph_attr_setter):
    self.save_path = save_path
    self.add_root_node = add_root_node
    self.graph_attributes_setter = graph_attributes_setter

  def execute(self, current_input: PipelinePartResult) -> PipelinePartResult:
    raise NotImplementedError()


class DrawActivitiesGraph(InternalGraphDrawingPart):
  def __init__(self,
               save_path: str = None,
               use_hashes_as_name: bool = True,
               add_root_node: bool = True,
               graph_attributes_setter: GraphAttributesSetter = default_graph_attr_setter):
    super().__init__(save_path=save_path,
                     add_root_node=add_root_node,
                     graph_attributes_setter=graph_attributes_setter)
    self.use_hashes_as_name = use_hashes_as_name

  def execute(self, current_input: PipelinePartResult) -> PipelinePartResult:
    draw_activity_graph(activities(current_input),
                        save_path=self.save_path,
                        use_hashes_as_name=self.use_hashes_as_name,
                        add_root_node=self.add_root_node,
                        set_attributes_to_func=self.graph_attributes_setter)

    return current_input


class PrintActivitiesNamesToHashes(InternalPipelinePart):
  def execute(self, current_input: PipelinePartResult) -> PipelinePartResult:
    import queue
    q = queue.Queue()
    for top_level_activity in activities(current_input):
      q.put(top_level_activity)

    while not q.empty():
      current_activity = q.get()
      print(f'{hash(current_activity)} = {current_activity.name}')
      for child in current_activity.child_nodes:
        q.put(child)

    return current_input


class DrawSingleActivityPlacement(InternalDrawingPipelinePart):
  def __init__(self,
               activity_selector: Callable[[list[ActivityNode]], ActivityNode],
               use_different_colors: bool = True,
               title: str = None,
               plot_legend: bool = False,
               height_scale: int = 1,
               width_scale: int = 1,
               save_path: Union[str, SavePathCreator] = None):
    super().__init__(title, plot_legend, height_scale, width_scale, save_path)
    self.activity_selector = activity_selector
    self.use_different_colors = use_different_colors

  def execute(self, current_input: PipelinePartResult) -> PipelinePartResult:
    draw_activity_placement_diagram(log(current_input),
                                    self.activity_selector(activities(current_input)),
                                    traces_activities(current_input),
                                    use_different_colors=self.use_different_colors,
                                    plot_legend=self.plot_legend,
                                    height_scale=self.height_scale,
                                    title=self.title,
                                    save_path=self._get_save_path(current_input))

    return current_input


class CreateLogsForActivities(InternalPipelinePart):
  def __init__(self, activity_level: int, class_extractor: Callable[[MyEvent], str]):
    self.class_extractor = class_extractor
    self.activity_level = activity_level

  def execute(self, current_input: PipelinePartResult) -> PipelinePartResult:
    logs = create_logs_for_activities(log=log(current_input),
                                      activities=traces_activities(current_input),
                                      activity_level=self.activity_level,
                                      class_extractor=self.class_extractor)

    return current_input.with_activities_logs(logs)


class CreateLogsForActivitiesWithPromote(InternalPipelinePart):
  def __init__(self,
               desired_activity_level: int,
               class_extractors: list[ClassExtractor],
               adjust_events_to_max_level: bool = True):
    self.desired_activity_level = desired_activity_level
    self.class_extractors = class_extractors
    self.adjust_events_to_max_level = adjust_events_to_max_level

  def execute(self, current_input: PipelinePartResult) -> PipelinePartResult:
    logs = create_logs_for_activities_with_promote(log=log(current_input),
                                                   activities=traces_activities(current_input),
                                                   desired_activity_level=self.desired_activity_level,
                                                   class_extractors=self.class_extractors,
                                                   adjust_events_to_max_level=self.adjust_events_to_max_level)

    return current_input.with_activities_logs(logs)


class ClearActivities(InternalPipelinePart):
  def execute(self, current_input: PipelinePartResult) -> PipelinePartResult:
    def empty_list():
      return list(map(lambda x: [], log(current_input)))

    current_input.with_activities([]).with_trace_activities(empty_list())
    return current_input.with_repeat_sets([]).with_patterns(empty_list())


class ActivitiesDiscoveryStrategy:
  DiscoverFromAllTraces = 0
  DiscoverFromSingleMergedTrace = 1


class DiscoverActivitiesFromPatterns(InternalPipelinePart):
  def __init__(self,
               patterns_source: PipelinePart,
               activities_discovery_strategy=ActivitiesDiscoveryStrategy.DiscoverFromSingleMergedTrace,
               class_extractor: Callable[[MyEvent], str] = default_class_extractor,
               activity_level: int = 0,
               activity_name_creator: ActivityNameCreator = default_activity_name_creator):
    self.patterns_source = patterns_source
    self.class_extractor = class_extractor
    self.activity_level = activity_level
    self.activities_discovery_strategy = activities_discovery_strategy
    self.activity_name_creator = activity_name_creator

  def execute(self, current_input: PipelinePartResult) -> PipelinePartResult:
    def input_transformer(initial_input: PipelinePartResult):
      if self.activities_discovery_strategy == ActivitiesDiscoveryStrategy.DiscoverFromSingleMergedTrace:
        return copy.copy(initial_input).with_log(merge_all_traces_in_one_log(log(initial_input)))

      return copy.copy(initial_input)

    def output_merger(initial_input: PipelinePartResult, temp_result: PipelinePartResult):
      return (copy.copy(initial_input)
              .with_activities(activities(temp_result))
              .with_patterns(patterns(temp_result)))

    return WithTempInput(input_transformer, output_merger, Pipeline(
      self.patterns_source,
      DiscoverRepeatsSets(self.class_extractor),
      DiscoverActivities(activity_level=self.activity_level,
                         class_extractor=self.class_extractor,
                         activity_name_creator=self.activity_name_creator),
      PrintFoundActivitiesCount()
    ))(current_input)


class PrintFoundActivitiesCount(InternalPipelinePart):
  def execute(self, current_input: PipelinePartResult) -> PipelinePartResult:
    print(f"Discovered {len(activities(current_input))} activities")
    return current_input


class SaveAllActivitiesNames(InternalPipelinePart):
  def __init__(self, save_path: str):
    self.save_path = save_path

  def execute(self, current_input: PipelinePartResult) -> PipelinePartResult:
    activities_by_level = split_activities_by_level(activities(current_input))
    with open(self.save_path, 'w') as fout:
      sep_string = 4 * '================================================='
      for level, activities_for_level in activities_by_level:
        fout.write(f'ACTIVITIES LEVEL {level}\n\n')
        fout.write(sep_string)
        fout.write('\n')

        visited = set()
        q = [root for root in activities_for_level]
        while len(q) > 0:
          current_activity = q.pop()
          if current_activity.name in visited:
            continue

          visited.add(current_activity.name)
          fout.write(f'{current_activity.name}\n')

          for child in current_activity.child_nodes:
            q.append(child)

        fout.write(sep_string)
        fout.write('\n\n')

    return current_input


class PrintNumberOfUnderlyingEvents(InternalPipelinePart):
  def execute(self, current_input: PipelinePartResult) -> PipelinePartResult:
    print(f'Underlying events count: {calculate_underlying_events_count(log(current_input))}')
    return current_input


class SaveAllActivitiesInstancesNames(InternalPipelinePart):
  def __init__(self, save_path: Union[str, Callable[[], str]]):
    self.save_path = save_path

  def execute(self, current_input: PipelinePartResult) -> PipelinePartResult:
    path = self.save_path if type(self.save_path) == str else self.save_path()

    with open(path, 'w') as fout:
      visited = set()
      for trace in traces_activities(current_input):
        for activity in trace:
          node = activity.node
          if node in visited:
            continue

          fout.write(f'{node.unique_name()}={node.name}\n')
          visited.add(node)

    return current_input


class WriteActivitiesInstancesIndices(InternalPipelinePart):
  def __init__(self, save_path: str):
    self.save_path = save_path

  def execute(self, current_input: PipelinePartResult) -> PipelinePartResult:
    instances = traces_activities(current_input)

    with open(self.save_path, 'w') as fout:
      trace_index = 0
      for trace_activities in instances:
        fout.write(f"Trace {trace_index}\n")
        for instance in trace_activities:
          fout.write(f'({instance.node.unique_name()}, {instance.start_pos}, {instance.length})\n')

        fout.write('\n')
        trace_index += 1

    return current_input


class AddActivityNameToAllEvents(InternalPipelinePart):
  def execute(self, current_input: PipelinePartResult) -> PipelinePartResult:
    instances = traces_activities(current_input)
    event_log = log(current_input)
    for trace_activities, trace in zip(instances, event_log.traces):
      for activity in trace_activities:
        for i in range(activity.start_pos, activity.start_pos + activity.length):
          trace[i]['activity'] = activity.node.unique_name()

    return current_input
