from .. import log_creators
from ..test_data_provider import *
from ...ficus.legacy.log.functions import read_log_from_xes, parse_log_from_string, parse_log_from_strings
from ...ficus.legacy.pipelines.mutations.mutations_parts import *
from ...ficus.legacy.pipelines.pipelines import Pipeline
from ...ficus.legacy.pipelines.start.start_parts import ReadLogFromXes, UseExistingLog
from ...ficus.legacy.util import concept_name


def test_remove_event_by_name_pipeline():
  result = Pipeline(
    ReadLogFromXes(),
    RemoveEventFromLogByName('A')
  )(get_example_log_path('exercise1.xes'))

  events = set(create_log_information(log(result)).events_count.keys())
  assert 'A' not in events
  assert 'B' in events
  assert 'C' in events
  assert 'D' in events
  assert 'E' in events


def test_remove_not_existing_event_by_name():
  initial_info = create_log_information(read_log_from_xes(get_example_log_path('exercise1.xes')))
  result = Pipeline(
    ReadLogFromXes(),
    RemoveEventFromLogByName('asdasd')
  )(get_example_log_path('exercise1.xes'))

  result_info = create_log_information(log(result))
  assert initial_info.events_count == result_info.events_count


def test_removing_last_event_by_name():
  existing_log = parse_log_from_string(log_creators.insert_separator('aaaaaaa'))
  result = Pipeline(
    UseExistingLog(),
    RemoveEventFromLogByName('a')
  )(existing_log)

  assert len(log(result)) == 0


def test_removing_all_events_by_name():
  existing_log = parse_log_from_strings([
    log_creators.insert_separator('aaaaaaa'),
    log_creators.insert_separator('aaaaaaa'),
    log_creators.insert_separator('aaaaaaa')
  ])

  result = Pipeline(
    UseExistingLog(),
    RemoveEventFromLogByName('a')
  )(existing_log)

  assert len(log(result)) == 0


def test_remove_events_by_name_from_empty_log():
  existing_log = MyEventLog()
  result = Pipeline(
    UseExistingLog(),
    RemoveEventFromLogByName('a'),
    RemoveEventFromLogByName('b'),
  )(existing_log)

  assert len(existing_log) == 0
  assert len(log(result)) == 0


def test_remove_set_of_events():
  result = Pipeline(
    ReadLogFromXes(),
    RemoveEventsFromLogByNames({'A', 'B'})
  )(get_example_log_path('exercise1.xes'))

  events = set(create_log_information(log(result)).events_count.keys())
  assert 'A' not in events
  assert 'B' not in events
  assert 'C' in events
  assert 'D' in events
  assert 'E' in events


def test_remove_set_of_events2():
  result = Pipeline(
    ReadLogFromXes(),
    RemoveEventsFromLogByNames({'A', 'not existing event'})
  )(get_example_log_path('exercise1.xes'))

  events = set(create_log_information(log(result)).events_count.keys())
  assert 'A' not in events


def test_remove_set_of_events_all_events():
  existing_log = parse_log_from_strings([
    log_creators.insert_separator('aacaaacaaccc'),
    log_creators.insert_separator('baababa'),
    log_creators.insert_separator('bababaa')
  ])

  result = Pipeline(
    UseExistingLog(),
    RemoveEventsFromLogByNames(set('abcd'))
  )(existing_log)

  assert len(log(result)) == 0


def test_remove_events_by_names_from_empty_log():
  empty_log = MyEventLog()
  result = Pipeline(
    UseExistingLog(),
    RemoveEventsFromLogByNames(set('abcd'))
  )(empty_log)

  assert len(log(result)) == 0
  assert len(empty_log) == 0


def test_remove_events_by_predicate():
  def predicate(event: MyEvent):
    return event[concept_name] is 'A' or event[concept_name] is 'D'

  result = Pipeline(
    ReadLogFromXes(),
    RemoveEventsFromLogPredicate(predicate)
  )(get_example_log_path('exercise1.xes'))

  events = set(create_log_information(log(result)).events_count.keys())

  assert 'A' not in events
  assert 'D' not in events
  assert 'B' in events
  assert 'C' in events
  assert 'E' in events


def test_remove_events_by_predicate2():
  def predicate(event: MyEvent):
    return not (event[concept_name] is 'A' or event[concept_name] is 'D')

  result = Pipeline(
    ReadLogFromXes(),
    RemoveEventsFromLogPredicate(predicate)
  )(get_example_log_path('exercise1.xes'))

  events = set(create_log_information(log(result)).events_count.keys())

  assert 'A' in events
  assert 'D' in events
  assert 'B' not in events
  assert 'C' not in events
  assert 'E' not in events


