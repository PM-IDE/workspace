from typing import Callable

from .. import log_creators
from ..test_data_provider import get_example_log_path
from ...ficus.legacy.analysis.event_log_info import create_log_information
from ...ficus.legacy.log.functions import read_log_from_xes, parse_log_from_strings
from ...ficus.legacy.pipelines.contexts.accessors import log
from ...ficus.legacy.pipelines.filtering.filter_parts import PosEntropyDirectFilter, PosEntropyIndirectFilter, \
  DfgEntropyDirectFilter, DfgEntropyIndirectFilter, FilterTracesByCount, FilterTracesByVariants
from ...ficus.legacy.pipelines.pipelines import Pipeline
from ...ficus.legacy.pipelines.start.start_parts import ReadLogFromXes, UseExistingLog


def test_pos_entropy_direct_filter():
  result = Pipeline(
    ReadLogFromXes(),
    PosEntropyDirectFilter(threshold=0),
  )(get_example_log_path('exercise1.xes'))

  assert len(log(result)) == 3


def test_pos_entropy_direct_filter_with_max_events():
  max_events_to_remove = 2
  _do_direct_filter_test_with_max_events(max_events_to_remove, Pipeline(
    ReadLogFromXes(),
    PosEntropyDirectFilter(threshold=0, max_events_to_remove=max_events_to_remove)
  ))


def _do_direct_filter_test_with_max_events(max_events_to_remove: int, pipeline: Pipeline):
  log_path = get_example_log_path('exercise1.xes')
  initial_log = read_log_from_xes(log_path)
  max_events_to_remove = 2
  result = pipeline(log_path)

  new_count = len(create_log_information(log(result)).events_count.keys())
  old_count = len(create_log_information(initial_log).events_count.keys())
  assert old_count - max_events_to_remove == new_count


def test_pos_entropy_direct_filter_order():
  _do_test_direct_filter_order(['E', 'C', 'D', 'B', 'A'], lambda i: Pipeline(
    UseExistingLog(),
    PosEntropyDirectFilter(threshold=-1, max_events_to_remove=i)
  ))


def _do_test_direct_filter_order(expected_order: list[str],
                                 pipeline_creator: Callable[[int], Pipeline]):
  initial_log = read_log_from_xes(get_example_log_path('exercise1.xes'))
  log_info = create_log_information(initial_log)
  order = []
  for i in range(1, len(log_info.events_count) + 1):
    result = pipeline_creator(i)(initial_log)

    new_log_info = create_log_information(log(result))
    old_events = set(log_info.events_count.keys())
    new_events = set(new_log_info.events_count.keys())
    order_events = set(order)
    removed = old_events - new_events
    removed_event = list((removed - order_events))[0]
    order.append(removed_event)

  assert order == expected_order


def test_pos_entropy_indirect_filter():
  result = Pipeline(
    ReadLogFromXes(),
    PosEntropyIndirectFilter(threshold=0),
  )(get_example_log_path('exercise1.xes'))

  assert len(log(result)) == 3


def test_pos_entropy_indirect_filter_max_events():
  max_events_to_remove = 2
  _do_direct_filter_test_with_max_events(max_events_to_remove, Pipeline(
    ReadLogFromXes(),
    PosEntropyIndirectFilter(threshold=0, max_events_to_remove=max_events_to_remove)
  ))


def test_pos_entropy_indirect_filter_order():
  _do_test_direct_filter_order(['C', 'E', 'B', 'A', 'D'], lambda i: Pipeline(
    UseExistingLog(),
    PosEntropyIndirectFilter(threshold=-1, max_events_to_remove=i)
  ))


def test_dfg_entropy_direct_filter_laplace():
  result = Pipeline(
    ReadLogFromXes(),
    DfgEntropyDirectFilter(threshold=0, laplace=True),
  )(get_example_log_path('exercise1.xes'))

  assert len(log(result)) == 0


def test_dfg_entropy_direct_filter_non_laplace():
  result = Pipeline(
    ReadLogFromXes(),
    DfgEntropyDirectFilter(threshold=0, laplace=False),
  )(get_example_log_path('exercise1.xes'))

  assert len(log(result)) == 3


def test_dfg_entropy_direct_filter_max_events_laplace():
  max_events_to_remove = 2
  _do_direct_filter_test_with_max_events(max_events_to_remove, Pipeline(
    ReadLogFromXes(),
    DfgEntropyDirectFilter(threshold=0,
                           laplace=True,
                           max_events_to_remove=max_events_to_remove)
  ))


