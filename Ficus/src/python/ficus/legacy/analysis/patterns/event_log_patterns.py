from typing import Optional

import intervaltree
from suffix_tree.tree import Tree

from .constants import *
from .patterns_models import TandemArrayInfo, MaximalRepeatInfo, SubArrayWithTraceIndex, ActivityNode, \
  ActivityInTraceInfo, UndefinedActivityHandlingStrategy, EventClassNode
from .util import _create_hashes_traces_with_selector, default_class_extractor, _split_activity_nodes_by_size, \
  _do_create_hashes, _find_events_for_repeat_set
from ..common.common_models import SubArrayInEventLog
from ...analysis.event_log_info import create_log_information
from ...analysis.type_aliases import *
from ...log.event_log import *
from ...util import concept_name, calculate_poly_hash_for_collection


def find_maximal_tandem_arrays(log: MyEventLog,
                               max_tandem_array_length=10,
                               event_class_extractor: ClassExtractor = None) -> list[list[TandemArrayInfo]]:
  traces = _create_hashes_traces_with_selector(log, event_class_extractor)
  return _find_maximal_tandem_arrays(traces, max_tandem_array_length)


def _find_maximal_tandem_arrays(traces: list[list[int]],
                                max_tandem_array_length=10) -> list[list[TandemArrayInfo]]:
  result = []
  for trace in traces:
    visited = set()
    tandem_arrays = []
    for length in range(2, max_tandem_array_length):
      for i in range(0, len(trace) - length):
        sub_array_hash = calculate_poly_hash_for_collection(trace, i, i + length)
        if sub_array_hash in visited:
          continue

        visited.add(sub_array_hash)
        candidate_tandem_array = _try_extract_tandem_array(trace, i, length)
        if candidate_tandem_array is not None:
          tandem_arrays.append(candidate_tandem_array)

    result.append(tandem_arrays)

  return result


def _try_extract_tandem_array(trace, start_index, length) -> Optional[TandemArrayInfo]:
  current_index = start_index + length
  number_of_repeats = 1

  while True:
    if current_index + length - 1 >= len(trace):
      break

    found_another_repeat = True
    for i in range(length):
      if trace[current_index + i] != trace[start_index + i]:
        found_another_repeat = False
        break

    if not found_another_repeat:
      break

    number_of_repeats += 1
    current_index += length

  if number_of_repeats > 1:
    return TandemArrayInfo(first_pos=start_index, repeat_count=number_of_repeats, length=length)

  return None


def find_primitive_tandem_arrays(log: MyEventLog,
                                 max_tandem_array_length=10,
                                 event_class_extractor: ClassExtractor = None) -> list[list[TandemArrayInfo]]:
  traces = _create_hashes_traces_with_selector(log, event_class_extractor)
  traces_arrays = _find_maximal_tandem_arrays(traces, max_tandem_array_length)
  primitive_arrays = []
  for trace_arrays, trace in zip(traces_arrays, traces):
    trace_primitive_arrays = []
    for array in trace_arrays:
      is_primitive_array = True
      for current_length in range(2, int((array.length + 1) / 2 + 1)):
        if _try_extract_tandem_array(trace, array.first_pos, current_length):
          is_primitive_array = False
          break

      if is_primitive_array:
        trace_primitive_arrays.append(array)

    primitive_arrays.append(trace_primitive_arrays)

  return primitive_arrays


def find_maximal_repeats(log: MyEventLog,
                         event_class_extractor: ClassExtractor = None) -> list[list[MaximalRepeatInfo]]:
  traces = _create_hashes_traces_with_selector(log, event_class_extractor)
  return _do_find_maximal_repeats(traces)


def _do_find_maximal_repeats(hash_traces: list[list[int]]) -> list[list[MaximalRepeatInfo]]:
  repeats_by_traces = []
  for trace in hash_traces:
    tree = Tree({'Trace': trace})
    repeats = tree.maximal_repeats()
    infos = [MaximalRepeatInfo(first_pos=r[1].start, length=r[1].end - r[1].start) for r in repeats]
    repeats_by_traces.append(infos)

  return repeats_by_traces


def find_super_maximal_repeats(log: MyEventLog,
                               event_class_extractor: ClassExtractor = None) -> list[list[MaximalRepeatInfo]]:
  traces = _create_hashes_traces_with_selector(log, event_class_extractor)
  return _do_find_super_maximal_repeats(traces)


