from typing import Callable

from ..log.event_log import MyEventLog, MyEvent, MyTrace
from ..util import concept_name, lifecycle_transition


def remove_event_from_log(log: MyEventLog, event_name: str) -> MyEventLog:
  return _filter_events(log, lambda evt: evt[concept_name] != event_name)


def _filter_events(log: MyEventLog, select_func: Callable[[MyEvent], bool]) -> MyEventLog:
  new_log = MyEventLog()
  for trace in log:
    new_trace = MyTrace()
    for event in trace:
      if select_func(event):
        new_trace.append(event)

    if len(new_trace) > 0:
      new_log.append(new_trace)

  return new_log


def _filter_traces(log: MyEventLog, select_func: Callable[[MyTrace], bool]) -> MyEventLog:
  new_log = MyEventLog()
  for trace in log:
    if select_func(trace):
      new_log.append(trace)

  return new_log


def remove_events_from_log(log: MyEventLog, events_names: set[str]) -> MyEventLog:
  return _filter_events(log, lambda evt: evt[concept_name] not in events_names)


def preprocess_event_names_inplace(log: MyEventLog, name_mutator: Callable[[str], str]):
  for trace in log:
    for event in trace:
      event[concept_name] = name_mutator(event[concept_name])


def preprocess_events(log: MyEventLog, event_mutator: Callable[[MyEvent], None]):
  for trace in log:
    for event in trace:
      event_mutator(event)


def filter_log(log: MyEventLog, predicate: Callable[[MyEvent], bool]) -> MyEventLog:
  fixed_predicate = lambda evt: not predicate(evt)
  return _filter_events(log, fixed_predicate)


def remove_empty_traces(log: MyEventLog, min_event_count: int = 0) -> MyEventLog:
  return _filter_traces(log, lambda trace: len(trace) > min_event_count)


def remove_lifecycle_attribute(log: MyEventLog):
  def process_event(event: MyEvent):
    if lifecycle_transition in event:
      del event[lifecycle_transition]

  preprocess_events(log, process_event)
