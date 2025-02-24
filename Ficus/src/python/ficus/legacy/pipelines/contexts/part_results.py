from dataclasses import dataclass
from enum import Enum
from typing import Any

import graphviz
from pm4py.objects.petri_net.obj import PetriNet, Marking

from ...analysis.patterns.event_log_patterns import ActivityNode, SubArrayWithTraceIndex, ActivityInTraceInfo, \
  SubArrayInEventLog, \
  EventClassNode
from ...log.event_log import MyEventLog
from ...pipelines.contexts.keys import *


class RepeatActivitiesSource(Enum):
  MaximalRepeats = 0
  SuperMaximalRepeats = 1
  NearSuperMaximalRepeats = 2


@dataclass
class PetriNetWrapper:
  petri_net: PetriNet
  start_marking: Marking
  end_marking: Marking


@dataclass
class ActivitiesContext:
  activities: list[ActivityNode]
  hashes_to_activities: dict[int, ActivityNode]


class PipelinePartResult:
  def __init__(self):
    self.values = dict()

  def with_log(self, log: MyEventLog) -> 'PipelinePartResult':
    self.values[log_key] = log
    return self

  def with_activities(self, activities: list[ActivityNode]) -> 'PipelinePartResult':
    self.values[activities_key] = activities
    return self

  def with_repeat_sets(self, repeat_sets: list[SubArrayWithTraceIndex]) -> 'PipelinePartResult':
    self.values[repeat_sets_key] = repeat_sets
    return self

  def with_trace_activities(self, trace_activities: list[list[ActivityInTraceInfo]]) -> 'PipelinePartResult':
    self.values[traces_activities_key] = trace_activities
    return self

  def with_patterns(self, patterns: list[list[SubArrayInEventLog]]) -> 'PipelinePartResult':
    self.values[patterns_key] = patterns
    return self

  def with_petri_net(self, petri_net: PetriNetWrapper):
    self.values[petri_net_key] = petri_net
    return self

  def with_event_class_tree(self, nodes: list[EventClassNode]):
    self.values[event_class_tree_key] = nodes
    return self

  def with_activities_logs(self, activities_to_logs: dict[str, MyEventLog]):
    self.values[activities_to_logs_key] = activities_to_logs
    return self

  def with_serialized_graph(self, serialized_graph: str):
    self.values[serialized_graph_key] = serialized_graph
    return self

  def with_graph(self, graph: graphviz.Digraph):
    self.values[graph_key] = graph
    return self

  def with_activity_name(self, activity_name: str):
    self.values[activity_name_key] = activity_name
    return self

  def with_cached_colors(self, colors: dict[str, str]):
    self.values[cached_colors_key] = colors
    return self

  def with_custom_data(self, key: str, data: Any):
    self.values[key] = data
    return self

  def has_value(self, key):
    return key in self.values

  def get_value_or_throw(self, key):
    if not self.has_value(key):
      raise ValueError()

    return self.values[key]

  def remove(self, key):
    del self.values[key]

  def __log__(self) -> MyEventLog:
    return self.get_value_or_throw(log_key)

  def __activities__(self) -> list[ActivityNode]:
    return self.get_value_or_throw(activities_key)

  def __repeat_sets__(self) -> list[SubArrayWithTraceIndex]:
    return self.get_value_or_throw(repeat_sets_key)

  def __traces_activities__(self) -> list[list[ActivityInTraceInfo]]:
    return self.get_value_or_throw(traces_activities_key)

  def __petri_net__(self) -> PetriNetWrapper:
    return self.get_value_or_throw(petri_net_key)

  def __patterns__(self) -> list[list[SubArrayInEventLog]]:
    return self.get_value_or_throw(patterns_key)

  def __event_class_tree__(self) -> list[EventClassNode]:
    return self.get_value_or_throw(event_class_tree_key)

  def __activities_to_logs__(self) -> dict[str, MyEventLog]:
    return self.get_value_or_throw(activities_to_logs_key)

  def __serialized_graph__(self) -> str:
    return self.get_value_or_throw(serialized_graph_key)

  def __graph__(self) -> graphviz.Digraph:
    return self.get_value_or_throw(graph_key)

  def __activity_name__(self) -> str:
    return self.get_value_or_throw(activity_name_key)

  def __cached_colors__(self) -> dict[str, str]:
    if cached_colors_key not in self.values:
      self.values[cached_colors_key] = dict()

    return self.get_value_or_throw(cached_colors_key)

  def __copy__(self):
    new_obj = PipelinePartResult()
    new_obj.values = dict(self.values)
    return new_obj

  def __deepcopy__(self, memodict={}):
    raise NotImplementedError()