def _do_find_super_maximal_repeats(traces: list[list[int]]) -> list[list[MaximalRepeatInfo]]:
  maximal_repeats_by_traces = _do_find_maximal_repeats(traces)
  super_maximal_repeats_by_traces = []
  for trace, trace_maximal_repeats in zip(traces, maximal_repeats_by_traces):
    index_to_repeats = dict()
    for index, repeat in enumerate(trace_maximal_repeats):
      index_to_repeats[index] = trace[repeat.first_pos:(repeat.first_pos + repeat.length)]

    tree = Tree(index_to_repeats)
    this_trace_super_max_repeats = []
    for index, sub_trace in index_to_repeats.items():
      if len(tree.find_all(sub_trace)) == 1:
        this_trace_super_max_repeats.append(trace_maximal_repeats[index])

    super_maximal_repeats_by_traces.append(this_trace_super_max_repeats)

  return super_maximal_repeats_by_traces


def create_repeat_sets(log: MyEventLog,
                       sub_arrays: list[list[SubArrayInEventLog]],
                       event_class_extractor: ClassExtractor = None) -> list[SubArrayWithTraceIndex]:
  hashes_traces = _create_hashes_traces_with_selector(log, event_class_extractor)
  return _do_create_repeat_sets(hashes_traces, sub_arrays)


def _do_create_repeat_sets(hashes_traces: list[list[int]],
                           sub_arrays: list[list[SubArrayInEventLog]]) -> list[SubArrayWithTraceIndex]:
  repeat_sets = dict()
  index = 0
  for trace_sub_arrays, trace in zip(sub_arrays, hashes_traces):
    for sub_array in trace_sub_arrays:
      start = sub_array.first_pos
      end = sub_array.first_pos + sub_array.length
      sub_array_from_log = list(sorted(set(trace[start:end])))
      sub_array_hash = calculate_poly_hash_for_collection(sub_array_from_log)

      if sub_array_hash not in repeat_sets:
        repeat_sets[sub_array_hash] = SubArrayWithTraceIndex(first_pos=start,
                                                             length=sub_array.length,
                                                             trace_index=index)
    index += 1

  return list(repeat_sets.values())


def build_repeat_set_tree(log: MyEventLog,
                          repeat_sets: list[SubArrayWithTraceIndex],
                          activity_names_creator: Callable[[SubArrayWithTraceIndex], str],
                          activity_level: int,
                          event_class_extractor: ClassExtractor = default_class_extractor) -> list[ActivityNode]:
  hash_traces = _create_hashes_traces_with_selector(log, event_class_extractor)
  return _do_build_repeat_set_tree(hash_traces,
                                   repeat_sets=repeat_sets,
                                   activity_names_creator=activity_names_creator,
                                   class_extractor=event_class_extractor,
                                   activity_level=activity_level)


def _do_build_repeat_set_tree(hashes_traces: list[list[int]],
                              repeat_sets: list[SubArrayWithTraceIndex],
                              activity_names_creator: Callable[[SubArrayWithTraceIndex], str],
                              class_extractor: ClassExtractor,
                              activity_level: int) -> list[ActivityNode]:
  def extract_set_events(r_set: SubArrayWithTraceIndex):
    trace = hashes_traces[r_set.trace_index]
    return set(trace[r_set.first_pos:(r_set.first_pos + r_set.length)])

  def create_activity_node(r_set: SubArrayWithTraceIndex):
    events = extract_set_events(r_set)
    return ActivityNode(name=activity_names_creator(r_set),
                        repeat_set=r_set,
                        set_of_events=events,
                        class_extractor=class_extractor,
                        activity_level=activity_level)

  if len(repeat_sets) == 0:
    return []

  activity_nodes = sorted(list(map(create_activity_node, repeat_sets)), key=lambda x: x.length, reverse=True)
  max_length = activity_nodes[0].length
  current_length = max_length
  top_level_nodes = [activity_nodes[0]]

  next_length_index = 1
  for i in range(1, len(activity_nodes)):
    activity_node = activity_nodes[i]
    if activity_node.length != max_length:
      next_length_index = i
      current_length = activity_node.length
      break

    top_level_nodes.append(activity_node)

  if len(top_level_nodes) == len(activity_nodes):
    return top_level_nodes

  nodes_by_level = [[]]

  for i in range(next_length_index, len(activity_nodes)):
    current_activity_node = activity_nodes[i]
    if current_activity_node.length < current_length:
      current_length = current_activity_node.length
      nodes_by_level.append([])

    found_any_match = False
    for level_index in range(len(nodes_by_level) - 1, -1, -1):
      for activity_node in nodes_by_level[level_index]:
        if activity_node.contains_other(current_activity_node) and activity_node != current_activity_node:
          activity_node.child_nodes.append(current_activity_node)
          found_any_match = True
          break

      if found_any_match:
        break

    if not found_any_match:
      for top_level_node in top_level_nodes:
        if top_level_node.contains_other(current_activity_node) and top_level_node != current_activity_node:
          top_level_node.child_nodes.append(current_activity_node)
          found_any_match = True
          break

    nodes_by_level[-1].append(current_activity_node)
    if not found_any_match:
      top_level_nodes.append(current_activity_node)

  return top_level_nodes


