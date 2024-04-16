from ...ficus.grpc_pipelines.context_values import StringContextValue
from ...ficus.grpc_pipelines.filtering_parts import FilterEventsByRegex2, FilterLogByVariants2

from ...ficus.grpc_pipelines.xes_parts import ReadLogFromXes2

from ...ficus.legacy.analysis.patterns.patterns_models import UndefinedActivityHandlingStrategy
from ...ficus.grpc_pipelines.data_models import PatternsKind, PatternsDiscoveryStrategy, NarrowActivityKind, \
    ActivityFilterKind, ActivitiesLogsSource

from ...ficus.grpc_pipelines.drawing_parts import TracesDiversityDiagram2

from ...ficus.grpc_pipelines.activities_parts import DiscoverActivities2, DiscoverActivitiesInstances2, \
    CreateLogFromActivitiesInstances2, DiscoverActivitiesForSeveralLevels2, DiscoverActivitiesFromPatterns2, \
    DiscoverActivitiesUntilNoMore2, ExecuteWithEachActivityLog2, ClearActivitiesRelatedStuff2, \
    PrintNumberOfUnderlyingEvents2, SubstituteUnderlyingEvents2

from ...ficus.grpc_pipelines.patterns_parts import FindSuperMaximalRepeats2

from ...ficus.grpc_pipelines.util_parts import UseNamesEventLog2

from ...ficus.grpc_pipelines.grpc_pipelines import Pipeline2
from .pipeline_parts_for_tests import AssertNamesLogTestPart
from .test_grpc_pipelines import _execute_test_with_names_log, _execute_test_with_exercise_log, \
    _execute_test_with_context
from ..test_data_provider import console_app_method2_log_path


