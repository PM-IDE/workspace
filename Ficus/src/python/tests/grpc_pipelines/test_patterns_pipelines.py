from ...ficus.grpc_pipelines.context_values import BytesContextValue, read_file_bytes
from ...ficus.grpc_pipelines.filtering_parts import FilterEventsByRegex, FilterLogByVariants

from ...ficus.grpc_pipelines.xes_parts import ReadLogFromXes, ReadLogFromBxes

from ...ficus.legacy.analysis.patterns.patterns_models import UndefinedActivityHandlingStrategy
from ...ficus.grpc_pipelines.data_models import PatternsKind, PatternsDiscoveryStrategy, NarrowActivityKind, \
    ActivityFilterKind, ActivitiesLogsSource

from ...ficus.grpc_pipelines.drawing_parts import TracesDiversityDiagram

from ...ficus.grpc_pipelines.activities_parts import DiscoverActivities, DiscoverActivitiesInstances, \
    CreateLogFromActivitiesInstances, DiscoverActivitiesForSeveralLevels, DiscoverActivitiesFromPatterns, \
    DiscoverActivitiesUntilNoMore, ExecuteWithEachActivityLog, ClearActivitiesRelatedStuff, \
    PrintNumberOfUnderlyingEvents, SubstituteUnderlyingEvents

from ...ficus.grpc_pipelines.patterns_parts import FindSuperMaximalRepeats

from ...ficus.grpc_pipelines.util_parts import UseNamesEventLog

from ...ficus.grpc_pipelines.entry_points.default_pipeline import Pipeline
from .pipeline_parts_for_tests import AssertNamesLogTestPart
from .test_grpc_pipelines import _execute_test_with_names_log, _execute_test_with_exercise_log, \
    _execute_test_with_context
from ..test_data_provider import console_app_method2_bxes_log_path


def test_class_extractors():
    _execute_test_with_names_log(
        [
            ['A.A', 'B.B', 'C', 'A.C', 'B.D'],
            ['A.D', 'B.C', 'C', 'A.A', 'B.B'],
        ],
        Pipeline(
            UseNamesEventLog(),
            FindSuperMaximalRepeats(strategy=PatternsDiscoveryStrategy.FromAllTraces, class_extractor='^(.*?)\\.'),
            DiscoverActivities(activity_level=0),
            DiscoverActivitiesInstances(narrow_activities=NarrowActivityKind.NarrowDown),
            CreateLogFromActivitiesInstances(strategy=UndefinedActivityHandlingStrategy.InsertAllEvents),
            AssertNamesLogTestPart([
                ['(A.A)::(B.B)', 'C', '(A.A)::(B.B)'],
                ['(A.A)::(B.B)', 'C', '(A.A)::(B.B)']
            ])
        ))


def test_several_levels():
    _execute_test_with_names_log(
        [
            ['A.A', 'B.B', 'C', 'D', 'A.C', 'B.D', 'C', 'D'],
            ['A.D', 'B.C', 'C', 'D', 'A.A', 'B.B'],
        ],
        Pipeline(
            UseNamesEventLog(),
            TracesDiversityDiagram(plot_legend=True, title='InitialLog'),
            DiscoverActivitiesForSeveralLevels(event_classes=[r'^(.*?)(?=\.)', '.*'],
                                               patterns_kind=PatternsKind.MaximalRepeats),
            CreateLogFromActivitiesInstances(),
            AssertNamesLogTestPart([
                ['(A)::(B)', '(C)::(D)', '(A)::(B)', '(C)::(D)'],
                ['(A)::(B)', '(C)::(D)', '(A)::(B)']
            ])
        )
    )


def test_discover_activities_from_patterns():
    _execute_test_with_names_log(
        [
            ['A', 'B', 'C', 'A', 'B'],
            ['A', 'B', 'C', 'A', 'B'],
        ],
        Pipeline(
            UseNamesEventLog(),
            DiscoverActivitiesFromPatterns(patterns_kind=PatternsKind.MaximalRepeats),
            DiscoverActivitiesInstances(narrow_activities=NarrowActivityKind.NarrowDown),
            CreateLogFromActivitiesInstances(strategy=UndefinedActivityHandlingStrategy.InsertAllEvents),
            AssertNamesLogTestPart([['(A)::(B)', 'C', '(A)::(B)'], ['(A)::(B)', 'C', '(A)::(B)']])
        )
    )