def find_near_super_maximal_repeats(log: MyEventLog,
                                    event_class_extractor: ClassExtractor = None) -> list[list[MaximalRepeatInfo]]:
  hashes_traces = _create_hashes_traces_with_selector(log, event_class_extractor)
  return _do_find_near_super_maximal_repeats(hashes_traces)


def _do_find_near_super_maximal_repeats(hashes_traces: list[list[int]]):
  maximal_repeats_by_traces = _do_find_maximal_repeats(hashes_traces)
  near_super_maximal_repeats = []

  for hash_trace, maximal_repeats_by_trace in zip(hashes_traces, maximal_repeats_by_traces):
    tree = Tree({'Tree': hash_trace})
    near_super_maximal_repeats_for_trace = set()
    max_repeat_count = [0 for _ in range(len(maximal_repeats_by_trace))]
    interval_tree = intervaltree.IntervalTree()

    for idx, repeat in enumerate(maximal_repeats_by_trace):
      repeat_poses = tree.find_all(hash_trace[repeat.first_pos:(repeat.first_pos + repeat.length)])
      for repeat_pos in repeat_poses:
        start = repeat_pos[1].start
        end = start + repeat.length
        interval_tree.addi(start, end, idx)

      max_repeat_count[idx] = len(repeat_poses)

    visited = set()
    all_intervals = sorted(list(interval_tree.all_intervals), key=lambda x: x.end - x.begin + 1, reverse=True)
    for interval in all_intervals:
      if interval in visited:
        continue

      visited.add(interval)
      near_super_maximal_repeats_for_trace.add(interval.data)
      for envelope in interval_tree.envelop(interval.begin, interval.end):
        visited.add(envelope)

    lst = list(map(lambda i: maximal_repeats_by_trace[i], near_super_maximal_repeats_for_trace))
    near_super_maximal_repeats.append(lst)

  return near_super_maximal_repeats


ActivityInTraceFilter = Callable[[ActivityInTraceInfo], bool]


def default_activity_in_trace_filter(_: ActivityInTraceInfo) -> bool:
  return True


def extract_activities_from_log(log: MyEventLog,
                                activities_nodes: list[ActivityNode],
                                event_class_extractor: ClassExtractor = None,
                                activity_filter: ActivityInTraceFilter = default_activity_in_trace_filter,
                                should_narrow_activity: bool = True) -> list[list[ActivityInTraceInfo]]:
  hashes_traces = _create_hashes_traces_with_selector(log, event_class_extractor)
  return _do_extract_activities_from_log(hashes_traces, activities_nodes, should_narrow_activity, activity_filter)