def test_dfg_entropy_direct_filter_max_events_non_laplace():
  max_events_to_remove = 2
  _do_direct_filter_test_with_max_events(max_events_to_remove, Pipeline(
    ReadLogFromXes(),
    DfgEntropyDirectFilter(threshold=0,
                           laplace=False,
                           max_events_to_remove=max_events_to_remove)
  ))


def test_dfg_entropy_direct_filter_order_laplace():
  _do_test_direct_filter_order(['E', 'C', 'B', 'A', 'D'], lambda i: Pipeline(
    UseExistingLog(),
    DfgEntropyDirectFilter(threshold=-1, laplace=True, max_events_to_remove=i)
  ))


def test_dfg_entropy_direct_filter_order_non_laplace():
  _do_test_direct_filter_order(['C', 'A', 'D', 'E', 'B'], lambda i: Pipeline(
    UseExistingLog(),
    DfgEntropyDirectFilter(threshold=-1, laplace=False, max_events_to_remove=i)
  ))


def test_dfg_entropy_indirect_filter_laplace():
  result = Pipeline(
    ReadLogFromXes(),
    DfgEntropyIndirectFilter(threshold=0, laplace=True, top_percent=1),
  )(get_example_log_path('exercise1.xes'))

  assert len(log(result)) == 0


def test_dfg_entropy_indirect_filter_non_laplace():
  result = Pipeline(
    ReadLogFromXes(),
    DfgEntropyIndirectFilter(threshold=0, laplace=False, top_percent=1),
  )(get_example_log_path('exercise1.xes'))

  assert len(log(result)) == 3


def test_dfg_entropy_indirect_filter_max_events_laplace():
  max_events_to_remove = 2
  _do_direct_filter_test_with_max_events(max_events_to_remove, Pipeline(
    ReadLogFromXes(),
    DfgEntropyIndirectFilter(threshold=0,
                             laplace=True,
                             top_percent=1,
                             max_events_to_remove=max_events_to_remove)
  ))


def test_dfg_entropy_indirect_filter_max_events_non_laplace():
  max_events_to_remove = 2
  _do_direct_filter_test_with_max_events(max_events_to_remove, Pipeline(
    ReadLogFromXes(),
    DfgEntropyIndirectFilter(threshold=0,
                             laplace=False,
                             top_percent=1,
                             max_events_to_remove=max_events_to_remove)
  ))


def test_dfg_entropy_indirect_filter_order_laplace():
  _do_test_direct_filter_order(['B', 'E', 'C', 'A', 'D'], lambda i: Pipeline(
    UseExistingLog(),
    DfgEntropyIndirectFilter(threshold=-1, laplace=True, max_events_to_remove=i, top_percent=1)
  ))


def test_dfg_entropy_indirect_filter_order_non_laplace():
  _do_test_direct_filter_order(['C', 'A', 'D', 'E', 'B'], lambda i: Pipeline(
    UseExistingLog(),
    DfgEntropyIndirectFilter(threshold=-1, laplace=False, max_events_to_remove=i, top_percent=1)
  ))


def test_filter_traces_by_count():
  initial_log = parse_log_from_strings([
    log_creators.insert_separator('a'),
    log_creators.insert_separator('a'),
    log_creators.insert_separator('a'),
    log_creators.insert_separator('a'),
    log_creators.insert_separator('aaaaaaa'),
    log_creators.insert_separator('aaaaaaa'),
  ])

  result = Pipeline(
    UseExistingLog(),
    FilterTracesByCount(min_event_in_trace_count=1),
  )(initial_log)

  assert len(log(result)) == 2


def test_filter_traces_by_variants():
  initial_log = parse_log_from_strings([
    log_creators.insert_separator('a'),
    log_creators.insert_separator('a'),
    log_creators.insert_separator('a'),
    log_creators.insert_separator('a'),
    log_creators.insert_separator('b'),
    log_creators.insert_separator('aaaaaaa'),
    log_creators.insert_separator('aaaaaaa'),
  ])

  result = Pipeline(
    UseExistingLog(),
    FilterTracesByVariants(),
  )(initial_log)

  assert len(log(result)) == 3


def test_filter_traces_by_variants2():
  result = Pipeline(
    ReadLogFromXes(),
    FilterTracesByVariants(),
  )(get_example_log_path('exercise1.xes'))

  assert len(log(result)) == 3