def test_remove_events_by_predicate_from_empty_log():
  def predicate(event: MyEvent):
    return True

  result = Pipeline(
    UseExistingLog(),
    RemoveEventsFromLogPredicate(predicate)
  )(MyEventLog())

  assert len(log(result)) == 0


def test_preprocess_events_names():
  def preprocessor(name):
    return f'{name}_{name}'

  result = Pipeline(
    ReadLogFromXes(),
    PreprocessEventsNames(preprocessor)
  )(get_example_log_path('exercise1.xes'))

  events = set(create_log_information(log(result)).events_count.keys())

  assert 'A' not in events
  assert 'B' not in events
  assert 'C' not in events
  assert 'D' not in events
  assert 'E' not in events

  assert 'A_A' in events
  assert 'B_B' in events
  assert 'C_C' in events
  assert 'D_D' in events
  assert 'E_E' in events


def test_preprocess_events_names_empty_log():
  def preprocessor(name):
    return f'{name}_{name}'

  result = Pipeline(
    UseExistingLog(),
    PreprocessEventsNames(preprocessor)
  )(MyEventLog())

  assert len(log(result)) == 0


def test_preprocess_events():
  def preprocessor(event):
    name = event[concept_name]
    event[concept_name] = f'{name}_{name}_{name}'

  result = Pipeline(
    ReadLogFromXes(),
    PreprocessEvents(preprocessor)
  )(get_example_log_path('exercise1.xes'))

  events = set(create_log_information(log(result)).events_count.keys())

  assert 'A' not in events
  assert 'B' not in events
  assert 'C' not in events
  assert 'D' not in events
  assert 'E' not in events

  assert 'A_A_A' in events
  assert 'B_B_B' in events
  assert 'C_C_C' in events
  assert 'D_D_D' in events
  assert 'E_E_E' in events


def test_preprocess_events_empty_log():
  def preprocessor(event):
    name = event[concept_name]
    event[concept_name] = f'{name}_{name}_{name}'

  result = Pipeline(
    UseExistingLog(),
    PreprocessEvents(preprocessor)
  )(MyEventLog())

  assert len(log(result)) == 0


def test_apply_class_extractor():
  def class_extractor(event: MyEvent):
    return event[concept_name].lower()

  result = Pipeline(
    ReadLogFromXes(),
    ApplyClassExtractor(class_extractor)
  )(get_example_log_path('exercise1.xes'))

  events = set(create_log_information(log(result)).events_count.keys())

  assert 'A' not in events
  assert 'B' not in events
  assert 'C' not in events
  assert 'D' not in events
  assert 'E' not in events

  assert 'a' in events
  assert 'b' in events
  assert 'c' in events
  assert 'd' in events
  assert 'e' in events


def test_apply_class_extractor_empty_log():
  def class_extractor(event: MyEvent):
    return event[concept_name].lower()

  result = Pipeline(
    UseExistingLog(),
    ApplyClassExtractor(class_extractor)
  )(MyEventLog())

  assert len(log(result)) == 0


def test_substitute_underlying_events():
  class NextUnderlyingEventsProvider:
    def __init__(self):
      self.next_underlying_event_index = 1

    def next(self):
      next_event = MyEvent()
      next_event[concept_name] = str(self.next_underlying_event_index)
      self.next_underlying_event_index += 1
      return next_event

  initial_log = parse_log_from_string(log_creators.insert_separator('aaaaaaa'))
  assert len(initial_log) == 1

  underlying_events_provider = NextUnderlyingEventsProvider()
  for trace in initial_log:
    for event in trace:
      event[underlying_events_key] = [underlying_events_provider.next() for _ in range(5)]

  result = Pipeline(
    UseExistingLog(),
    SubstituteUnderlyingEvents()
  )(initial_log)

  trace = log(result)[0]
  current_index = 1
  for event in trace:
    assert event[concept_name] == str(current_index)
    current_index += 1


def test_substitute_underlying_events2():
  log_path = get_example_log_path('exercise1.xes')
  initial_log = read_log_from_xes(log_path)

  result = Pipeline(
    ReadLogFromXes(),
    SubstituteUnderlyingEvents(),
  )(log_path)

  initial_trace = initial_log[0]
  result_trace = log(result)[0]

  for initial_event, result_event in zip(initial_trace, result_trace):
    assert initial_event[concept_name] == result_event[concept_name]


def test_removing_lifecycle_transition_attribute():
  result = Pipeline(
    ReadLogFromXes(),
    RemoveLifecycleTransitionsAttributes(),
  )(get_repair_example_path())

  for trace in log(result):
    for event in trace:
      assert lifecycle_transition not in event