def _do_extract_activities_from_log(hashes_traces: list[list[int]],
                                    activities_nodes: list[ActivityNode],
                                    should_narrow_activity: bool = True,
                                    activity_filter: ActivityInTraceFilter = default_activity_in_trace_filter) -> list[
  list[ActivityInTraceInfo]]:
  activities_by_size = _split_activity_nodes_by_size(activities_nodes)
  result = []

  for trace in hashes_traces:
    current_trace_activities = []
    index = -1
    current_activity = None
    last_activity_start_index = -1
    current_set = set()

    while index < len(trace):
      index += 1
      if index >= len(trace):
        break

      event_hash = trace[index]
      if current_activity is None:
        found_activity = False
        for activities in activities_by_size:
          for activity in activities:
            if event_hash in activity.set_of_events:
              current_activity = activity
              last_activity_start_index = index
              found_activity = True
              break

          if found_activity:
            current_set.clear()
            current_set.add(event_hash)
            break

        continue

      if event_hash not in current_activity.set_of_events:
        new_set = current_set.copy()
        new_set.add(event_hash)
        found_new_set = False
        for activities_set in activities_by_size:
          if len(activities_set) == 0 or activities_set[0].length < current_activity.length:
            continue

          for activity in activities_set:
            if new_set.issubset(activity.set_of_events):
              current_activity = activity
              found_new_set = True
              break

          if found_new_set:
            current_set.add(event_hash)
            break

        if not found_new_set:
          if should_narrow_activity:
            current_activity = narrow_activity(current_activity, current_set)

          info = ActivityInTraceInfo(node=current_activity,
                                     start_pos=last_activity_start_index,
                                     length=index - last_activity_start_index)

          if activity_filter(info):
            current_trace_activities.append(info)

          current_activity = None
          current_set.clear()
          last_activity_start_index = -1
          index -= 1
      else:
        current_set.add(event_hash)

    if last_activity_start_index != -1:
      if should_narrow_activity:
        current_activity = narrow_activity(current_activity, current_set)

      info = ActivityInTraceInfo(node=current_activity,
                                 start_pos=last_activity_start_index,
                                 length=index - last_activity_start_index)

      if activity_filter(info):
        current_trace_activities.append(info)

    result.append(current_trace_activities)

  return result


def narrow_activity(node: ActivityNode, activities_set: set[int]) -> ActivityNode:
  q = []
  for child in node.child_nodes:
    q.append(child)

  result = []
  while len(q) != 0:
    current_activity = q[0]
    q.pop(0)
    if current_activity.set_of_events.issuperset(activities_set):
      result.append(current_activity)
      for child_node in current_activity.child_nodes:
        q.append(child_node)

  if len(result) == 0:
    return node

  result = sorted(result, key=lambda x: x.length)
  return result[0]


def _process_trace_activities(trace: MyTrace,
                              activities: list[ActivityInTraceInfo],
                              undefined_activity_func: Callable[[int, int], None],
                              activity_func: Callable[[ActivityInTraceInfo], None]):
  index = 0
  for activity in activities:
    if index < activity.start_pos:
      undefined_activity_func(index, activity.start_pos)

    activity_func(activity)
    index = activity.start_pos + activity.length

  if index < len(trace):
    undefined_activity_func(index, len(trace))


def create_new_log_from_activities(log: MyEventLog,
                                   activities: list[list[ActivityInTraceInfo]],
                                   use_hashes_as_names: bool = True,
                                   strategy=UndefinedActivityHandlingStrategy.InsertAsSingleEvent) -> MyEventLog:
  new_log = MyEventLog()

  for trace_activities, trace in zip(activities, log):
    resulting_events = []

    def undefined_activity_func(start_index_of_undefined_activity, start_index_of_next_activity):
      if strategy == UndefinedActivityHandlingStrategy.InsertAsSingleEvent:
        new_event = MyEvent()
        new_event[concept_name] = undefined_activity
        resulting_events.append(new_event)
      elif strategy == UndefinedActivityHandlingStrategy.InsertAllEvents:
        for i in range(start_index_of_undefined_activity, start_index_of_next_activity):
          resulting_events.append(trace[i])
      elif strategy == UndefinedActivityHandlingStrategy.DontInsert:
        pass
      else:
        raise ValueError()

    def activity_func(activity: ActivityInTraceInfo):
      new_event = MyEvent()
      if use_hashes_as_names:
        new_event[concept_name] = activity.node.unique_name()
      else:
        new_event[concept_name] = activity.node.name

      underlying_events = trace[activity.start_pos:(activity.start_pos + activity.length)]
      new_event[underlying_events_key] = list(map(copy.copy, underlying_events))

      def get_underlying_activities(current_event: MyEvent):
        return current_event[underlying_activities_key] if underlying_activities_key in current_event else []

      underlying_activities = [x for evt in underlying_events for x in get_underlying_activities(evt)]
      underlying_activities.append(activity.node)

      new_event[underlying_activities_key] = underlying_activities
      resulting_events.append(new_event)

    _process_trace_activities(trace, trace_activities, undefined_activity_func, activity_func)

    new_trace = MyTrace()
    for event in resulting_events:
      new_trace.append(event)

    new_log.append(new_trace)

  return new_log


