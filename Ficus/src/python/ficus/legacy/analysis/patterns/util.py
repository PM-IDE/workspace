from .constants import activity_name_sep, underlying_events_key
from ..common.common_models import SubArrayInEventLog
from ..type_aliases import ClassExtractor
from ...analysis.patterns.patterns_models import ActivityInTraceInfo, SubArrayWithTraceIndex, ActivityNode
from ...log.event_log import *
from ...util import concept_name, calculate_string_poly_hash


def _create_activity_name(log: MyEventLog, activity: ActivityInTraceInfo) -> str:
  names = sub_array_with_event_index_to_event_names(log, activity.node.repeat_set)
  return ', '.join(names)


def default_class_extractor(event: MyEvent) -> str:
  return event[concept_name]


def sub_array_with_event_index_to_event_names(log: MyEventLog,
                                              repeat_set: SubArrayWithTraceIndex,
                                              class_extractor: ClassExtractor = default_class_extractor) -> list[str]:
  trace = log[repeat_set.trace_index]
  events = set()
  for i in range(repeat_set.first_pos, repeat_set.first_pos + repeat_set.length):
    events.add(class_extractor(trace[i]))

  return list(events)


def convert_sub_arrays_to_event_names(log: MyEventLog,
                                      sub_arrays: list[SubArrayWithTraceIndex]) -> list[list[str]]:
  result = []
  for array in sub_arrays:
    current_names_from_set = set()
    trace = log[array.trace_index]
    for i in range(array.first_pos, array.first_pos + array.length):
      current_names_from_set.add(trace[i][concept_name])

    result.append(list(current_names_from_set))

  return result


def _create_hashes_traces(log: MyEventLog) -> list[list[int]]:
  return _do_create_hashes_traces(log)


def _do_create_hashes_traces(log: MyEventLog,
                             event_class_extractor: ClassExtractor = None):
  def calculate_event_hash(event: MyEvent):
    if event_class_extractor is not None:
      return calculate_string_poly_hash(event_class_extractor(event))

    return calculate_string_poly_hash(event[concept_name])

  traces = []
  for trace in log:
    traces.append([calculate_event_hash(event) for event in trace])

  return traces


def _do_create_hashes(events: list[MyEvent],
                      class_extractor: ClassExtractor = default_class_extractor):
  return list(map(calculate_string_poly_hash, list(map(class_extractor, events))))


def _create_hashes_traces_with_selector(log: MyEventLog,
                                        event_class_extractor: ClassExtractor = None) -> list[list[int]]:
  return _do_create_hashes_traces(log, event_class_extractor)


def _create_hashes_traces_with_decode(log: MyEventLog) -> (list[list[int]], dict[int, str]):
  traces_hashes = []
  hashes_to_names = {}
  for trace in log:
    trace_hashes = []
    for event in trace:
      name = event[concept_name]
      name_hash = calculate_string_poly_hash(name)
      hashes_to_names[name_hash] = name
      trace_hashes.append(name_hash)

    traces_hashes.append(trace_hashes)

  return traces_hashes, hashes_to_names


def convert_log_sub_arrays_to_event_names(log: MyEventLog,
                                          trace_repeats: list[list[SubArrayInEventLog]]) -> list[list[list[str]]]:
  transformed_repeats = []
  for repeats, trace in zip(trace_repeats, log):
    traces_repeats = []
    for repeat in repeats:
      current_repeat = []
      for i in range(repeat.first_pos, repeat.first_pos + repeat.length):
        current_repeat.append(trace[i][concept_name])
      traces_repeats.append(current_repeat)

    transformed_repeats.append(traces_repeats)

  return transformed_repeats


def create_activity_name_from_log(log: MyEventLog,
                                  sub_array: SubArrayWithTraceIndex,
                                  class_selector: ClassExtractor) -> str:
  return create_activity_name_from_trace(log[sub_array.trace_index], sub_array, class_selector)


def create_activity_name_from_trace(trace: MyTrace,
                                    sub_array: SubArrayWithTraceIndex,
                                    class_extractor: ClassExtractor) -> str:
  names = list(sorted(set(map(class_extractor, trace[sub_array.first_pos:(sub_array.first_pos + sub_array.length)]))))
  return activity_name_sep.join(names)


def _split_activity_nodes_by_size(activities_nodes: list[ActivityNode]) -> list[list[ActivityNode]]:
  if len(activities_nodes) == 0:
    return []

  activities_nodes = sorted(activities_nodes, key=lambda x: x.length)
  current_length = activities_nodes[0].length
  activities_sets_by_size = [[activities_nodes[0]]]
  for i in range(1, len(activities_nodes)):
    if activities_nodes[i].length != current_length:
      activities_sets_by_size.append([])
      current_length = activities_nodes[i].length

    activities_sets_by_size[-1].append(activities_nodes[i])

  for i in range(len(activities_sets_by_size)):
    activities_sets_by_size[i] = sorted(activities_sets_by_size[i], key=lambda x: x.name)

  return activities_sets_by_size


def substitute_original_events(events: MyTrace) -> MyTrace:
  new_list = []
  for event in events:
    if underlying_events_key in event:
      new_list.extend(substitute_original_events(event[underlying_events_key]))
    else:
      new_list.append(event)

  trace = MyTrace()
  for event in new_list:
    trace.append(event)

  return trace


def _find_events_for_repeat_set(log: MyEventLog, sub_array: SubArrayWithTraceIndex) -> list[MyEvent]:
  trace = log[sub_array.trace_index]
  return trace[sub_array.first_pos:(sub_array.first_pos + sub_array.length)]


def split_activities_by_levels_dict(activities: list[ActivityNode]) -> dict[int, list[ActivityNode]]:
  activities_by_levels = dict()
  for activity in activities:
    if activity.activity_level in activities_by_levels:
      activities_by_levels[activity.activity_level].append(activity)
    else:
      activities_by_levels[activity.activity_level] = [activity]

  return activities_by_levels


def split_activities_by_level(activities: list[ActivityNode]) -> list[(int, list[ActivityNode])]:
  activities_by_levels = split_activities_by_levels_dict(activities)
  return sorted(list(activities_by_levels.items()), key=lambda x: x[0])


def get_activities_for_level(activities: list[ActivityNode], activity_level: int):
  return split_activities_by_levels_dict(activities)[activity_level]


def select_traces_activities_for_activity_level(traces_activities: list[list[ActivityInTraceInfo]],
                                                activity_level: int):
  result = []
  for trace_activities in traces_activities:
    result.append(list(filter(lambda x: x.node.activity_level == activity_level, trace_activities)))

  return result
