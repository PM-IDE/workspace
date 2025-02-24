import graphviz

from .part_results import PetriNetWrapper
from ...analysis.patterns.event_log_patterns import *


def log(obj) -> MyEventLog:
  return obj.__log__()


def activities(obj) -> list[ActivityNode]:
  return obj.__activities__()


def repeat_sets(obj) -> list[SubArrayWithTraceIndex]:
  return obj.__repeat_sets__()


def traces_activities(obj) -> list[list[ActivityInTraceInfo]]:
  return obj.__traces_activities__()


def patterns(obj) -> list[list[SubArrayInEventLog]]:
  return obj.__patterns__()


def petri_net(obj) -> PetriNetWrapper:
  return obj.__petri_net__()


def event_class_tree(obj) -> list[EventClassNode]:
  return obj.__event_class_tree__()


def activities_to_logs(obj) -> dict[str, MyEventLog]:
  return obj.__activities_to_logs__()


def serialized_graph(obj) -> str:
  return obj.__serialized_graph__()


def graph(obj) -> graphviz.Digraph:
  return obj.__graph__()


def activity_name(obj) -> str:
  return obj.__activity_name__()


def cached_colors(obj) -> dict[str, str]:
  return obj.__cached_colors__()