def full_fill_activities(log: MyEventLog,
                         activities: list[list[ActivityInTraceInfo]],
                         activities_creator: Callable[[list[MyEvent]], list[ActivityInTraceInfo]]) -> list[
  list[ActivityInTraceInfo]]:
  new_activities = []
  for trace_activities, trace in zip(activities, log):
    current_new_activities = trace_activities.copy()

    def process_undefined_activity(start_index, end_index):
      events = trace[start_index:end_index]
      current_new_activities.extend(activities_creator(events))

    def process_activity(_: ActivityInTraceInfo):
      pass

    _process_trace_activities(trace, trace_activities, process_undefined_activity, process_activity)

    current_new_activities = sorted(current_new_activities, key=lambda x: x.start_pos, reverse=True)
    new_activities.append(current_new_activities)

  return new_activities


def create_unattached_events_log(log: MyEventLog,
                                 activities_in_traces: list[list[ActivityInTraceInfo]]) -> MyEventLog:
  result_log = MyEventLog()
  for trace_activities, trace in zip(activities_in_traces, log):
    new_trace = MyTrace()

    def process_undefined_activity(start_index, end_index):
      for event in trace[start_index:end_index]:
        new_trace.append(event)

    def process_activity(_):
      pass

    _process_trace_activities(trace, trace_activities, process_undefined_activity, process_activity)
    result_log.append(new_trace)

  return result_log


def add_unattached_activities(log: MyEventLog,
                              unattached_activities: list[ActivityNode],
                              existing_activities: list[list[ActivityInTraceInfo]],
                              class_extractor: ClassExtractor,
                              activities_in_trace_filter: ActivityInTraceFilter = default_activity_in_trace_filter,
                              min_number_of_events: int = 1,
                              should_narrow_activity: bool = True) -> list[list[ActivityInTraceInfo]]:
  new_activities_list = []
  for trace_activities, trace in zip(existing_activities, log):
    new_activities_for_trace = []

    def process_undefined_events(start_index: int, end_index: int):
      def adjust_activity(activity_in_trace: ActivityInTraceInfo) -> ActivityInTraceInfo:
        return ActivityInTraceInfo(activity_in_trace.node,
                                   start_index + activity_in_trace.start_pos,
                                   activity_in_trace.length)

      hashes = _do_create_hashes(trace[start_index:end_index], class_extractor=class_extractor)
      if len(hashes) < min_number_of_events:
        return

      new_activities = _do_extract_activities_from_log([hashes],
                                                       unattached_activities,
                                                       activity_filter=activities_in_trace_filter,
                                                       should_narrow_activity=should_narrow_activity)[0]

      new_activities_for_trace.extend(list(map(adjust_activity, new_activities)))

    def process_activity(_):
      pass

    _process_trace_activities(trace, trace_activities, process_undefined_events, process_activity)
    new_activities_for_trace.extend(trace_activities)
    new_activities_list.append(list(sorted(new_activities_for_trace, key=lambda x: x.start_pos)))

  return new_activities_list


def build_class_tree(log: MyEventLog,
                     class_extractors: list[Callable[[str], str]]) -> list[EventClassNode]:
  initial_set_of_names = set(create_log_information(log).events_count.keys())
  first_layer_of_nodes = list(map(lambda name: EventClassNode(name), initial_set_of_names))
  nodes_by_level = [first_layer_of_nodes]

  for extractor in class_extractors:
    names_to_new_nodes = dict()
    for prev_layer_node in nodes_by_level[-1]:
      node_name = extractor(prev_layer_node.name)
      if node_name in names_to_new_nodes:
        names_to_new_nodes[node_name].child_nodes.append(prev_layer_node)
      else:
        node = EventClassNode(node_name)
        node.child_nodes.append(prev_layer_node)
        names_to_new_nodes[node_name] = node

    nodes_by_level.append(list(names_to_new_nodes.values()))

  return nodes_by_level[-1]


def create_logs_for_activities(log: MyEventLog,
                               activities: list[list[ActivityInTraceInfo]],
                               activity_level: int,
                               class_extractor: ClassExtractor) -> dict[str, MyEventLog]:
  def trace_creator(_: MyEventLog, trace: MyTrace, info: ActivityInTraceInfo) -> Optional[MyTrace]:
    if info.node.activity_level != activity_level:
      return None

    return _create_trace_for_activity(trace, info, class_extractor)

  return _create_logs_for_activities(log, activities, trace_creator)


def _create_trace_for_activity(original_trace: MyTrace,
                               info: ActivityInTraceInfo,
                               class_extractor: ClassExtractor) -> MyTrace:
  new_activity_trace = MyTrace()
  for event in original_trace[info.start_pos:(info.start_pos + info.length)]:
    event_copy = copy.copy(event)
    event_copy[concept_name] = class_extractor(event_copy)
    new_activity_trace.append(event_copy)

  return new_activity_trace


