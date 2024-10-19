import os.path
from enum import Enum

from ...ficus.legacy.analysis.patterns.patterns_models import UndefinedActivityHandlingStrategy
from ...ficus.grpc_pipelines.activities_parts import DiscoverActivities, DiscoverActivitiesInstances, \
    CreateLogFromActivitiesInstances, ApplyClassExtractor
from ...ficus.grpc_pipelines.constants import const_names_event_log
from ...ficus.grpc_pipelines.context_values import StringContextValue, NamesLogContextValue, ContextValue, \
    BytesContextValue, read_file_bytes
from ...ficus.grpc_pipelines.data_models import NarrowActivityKind
from ...ficus.grpc_pipelines.drawing_parts import TracesDiversityDiagram, DrawPlacementsOfEventByName, \
    DrawPlacementOfEventsByRegex
from ...ficus.grpc_pipelines.filtering_parts import FilterTracesByEventsCount
from ...ficus.grpc_pipelines.entry_points.default_pipeline import Pipeline, PrintEventLogInfo, ficus_backend_addr_key
from ...ficus.grpc_pipelines.patterns_parts import FindSuperMaximalRepeats, PatternsDiscoveryStrategy
from ...ficus.grpc_pipelines.util_parts import UseNamesEventLog
from ...ficus.grpc_pipelines.xes_parts import ReadLogFromXes
from .pipeline_parts_for_tests import AssertNamesLogTestPart
from ..test_data_provider import get_example_log_path


def test_simple_pipeline():
    pipeline = Pipeline(
        ReadLogFromXes()
    )

    result = _execute_pipeline(pipeline, {
        'path': StringContextValue('asdasdasdasdas')
    })

    assert not result.finalResult.HasField('success')
    assert result.finalResult.error is not None
    assert result.finalResult.error == 'Failed to read event log from asdasdasdasdas'


def _execute_pipeline(pipeline, config):
    backend_addr = os.getenv('FICUS_BACKEND_ADDR')
    if backend_addr is None:
        backend_addr = 'localhost:8080'

    config[ficus_backend_addr_key] = backend_addr
    return pipeline.execute(config)


def test_pipeline_with_getting_context_value():
    _execute_test_with_exercise_log('exercise1', Pipeline(
        ReadLogFromXes(use_bytes=True),
        TracesDiversityDiagram(),
    ))


def _execute_test_with_exercise_log(log_name: str, pipeline: Pipeline):
    result = _execute_pipeline(pipeline, {
        'bytes': BytesContextValue(read_file_bytes(get_example_log_path(f'{log_name}.xes')))
    })

    assert result.finalResult.HasField('success')
    assert not result.finalResult.HasField('error')


def _execute_test_with_context(pipeline: Pipeline, context: dict[str, ContextValue]):
    result = _execute_pipeline(pipeline, context)

    assert result.finalResult.HasField('success')
    assert not result.finalResult.HasField('error')


def test_pipeline_with_getting_context_value2():
    _execute_test_with_exercise_log('exercise1', Pipeline(
        ReadLogFromXes(use_bytes=True),
        DrawPlacementsOfEventByName('A'),
    ))


def test_pipeline_with_getting_context_value3():
    _do_simple_test_with_regex('A|B')


def _do_simple_test_with_regex(regex: str):
    _execute_test_with_exercise_log('exercise1', Pipeline(
        ReadLogFromXes(use_bytes=True),
        DrawPlacementOfEventsByRegex(regex)
    ))


def test_pipeline_with_getting_context_value4():
    _do_simple_test_with_regex('A|B|C')


def test_pipeline_with_getting_context_value5():
    _do_simple_test_with_regex('A|B|C|D')


def test_draw_short_activities_diagram():
    _execute_test_with_exercise_log('exercise4', Pipeline(
        ReadLogFromXes(use_bytes=True),
        FindSuperMaximalRepeats(strategy=PatternsDiscoveryStrategy.FromSingleMergedTrace),
        DiscoverActivities(activity_level=0),
        DiscoverActivitiesInstances(narrow_activities=NarrowActivityKind.NarrowDown),
        CreateLogFromActivitiesInstances(strategy=UndefinedActivityHandlingStrategy.InsertAllEvents),
        AssertNamesLogTestPart([
            ['(a)::(b)', '(c)::(d)', 'f'],
            ['(a)::(c)', '(b)::(d)', 'f'],
            ['(a)::(c)', '(b)::(d)', 'f'],
            ['a', 'd', '(e)', 'f'],
            ['(a)::(b)', '(c)::(d)', 'f'],
            ['a', '(e)', 'd', 'f']
        ])
    ))


def test_draw_full_activities_diagram_2():
    _execute_test_with_exercise_log('exercise4', Pipeline(
        ReadLogFromXes(use_bytes=True),
        FindSuperMaximalRepeats(strategy=PatternsDiscoveryStrategy.FromAllTraces),
        DiscoverActivities(activity_level=0),
        DiscoverActivitiesInstances(narrow_activities=NarrowActivityKind.NarrowDown),
        CreateLogFromActivitiesInstances(),
        AssertNamesLogTestPart([[], [], [], [], [], []])
    ))


def test_get_event_log_info():
    _execute_test_with_exercise_log('exercise4', Pipeline(
        ReadLogFromXes(use_bytes=True),
        PrintEventLogInfo(),
    ))


def test_filter_traces_by_events_count():
    _execute_test_with_exercise_log('exercise4', Pipeline(
        ReadLogFromXes(use_bytes=True),
        FilterTracesByEventsCount(min_events_in_trace=5),
        AssertNamesLogTestPart([
            ['a', 'b', 'd', 'c', 'f'],
            ['a', 'c', 'b', 'd', 'f'],
            ['a', 'c', 'd', 'b', 'f'],
            ['a', 'b', 'c', 'd', 'f']
        ])
    ))


class ResultAssertanceKind(Enum):
    Success = 0
    Error = 1


def _execute_test_with_names_log(names_log: list[list[str]],
                                 pipeline: Pipeline,
                                 assertance_kind: ResultAssertanceKind = ResultAssertanceKind.Success):
    result = _execute_pipeline(pipeline, {
        const_names_event_log: NamesLogContextValue(names_log)
    })

    success_field = 'success'
    error_field = 'error'

    (present_field, not_present_error_field) = (success_field, error_field) if assertance_kind == ResultAssertanceKind.Success else (error_field, success_field)

    assert result.finalResult.HasField(present_field)
    assert not result.finalResult.HasField(not_present_error_field)


def test_apply_class_extractor():
    _execute_test_with_names_log(
        [
            ['A.A', 'B.B', 'C', 'D', 'A.C', 'B.D', 'C', 'D'],
            ['A.D', 'B.C', 'C', 'D', 'A.A', 'B.B'],
        ],
        Pipeline(
            UseNamesEventLog(),
            ApplyClassExtractor(class_extractor_regex=r'^(.*?)(?=\.)', filter_regex=r'A\..*'),
            AssertNamesLogTestPart(
                [
                    ['A', 'B.B', 'C', 'D', 'A', 'B.D', 'C', 'D'],
                    ['A', 'B.C', 'C', 'D', 'A', 'B.B'],
                ]
            )
        )
    )