def test_discover_activities_until_no_more():
    _execute_test_with_names_log(
        [
            ['A.A', 'B.B', 'C', 'D', 'A.C', 'B.D', 'C', 'D'],
            ['A.D', 'B.C', 'C', 'D', 'A.A', 'B.B'],
        ],
        Pipeline(
            UseNamesEventLog(),
            DiscoverActivitiesUntilNoMore(event_class=r'^(.*?)(?=\.)'),
            AssertNamesLogTestPart([['(A)::(B)::(C)::(D)'], ['(A)::(B)::(C)::(D)']])
        )
    )


def test_execute_with_each_activity_log():
    _execute_test_with_exercise_log('exercise4', Pipeline(
        ReadLogFromXes(use_bytes=True),
        DiscoverActivitiesFromPatterns(patterns_kind=PatternsKind.MaximalRepeats,
                                       strategy=PatternsDiscoveryStrategy.FromSingleMergedTrace),
        DiscoverActivitiesInstances(narrow_activities=NarrowActivityKind.NarrowDown),
        ExecuteWithEachActivityLog(ActivitiesLogsSource.TracesActivities, 0, Pipeline(
            TracesDiversityDiagram(plot_legend=True)
        ))
    ))


def test_console_app1_log():
    _execute_test_with_context(Pipeline(
        ReadLogFromBxes(use_bytes=True),
        FilterEventsByRegex('Procfiler.*'),
        FilterEventsByRegex(r'GC/SampledObjectAllocation_\{System\.Int32\[\]\}'),
        FilterEventsByRegex(r'.*SuspendEE.*'),
        FilterEventsByRegex(r'.*RestartEE.*'),
        FilterLogByVariants(),
        DiscoverActivitiesFromPatterns(PatternsKind.PrimitiveTandemArrays,
                                       activity_level=0),
        DiscoverActivitiesInstances(narrow_activities=NarrowActivityKind.NarrowDown, min_events_in_activity=2),
        CreateLogFromActivitiesInstances(strategy=UndefinedActivityHandlingStrategy.InsertAllEvents),
        ClearActivitiesRelatedStuff(),
        DiscoverActivitiesForSeveralLevels(['.*'],
                                           PatternsKind.MaximalRepeats,
                                           min_events_in_activity_count=2,
                                           strategy=PatternsDiscoveryStrategy.FromAllTraces),
        CreateLogFromActivitiesInstances(strategy=UndefinedActivityHandlingStrategy.DontInsert),
        ClearActivitiesRelatedStuff(),
        DiscoverActivitiesUntilNoMore(strategy=PatternsDiscoveryStrategy.FromSingleMergedTrace,
                                      undef_strategy=UndefinedActivityHandlingStrategy.InsertAllEvents),
        PrintNumberOfUnderlyingEvents()
    ), {
        'bytes': BytesContextValue(read_file_bytes(console_app_method2_bxes_log_path()))
    })


def test_console_app1_two_levels_of_abstraction():
    _execute_test_with_context(Pipeline(
        ReadLogFromBxes(use_bytes=True),
        FilterEventsByRegex('Procfiler.*'),
        FilterEventsByRegex(r'GC/SampledObjectAllocation_\{System\.Int32\[\]\}'),
        FilterEventsByRegex(r'.*SuspendEE.*'),
        FilterEventsByRegex(r'.*RestartEE.*'),
        FilterLogByVariants(),
        DiscoverActivitiesFromPatterns(PatternsKind.PrimitiveTandemArrays,
                                       activity_level=0),
        DiscoverActivitiesInstances(narrow_activities=NarrowActivityKind.NarrowDown, min_events_in_activity=2),
        CreateLogFromActivitiesInstances(strategy=UndefinedActivityHandlingStrategy.InsertAllEvents),
        ClearActivitiesRelatedStuff(),
        DiscoverActivitiesForSeveralLevels([r'^(.*?)_\{', '.*'],
                                           PatternsKind.MaximalRepeats,
                                           activity_filter_kind=ActivityFilterKind.NoFilter),
        CreateLogFromActivitiesInstances(strategy=UndefinedActivityHandlingStrategy.DontInsert),
        ClearActivitiesRelatedStuff(),
        DiscoverActivitiesUntilNoMore(strategy=PatternsDiscoveryStrategy.FromSingleMergedTrace,
                                      activity_filter_kind=ActivityFilterKind.NoFilter,
                                      undef_strategy=UndefinedActivityHandlingStrategy.InsertAllEvents),
        ExecuteWithEachActivityLog(ActivitiesLogsSource.Log, activity_level=2, activity_log_pipeline=Pipeline(
            SubstituteUnderlyingEvents(),
        ))
    ), {
        'bytes': BytesContextValue(read_file_bytes(console_app_method2_bxes_log_path()))
    })