TraceCreator = Callable[[MyEventLog, MyTrace, 'ActivityInTraceInfo'], Optional[MyTrace]]


def _create_logs_for_activities(log: MyEventLog,
                                activities: list[list[ActivityInTraceInfo]],
                                trace_creator: TraceCreator) -> dict[str, MyEventLog]:
  activities_to_logs = dict()
  for trace_activities, trace in zip(activities, log):
    def handle_activity(info: ActivityInTraceInfo):
      new_activity_trace = trace_creator(log, trace, info)
      if new_activity_trace is None:
        return

      name = info.node.name
      if name in activities_to_logs:
        activities_to_logs[name].append(new_activity_trace)
      else:
        new_log = MyEventLog()
        new_log.append(new_activity_trace)
        activities_to_logs[name] = new_log

    _process_trace_activities(trace, trace_activities, lambda x, y: None, handle_activity)

  return activities_to_logs


def create_logs_for_activities_with_promote(log: MyEventLog,
                                            activities: list[list[ActivityInTraceInfo]],
                                            desired_activity_level: int,
                                            class_extractors: list[ClassExtractor],
                                            adjust_events_to_max_level: bool = True) -> dict[str, MyEventLog]:
  assert len(class_extractors) == desired_activity_level + 1

  def trace_creator(log: MyEventLog, trace: MyTrace, info: ActivityInTraceInfo) -> Optional[MyTrace]:
    level = info.node.activity_level
    if level > desired_activity_level:
      return None

    if level == desired_activity_level:
      extractor = class_extractors[desired_activity_level] if adjust_events_to_max_level else default_class_extractor
      return _create_trace_for_activity(trace, info, extractor)

    repeat_events = _find_events_for_repeat_set(log, info.node.repeat_set)
    this_activity_events_orig = trace[info.start_pos:(info.start_pos + info.length)]
    this_activity_events_copy = list(map(copy.copy, this_activity_events_orig))
    for extractor in class_extractors[level:(desired_activity_level + 1)]:
      if level > desired_activity_level:
        break

      if level >= info.node.activity_level:
        for event in this_activity_events_copy:
          event[concept_name] = extractor(event)

    repeat_event_classes_set = set(map(class_extractors[desired_activity_level], repeat_events))
    this_activity_event_classes_set = set(map(lambda x: x[concept_name], this_activity_events_copy))

    if repeat_event_classes_set == this_activity_event_classes_set:
      new_trace = MyTrace()
      for event in this_activity_events_orig if not adjust_events_to_max_level else this_activity_events_copy:
        new_trace.append(event)

      return new_trace

    return None

  return _create_logs_for_activities(log, activities, trace_creator)


def calculate_underlying_events_count(log: MyEventLog):
  count = 0
  for trace in log:
    for event in trace:
      count += count_underlying_events(event)

  return count


def execute_with_underlying_events(top_level_event: MyEvent, action: Callable[[MyEvent], None]):
  q = [top_level_event]
  while len(q) != 0:
    current_event = q.pop(0)
    if underlying_events_key in current_event:
      for underlying_event in current_event[underlying_events_key]:
        q.append(underlying_event)
    else:
      action(current_event)


def count_underlying_events(event: MyEvent):
  count = 0
  q = [event]
  while len(q) != 0:
    current_event = q.pop(0)
    if underlying_events_key in current_event:
      for underlying_event in current_event[underlying_events_key]:
        q.append(underlying_event)
    else:
      count += 1

  return count


def add_all_patterns(log: list[list[int]], patterns: list[list[SubArrayInEventLog]]) -> list[list[SubArrayInEventLog]]:
  all_log_patterns = []
  for trace, trace_patterns in zip(log, patterns):
    tree_dict = dict()
    for index, pattern in enumerate(trace_patterns):
      start = pattern.first_pos
      end = start + pattern.length
      tree_dict[str(index)] = trace[start:end]

    tree = Tree(tree_dict)
    all_trace_patterns = []
    index = 1
    current_start = 0

    while index < len(trace):
      if not tree.find(trace[current_start:index]):
        all_trace_patterns.append(SubArrayInEventLog(first_pos=current_start, length=index - current_start))
        current_start = index

      index += 1

    all_log_patterns.append(all_trace_patterns)

  return all_log_patterns
