import copy

from pm4py.objects.log.obj import EventLog, Event, Trace
from .event_log import MyEventLog, MyTrace, MyEvent


def from_pm4py_log(log: EventLog) -> MyEventLog:
  my_log = MyEventLog()
  my_log.attributes = copy.deepcopy(log.attributes)
  my_log.properties = copy.deepcopy(log.properties)
  my_log.classifiers = copy.deepcopy(log.classifiers)
  my_log.extensions = copy.deepcopy(log.extensions)
  my_log.global_values = copy.deepcopy(log.omni_present)

  for trace in log:
    my_trace = MyTrace()
    my_trace.attributes = copy.deepcopy(trace.attributes)

    for event in trace:
      my_trace.append(pm4py_event_to_my_event(event))

    my_log.append(my_trace)

  return my_log


def pm4py_event_to_my_event(event: Event) -> MyEvent:
  return MyEvent(copy.deepcopy(event._dict))


def my_event_to_pm4py_event(my_event: MyEvent):
  event = Event()
  for key, value in my_event.payload_items():
    event[copy.deepcopy(key)] = copy.deepcopy(value)

  return event


def to_pm4py_log(my_log: MyEventLog) -> EventLog:
  log = EventLog()
  for my_trace in my_log:
    trace = Trace()
    for my_event in my_trace:
      trace.append(my_event_to_pm4py_event(my_event))

    log.append(trace)

  return log