def test_class_extractors():
    _execute_test_with_names_log(
        [
            ['A.A', 'B.B', 'C', 'A.C', 'B.D'],
            ['A.D', 'B.C', 'C', 'A.A', 'B.B'],
        ],
        Pipeline2(
            UseNamesEventLog2(),
            FindSuperMaximalRepeats2(strategy=PatternsDiscoveryStrategy.FromAllTraces, class_extractor='^(.*?)\\.'),
            DiscoverActivities2(activity_level=0),
            DiscoverActivitiesInstances2(narrow_activities=NarrowActivityKind.NarrowDown),
            CreateLogFromActivitiesInstances2(strategy=UndefinedActivityHandlingStrategy.InsertAllEvents),
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
        Pipeline2(
            UseNamesEventLog2(),
            TracesDiversityDiagram2(plot_legend=True, title='InitialLog'),
            DiscoverActivitiesForSeveralLevels2(event_classes=[r'^(.*?)(?=\.)', '.*'],
                                                patterns_kind=PatternsKind.MaximalRepeats),
            CreateLogFromActivitiesInstances2(),
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
        Pipeline2(
            UseNamesEventLog2(),
            DiscoverActivitiesFromPatterns2(patterns_kind=PatternsKind.MaximalRepeats),
            DiscoverActivitiesInstances2(narrow_activities=NarrowActivityKind.NarrowDown),
            CreateLogFromActivitiesInstances2(strategy=UndefinedActivityHandlingStrategy.InsertAllEvents),
            AssertNamesLogTestPart([['(A)::(B)', 'C', '(A)::(B)'], ['(A)::(B)', 'C', '(A)::(B)']])
        )
    )


def test_discover_activities_until_no_more():
    _execute_test_with_names_log(
        [
            ['A.A', 'B.B', 'C', 'D', 'A.C', 'B.D', 'C', 'D'],
            ['A.D', 'B.C', 'C', 'D', 'A.A', 'B.B'],
        ],
        Pipeline2(
            UseNamesEventLog2(),
            DiscoverActivitiesUntilNoMore2(event_class=r'^(.*?)(?=\.)'),
            AssertNamesLogTestPart([['(A)::(B)::(C)::(D)'], ['(A)::(B)::(C)::(D)']])
        )
    )


def test_execute_with_each_activity_log():
    _execute_test_with_exercise_log('exercise4', Pipeline2(
        ReadLogFromXes2(),
        DiscoverActivitiesFromPatterns2(patterns_kind=PatternsKind.MaximalRepeats,
                                        strategy=PatternsDiscoveryStrategy.FromSingleMergedTrace),
        DiscoverActivitiesInstances2(narrow_activities=NarrowActivityKind.NarrowDown),
        ExecuteWithEachActivityLog2(ActivitiesLogsSource.TracesActivities, 0, Pipeline2(
            TracesDiversityDiagram2(plot_legend=True)
        ))
    ))


def test_console_app1_log():
    _execute_test_with_context(Pipeline2(
        ReadLogFromXes2(),
        FilterEventsByRegex2('Procfiler.*'),
        FilterEventsByRegex2(r'GC/SampledObjectAllocation_\{System\.Int32\[\]\}'),
        FilterEventsByRegex2(r'.*SuspendEE.*'),
        FilterEventsByRegex2(r'.*RestartEE.*'),
        FilterLogByVariants2(),
        DiscoverActivitiesFromPatterns2(PatternsKind.PrimitiveTandemArrays,
                                        activity_level=0),
        DiscoverActivitiesInstances2(narrow_activities=NarrowActivityKind.NarrowDown, min_events_in_activity=2),
        CreateLogFromActivitiesInstances2(strategy=UndefinedActivityHandlingStrategy.InsertAllEvents),
        ClearActivitiesRelatedStuff2(),
        DiscoverActivitiesForSeveralLevels2(['.*'],
                                            PatternsKind.MaximalRepeats,
                                            min_events_in_activity_count=2,
                                            strategy=PatternsDiscoveryStrategy.FromAllTraces),
        CreateLogFromActivitiesInstances2(strategy=UndefinedActivityHandlingStrategy.DontInsert),
        ClearActivitiesRelatedStuff2(),
        DiscoverActivitiesUntilNoMore2(strategy=PatternsDiscoveryStrategy.FromSingleMergedTrace,
                                       undef_strategy=UndefinedActivityHandlingStrategy.InsertAllEvents),
        PrintNumberOfUnderlyingEvents2()
    ), {
        'path': StringContextValue(console_app_method2_log_path())
    })


def test_console_app1_two_levels_of_abstraction():
    _execute_test_with_context(Pipeline2(
        ReadLogFromXes2(),
        FilterEventsByRegex2('Procfiler.*'),
        FilterEventsByRegex2(r'GC/SampledObjectAllocation_\{System\.Int32\[\]\}'),
        FilterEventsByRegex2(r'.*SuspendEE.*'),
        FilterEventsByRegex2(r'.*RestartEE.*'),
        FilterLogByVariants2(),
        DiscoverActivitiesFromPatterns2(PatternsKind.PrimitiveTandemArrays,
                                        activity_level=0),
        DiscoverActivitiesInstances2(narrow_activities=NarrowActivityKind.NarrowDown, min_events_in_activity=2),
        CreateLogFromActivitiesInstances2(strategy=UndefinedActivityHandlingStrategy.InsertAllEvents),
        ClearActivitiesRelatedStuff2(),
        DiscoverActivitiesForSeveralLevels2([r'^(.*?)_\{', '.*'],
                                            PatternsKind.MaximalRepeats,
                                            activity_filter_kind=ActivityFilterKind.NoFilter),
        CreateLogFromActivitiesInstances2(strategy=UndefinedActivityHandlingStrategy.DontInsert),
        ClearActivitiesRelatedStuff2(),
        DiscoverActivitiesUntilNoMore2(strategy=PatternsDiscoveryStrategy.FromSingleMergedTrace,
                                       activity_filter_kind=ActivityFilterKind.NoFilter,
                                       undef_strategy=UndefinedActivityHandlingStrategy.InsertAllEvents),
        ExecuteWithEachActivityLog2(ActivitiesLogsSource.Log, activity_level=2, activity_log_pipeline=Pipeline2(
            SubstituteUnderlyingEvents2(),
        ))
    ), {
        'path': StringContextValue(console_app_method2_log_path())
    })
