from ...ficus.grpc_pipelines.filtering_parts import FilterEventsByName, FilterEventsByRegex, FilterLogByVariants

from ...ficus.grpc_pipelines.xes_parts import ReadLogFromXes

from ...ficus.grpc_pipelines.grpc_pipelines import Pipeline
from .pipeline_parts_for_tests import AssertNamesLogTestPart
from .test_grpc_pipelines import _execute_test_with_exercise_log


def test_filter_events_by_name():
    _execute_test_with_exercise_log('exercise4', Pipeline(
        ReadLogFromXes(),
        FilterEventsByName(event_name='a'),
        AssertNamesLogTestPart([
            ['b', 'd', 'c', 'f'],
            ['c', 'b', 'd', 'f'],
            ['c', 'd', 'b', 'f'],
            ['d', 'e', 'f'],
            ['b', 'c', 'd', 'f'],
            ['e', 'd', 'f']
        ])
    ))


def test_filter_events_by_regex():
    _execute_test_with_exercise_log('exercise4', Pipeline(
        ReadLogFromXes(),
        FilterEventsByRegex(regex='a|b'),
        AssertNamesLogTestPart([
            ['d', 'c', 'f'],
            ['c', 'd', 'f'],
            ['c', 'd', 'f'],
            ['d', 'e', 'f'],
            ['c', 'd', 'f'],
            ['e', 'd', 'f']
        ])
    ))


def test_filter_log_by_variants():
    _execute_test_with_exercise_log('exercise4', Pipeline(
        ReadLogFromXes(),
        FilterLogByVariants(),
        AssertNamesLogTestPart([
            ['a', 'b', 'd', 'c', 'f'],
            ['a', 'c', 'b', 'd', 'f'],
            ['a', 'c', 'd', 'b', 'f'],
            ['a', 'd', 'e', 'f'],
            ['a', 'b', 'c', 'd', 'f'],
            ['a', 'e', 'd', 'f']
        ])
    ))
