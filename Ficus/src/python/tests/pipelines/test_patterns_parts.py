from .. import log_creators
from ...ficus.legacy.analysis.patterns.patterns_models import TandemArrayInfo, SubArrayWithTraceIndex, \
  ActivityInTraceInfo, \
  UndefinedActivityHandlingStrategy
from ...ficus.legacy.analysis.patterns.util import default_class_extractor, select_traces_activities_for_activity_level
from ...ficus.legacy.analysis.type_aliases import ClassExtractor
from ...ficus.legacy.log.functions import parse_log_from_string, parse_log_from_strings, serialize_log, serialize_logs
from ...ficus.legacy.pipelines.analysis.high_level import DiscoverActivitiesForSeveralLevels, \
  DiscoverActivitiesFromTandemArrays
from ...ficus.legacy.pipelines.analysis.patterns.models import TandemArrayKind, AdjustingMode
from ...ficus.legacy.pipelines.analysis.patterns.patterns_graph_parts import BuildActivityGraph, BuildEventClassTree, \
  BuildEventClassGraph
from ...ficus.legacy.pipelines.analysis.patterns.patterns_parts import DiscoverTandemArrays, DiscoverRepeatsSets, \
  DiscoverActivities, DiscoverActivitiesInTraces, CreateLogFromActivities, DiscoverRepeats, CreateLogsForActivities, \
  DiscoverActivitiesFromPatterns, ActivitiesDiscoveryStrategy, ClearActivities
from ...ficus.legacy.pipelines.contexts.accessors import patterns, repeat_sets, activities, traces_activities, log, \
  activities_to_logs, graph, event_class_tree
from ...ficus.legacy.pipelines.contexts.keys import patterns_key, repeat_sets_key, activities_key, \
  traces_activities_key, \
  log_key, activities_to_logs_key, graph_key, event_class_tree_key
from ...ficus.legacy.pipelines.contexts.part_results import RepeatActivitiesSource
from ...ficus.legacy.pipelines.pipelines import Pipeline
from ...ficus.legacy.pipelines.start.start_parts import UseExistingLog
from ...ficus.legacy.util import concept_name


def test_discover_primitive_tandem_arrays():
  current_log = parse_log_from_string(log_creators.insert_separator('aaaaazbbc'))
  result = Pipeline(
    UseExistingLog(),
    DiscoverTandemArrays(array_kind=TandemArrayKind.PrimitiveArray)
  )(current_log)

  assert result.has_value(patterns_key)
  assert patterns(result) == [[TandemArrayInfo(first_pos=0, length=2, repeat_count=2)]]


def test_discover_primitive_tandem_arrays2():
  current_log = parse_log_from_string(log_creators.insert_separator('abcabcabc'))
  result = Pipeline(
    UseExistingLog(),
    DiscoverTandemArrays(array_kind=TandemArrayKind.PrimitiveArray)
  )(current_log)

  assert result.has_value(patterns_key)
  assert patterns(result) == [[TandemArrayInfo(first_pos=0, length=3, repeat_count=3),
                               TandemArrayInfo(first_pos=1, length=3, repeat_count=2),
                               TandemArrayInfo(first_pos=2, length=3, repeat_count=2)]]


def test_discover_primitive_tandem_arrays3():
  current_log = parse_log_from_string(log_creators.insert_separator('aabbccddee'))
  result = Pipeline(
    UseExistingLog(),
    DiscoverTandemArrays(array_kind=TandemArrayKind.PrimitiveArray)
  )(current_log)

  assert result.has_value(patterns_key)
  assert patterns(result) == [[]]


def test_discover_primitive_tandem_arrays4():
  current_log = parse_log_from_string(log_creators.insert_separator('abcabcabcabc'))
  result = Pipeline(
    UseExistingLog(),
    DiscoverTandemArrays(array_kind=TandemArrayKind.PrimitiveArray)
  )(current_log)

  assert result.has_value(patterns_key)
  assert patterns(result) == [[TandemArrayInfo(first_pos=0, length=3, repeat_count=4),
                               TandemArrayInfo(first_pos=1, length=3, repeat_count=3),
                               TandemArrayInfo(first_pos=2, length=3, repeat_count=3)]]


def test_discover_maximal_tandem_arrays():
  current_log = parse_log_from_string(log_creators.insert_separator('abcabcabcabc'))
  result = Pipeline(
    UseExistingLog(),
    DiscoverTandemArrays(array_kind=TandemArrayKind.MaximalArray)
  )(current_log)

  assert result.has_value(patterns_key)
  assert patterns(result) == [[TandemArrayInfo(first_pos=0, length=3, repeat_count=4),
                               TandemArrayInfo(first_pos=1, length=3, repeat_count=3),
                               TandemArrayInfo(first_pos=2, length=3, repeat_count=3),
                               TandemArrayInfo(first_pos=0, length=6, repeat_count=2)]]


def test_discover_maximal_tandem_arrays2():
  current_log = parse_log_from_string(log_creators.insert_separator('aaaaaazbbc'))
  result = Pipeline(
    UseExistingLog(),
    DiscoverTandemArrays(array_kind=TandemArrayKind.MaximalArray)
  )(current_log)

  assert result.has_value(patterns_key)
  assert patterns(result) == [[TandemArrayInfo(first_pos=0, length=2, repeat_count=3),
                               TandemArrayInfo(first_pos=0, length=3, repeat_count=2)]]


def test_discover_repeats_sets_from_maximal_tandem_arrays():
  current_log = parse_log_from_string(log_creators.insert_separator('abcabcabcabc'))
  result = Pipeline(
    UseExistingLog(),
    DiscoverTandemArrays(array_kind=TandemArrayKind.MaximalArray),
    DiscoverRepeatsSets(),
  )(current_log)

  assert result.has_value(repeat_sets_key)
  assert repeat_sets(result) == [SubArrayWithTraceIndex(first_pos=0, length=3, trace_index=0)]


def test_discover_repeats_sets_from_primitive_tandem_arrays():
  current_log = parse_log_from_string(log_creators.insert_separator('abcabcabc'))
  result = Pipeline(
    UseExistingLog(),
    DiscoverTandemArrays(array_kind=TandemArrayKind.PrimitiveArray),
    DiscoverRepeatsSets(),
  )(current_log)

  assert result.has_value(repeat_sets_key)
  assert repeat_sets(result) == [SubArrayWithTraceIndex(first_pos=0, length=3, trace_index=0)]


def test_discover_repeats_sets_from_primitive_tandem_arrays2():
  current_log = parse_log_from_string(log_creators.insert_separator('xxabcabcsdsabc'))
  result = Pipeline(
    UseExistingLog(),
    DiscoverTandemArrays(array_kind=TandemArrayKind.PrimitiveArray),
    DiscoverRepeatsSets(),
  )(current_log)

  assert result.has_value(repeat_sets_key)
  assert repeat_sets(result) == [SubArrayWithTraceIndex(first_pos=2, length=3, trace_index=0)]


def test_discover_activities_from_primitive_tandem_arrays():
  current_log = parse_log_from_string(log_creators.insert_separator('xxabcabcsdsabc'))
  result = Pipeline(
    UseExistingLog(),
    DiscoverTandemArrays(array_kind=TandemArrayKind.PrimitiveArray),
    DiscoverRepeatsSets(),
    DiscoverActivities(activity_level=0, class_extractor=default_class_extractor),
  )(current_log)

  assert result.has_value(activities_key)
  assert list(map(lambda x: x.repeat_set, activities(result))) == [
    SubArrayWithTraceIndex(first_pos=2, length=3, trace_index=0)]
  assert list(map(str, activities(result))) == ['[98, 99, 100]']


def test_discover_activities_from_maximal_tandem_arrays():
  current_log = parse_log_from_string(log_creators.insert_separator('abcabcabcabc'))
  result = Pipeline(
    UseExistingLog(),
    DiscoverTandemArrays(array_kind=TandemArrayKind.MaximalArray),
    DiscoverRepeatsSets(),
    DiscoverActivities(activity_level=0, class_extractor=default_class_extractor),
  )(current_log)

  assert result.has_value(activities_key)
  assert list(map(lambda x: x.repeat_set, activities(result))) == [
    SubArrayWithTraceIndex(first_pos=0, length=3, trace_index=0)]
  assert list(map(str, activities(result))) == ['[98, 99, 100]']


def test_discover_activities_in_traces_from_maximal_tandem_arrays():
  current_log = parse_log_from_string(log_creators.insert_separator('abcabcabcabc'))
  result = Pipeline(
    UseExistingLog(),
    DiscoverTandemArrays(array_kind=TandemArrayKind.MaximalArray),
    DiscoverRepeatsSets(),
    DiscoverActivities(activity_level=0, class_extractor=default_class_extractor),
    DiscoverActivitiesInTraces()
  )(current_log)

  assert result.has_value(traces_activities_key)
  assert _serialize_traces_activities(traces_activities(result)) == [['([98, 99, 100], 0, 12)']]


def _serialize_traces_activities(traces_activities: list[list[ActivityInTraceInfo]]):
  test_result = []
  for trace_activities in traces_activities:
    current_result = []
    for trace_activity in trace_activities:
      current_result.append(str(trace_activity))

    test_result.append(current_result)

  return test_result


def test_discover_activities_in_traces_from_primitive_tandem_arrays():
  current_log = parse_log_from_string(log_creators.insert_separator('ababcabcac'))
  result = Pipeline(
    UseExistingLog(),
    DiscoverTandemArrays(array_kind=TandemArrayKind.PrimitiveArray),
    DiscoverRepeatsSets(),
    DiscoverActivities(activity_level=0, class_extractor=default_class_extractor),
    DiscoverActivitiesInTraces()
  )(current_log)

  assert result.has_value(traces_activities_key)
  assert _serialize_traces_activities(traces_activities(result)) == [['([98, 99, 100]([98, 99]), 0, 10)']]


def test_discover_activities_in_traces_from_primitive_tandem_arrays2():
  current_log = parse_log_from_strings([
    log_creators.insert_separator('ababcxabcac'),
    log_creators.insert_separator('abcabcaabc')
  ])

  result = Pipeline(
    UseExistingLog(),
    DiscoverTandemArrays(array_kind=TandemArrayKind.PrimitiveArray),
    DiscoverRepeatsSets(),
    DiscoverActivities(activity_level=0, class_extractor=default_class_extractor),
    DiscoverActivitiesInTraces()
  )(current_log)

  assert result.has_value(traces_activities_key)
  assert _serialize_traces_activities(traces_activities(result)) == [
    ['([98, 99, 100]([98, 99]), 0, 5)', '([98, 99, 100]([98, 99]), 6, 5)'],
    ['([98, 99, 100]([98, 99]), 0, 10)']
  ]


def test_building_log_from_activities():
  current_log = parse_log_from_strings([
    log_creators.insert_separator('ababcxabcac'),
    log_creators.insert_separator('abcabcaabc')
  ])

  result = Pipeline(
    UseExistingLog(),
    DiscoverTandemArrays(array_kind=TandemArrayKind.PrimitiveArray),
    DiscoverRepeatsSets(),
    DiscoverActivities(activity_level=0, class_extractor=default_class_extractor),
    DiscoverActivitiesInTraces(),
    CreateLogFromActivities(use_hashes_as_names=False),
  )(current_log)

  assert result.has_value(log_key)
  assert log(result) != current_log
  assert serialize_log(log(result), traces_separator='||') == 'a::b::c,x,a::b::c||a::b::c'


def test_building_log_from_activities2():
  current_log = parse_log_from_strings([
    log_creators.insert_separator('abcxdfg'),
    log_creators.insert_separator('poiuabchgfabc')
  ])

  result = Pipeline(
    UseExistingLog(),
    DiscoverRepeats(repeat_kind=RepeatActivitiesSource.SuperMaximalRepeats),
    DiscoverRepeatsSets(),
    DiscoverActivities(activity_level=0, class_extractor=default_class_extractor),
    DiscoverActivitiesInTraces(),
    CreateLogFromActivities(use_hashes_as_names=False),
  )(current_log)

  assert result.has_value(log_key)
  assert log(result) != current_log
  assert serialize_log(log(result), traces_separator='||') == 'a::b::c,x,d,f,g||p,o,i,u,a::b::c,h,g,f,a::b::c'


def test_building_log_from_activities3():
  current_log = parse_log_from_strings([
    log_creators.insert_separator('ABABDCcABCFxABCDqABCDeABCF'),
  ])

  result = Pipeline(
    UseExistingLog(),
    DiscoverRepeats(repeat_kind=RepeatActivitiesSource.SuperMaximalRepeats),
    DiscoverRepeatsSets(),
    DiscoverActivities(activity_level=0, class_extractor=default_class_extractor),
    DiscoverActivitiesInTraces(),
    CreateLogFromActivities(use_hashes_as_names=False),
  )(current_log)

  assert result.has_value(log_key)
  assert log(result) != current_log

  assert serialize_log(log(result), traces_separator='||') == \
         'A::B::C::D,c,A::B::C::F,x,A::B::C::D,q,A::B::C::D,e,A::B::C::F'


def test_creating_log_for_activities():
  current_log = parse_log_from_strings([
    log_creators.insert_separator('abcxdfg'),
    log_creators.insert_separator('poiuabchgfabcrtybac')
  ])

  result = Pipeline(
    UseExistingLog(),
    DiscoverRepeats(repeat_kind=RepeatActivitiesSource.SuperMaximalRepeats),
    DiscoverRepeatsSets(),
    DiscoverActivities(activity_level=0, class_extractor=default_class_extractor),
    DiscoverActivitiesInTraces(),
    CreateLogsForActivities(activity_level=0, class_extractor=default_class_extractor),
  )(current_log)

  assert result.has_value(activities_to_logs_key)

  activities_logs = list(activities_to_logs(result).values())
  assert serialize_logs(activities_logs, traces_separator='||') == 'a,b,c||a,b,c||a,b,c||b,a,c'


def test_creating_log_for_activities2():
  current_log = parse_log_from_strings([
    log_creators.insert_separator('abcgxdfg'),
    log_creators.insert_separator('poiuabchgabcrtybac'),
    log_creators.insert_separator('1234xdf09dxf87xdf'),
    log_creators.insert_separator('cba'),
  ])

  result = Pipeline(
    UseExistingLog(),
    DiscoverRepeats(repeat_kind=RepeatActivitiesSource.SuperMaximalRepeats),
    DiscoverRepeatsSets(),
    DiscoverActivities(activity_level=1, class_extractor=default_class_extractor),
    DiscoverActivitiesInTraces(),
    CreateLogsForActivities(activity_level=1, class_extractor=default_class_extractor),
  )(current_log)

  assert result.has_value(activities_to_logs_key)

  activities_logs = list(activities_to_logs(result).values())
  assert serialize_logs(activities_logs, traces_separator='||') == \
         'a,b,c||a,b,c||a,b,c||b,a,c||c,b,a\n\ng||g||g\n\nx,d,f||x,d,f||d,x,f||x,d,f'


def test_creating_log_for_activities3():
  current_log = parse_log_from_strings([
    log_creators.insert_separator('abcasd'),
    log_creators.insert_separator('dfgabca'),
    log_creators.insert_separator('abcalkj'),
  ])

  result = Pipeline(
    UseExistingLog(),
    DiscoverActivitiesFromPatterns(
      DiscoverRepeats(repeat_kind=RepeatActivitiesSource.MaximalRepeats,
                      class_extractor=default_class_extractor),
      class_extractor=default_class_extractor,
      activity_level=0,
      activities_discovery_strategy=ActivitiesDiscoveryStrategy.DiscoverFromSingleMergedTrace,
    ),
    DiscoverActivitiesInTraces(),
    CreateLogsForActivities(activity_level=0, class_extractor=default_class_extractor),
  )(current_log)

  assert result.has_value(activities_to_logs_key)

  activities_logs = list(activities_to_logs(result).values())
  assert serialize_logs(activities_logs, traces_separator='||') == \
         'a,b,c,a||a,b,c,a||a,b,c,a\n\nd||d'


def test_creating_log_for_activities4():
  current_log = parse_log_from_strings([
    log_creators.insert_separator('acbd'),
    log_creators.insert_separator('abcd'),
    log_creators.insert_separator('abcd'),
  ])

  result = Pipeline(
    UseExistingLog(),
    DiscoverActivitiesFromPatterns(
      DiscoverRepeats(repeat_kind=RepeatActivitiesSource.NearSuperMaximalRepeats,
                      class_extractor=default_class_extractor),
      class_extractor=default_class_extractor,
      activity_level=1,
      activities_discovery_strategy=ActivitiesDiscoveryStrategy.DiscoverFromSingleMergedTrace,
    ),
    DiscoverActivitiesInTraces(),
    CreateLogsForActivities(activity_level=1, class_extractor=default_class_extractor),
  )(current_log)

  assert result.has_value(activities_to_logs_key)

  activities_logs = list(activities_to_logs(result).values())
  assert serialize_logs(activities_logs, traces_separator='||') == \
         'a,c,b,d||a,b,c,d||a,b,c,d'


def test_creating_log_for_activities5():
  current_log = parse_log_from_strings([
    log_creators.insert_separator('acbd'),
  ])

  result = Pipeline(
    UseExistingLog(),
    DiscoverActivitiesFromPatterns(
      DiscoverRepeats(repeat_kind=RepeatActivitiesSource.NearSuperMaximalRepeats,
                      class_extractor=default_class_extractor),
      class_extractor=default_class_extractor,
      activity_level=123,
      activities_discovery_strategy=ActivitiesDiscoveryStrategy.DiscoverFromSingleMergedTrace,
    ),
    DiscoverActivitiesInTraces(),
    CreateLogsForActivities(activity_level=123, class_extractor=default_class_extractor),
  )(current_log)

  assert result.has_value(activities_to_logs_key)

  activities_logs = list(activities_to_logs(result).values())
  assert serialize_logs(activities_logs, traces_separator='||') == ''


def test_discovering_activities_in_unattached_traces():
  current_log = _create_log_with_two_activities_level()

  def second_class_extractor(event):
    if '.' in event[concept_name]:
      return event[concept_name][:event[concept_name].index('.')]

    return event[concept_name]

  result = Pipeline(
    UseExistingLog(),
    DiscoverActivitiesForSeveralLevels([default_class_extractor, second_class_extractor],
                                       discovering_strategy=ActivitiesDiscoveryStrategy.DiscoverFromAllTraces,
                                       adjusting_mode=AdjustingMode.FromUnattachedSubTraces),
  )(current_log)

  assert result.has_value(traces_activities_key)

  first_level_activities = select_traces_activities_for_activity_level(traces_activities(result), 0)
  assert _serialize_traces_activities(first_level_activities) == \
         [['([66, 67, 68], 0, 3)', '([66, 67, 68], 6, 3)']]

  second_level_activities = select_traces_activities_for_activity_level(traces_activities(result), 1)
  assert _serialize_traces_activities(second_level_activities) == \
         [['([69, 70, 71], 3, 3)', '([69, 70, 71], 9, 3)']]


def _create_log_with_two_activities_level():
  return parse_log_from_strings([
    ','.join(['A', 'B', 'C', 'D.A', 'E.B', 'F.C', 'A', 'B', 'C', 'D.C', 'E.A', 'F.A']),
  ])


def _first_class_extractor(event):
  if '.' in event[concept_name]:
    return event[concept_name][:event[concept_name].index('.')]

  return event[concept_name]


def test_discovering_activities_with_first_tandem_array():
  current_log = parse_log_from_strings([
    log_creators.insert_separator('abcabcabcxyzabcabckjlxyzpo')
  ])

  strategy = ActivitiesDiscoveryStrategy.DiscoverFromSingleMergedTrace
  result = _get_pipeline_for_discovering_activities_with_first_tandem_array(strategy)(current_log)

  assert result.has_value(patterns_key)
  assert result.has_value(activities_key)
  assert result.has_value(traces_activities_key)
  assert result.has_value(log_key)

  assert serialize_log(log(result)) == 'a::b::c,x::y::z,a::b::c,k,j,l,x::y::z,p,o'


def _get_pipeline_for_discovering_activities_with_first_tandem_array(strategy) -> Pipeline:
  return Pipeline(
    UseExistingLog(),
    DiscoverActivitiesFromTandemArrays(array_kind=TandemArrayKind.PrimitiveArray,
                                       activity_level=0),
    ClearActivities(),
    DiscoverActivitiesForSeveralLevels([default_class_extractor],
                                       discovering_strategy=strategy),
    CreateLogFromActivities(strategy=UndefinedActivityHandlingStrategy.InsertAllEvents,
                            use_hashes_as_names=False),
  )


def test_discovering_activities_with_first_tandem_array2():
  current_log = parse_log_from_strings([
    log_creators.insert_separator('abcabcabcxyzabcabckjlxyzpo'),
    log_creators.insert_separator('kjlmnbnmbnmbmnbmnbnmb'),
  ])

  strategy = ActivitiesDiscoveryStrategy.DiscoverFromSingleMergedTrace
  result = _get_pipeline_for_discovering_activities_with_first_tandem_array(strategy)(current_log)

  assert result.has_value(patterns_key)
  assert result.has_value(activities_key)
  assert result.has_value(traces_activities_key)
  assert result.has_value(log_key)

  assert serialize_log(log(result)) == 'a::b::c,x::y::z,a::b::c,j::k::l,x::y::z,p,o\nj::k::l,b::m::n'


def test_building_graph_for_activities():
  current_log = parse_log_from_strings([
    log_creators.insert_separator('abcabcabcxyzabcabckjlxyzpo'),
    log_creators.insert_separator('kjlmnbnmbnmbmnbmnbnmb'),
  ])

  strategy = ActivitiesDiscoveryStrategy.DiscoverFromSingleMergedTrace
  graph_name = 'Graph name'

  result = Pipeline(
    UseExistingLog(),
    DiscoverActivitiesFromTandemArrays(array_kind=TandemArrayKind.PrimitiveArray,
                                       activity_level=0),
    ClearActivities(),
    DiscoverActivitiesForSeveralLevels([default_class_extractor], discovering_strategy=strategy),
    BuildActivityGraph(activity_level=0, graph_name=graph_name)
  )(current_log)

  assert result.has_value(graph_key)
  assert graph(result).name == graph_name


def test_building_event_class_tree():
  current_log = parse_log_from_strings([
    log_creators.insert_separator('abcabcabcxyzabcabckjlxyzpo'),
    log_creators.insert_separator('kjlmnbnmbnmbmnbmnbnmboiab'),
  ])

  strategy = ActivitiesDiscoveryStrategy.DiscoverFromSingleMergedTrace

  result = Pipeline(
    UseExistingLog(),
    DiscoverActivitiesFromTandemArrays(array_kind=TandemArrayKind.PrimitiveArray,
                                       activity_level=0),
    CreateLogFromActivities(use_hashes_as_names=False),
    ClearActivities(),
    DiscoverActivitiesForSeveralLevels([default_class_extractor], discovering_strategy=strategy),
    BuildEventClassTree(class_extractors=[lambda x: x])
  )(current_log)

  assert result.has_value(event_class_tree_key)
  assert list(sorted(map(str, event_class_tree(result)))) == \
         ['a::b::c(a::b::c)', 'b::m::n(b::m::n)', 'i(i)', 'j(j)', 'k(k)', 'l(l)',
          'o(o)', 'p(p)', 'x(x)', 'y(y)', 'z(z)']


def test_create_logs_for_activities_of_different():
  current_log = _create_log_with_two_activities_level()

  def create_logs_for_activities(activity_level: int, class_extractor: ClassExtractor):
    result = Pipeline(
      UseExistingLog(),
      DiscoverActivitiesForSeveralLevels([default_class_extractor, _first_class_extractor],
                                         discovering_strategy=ActivitiesDiscoveryStrategy.DiscoverFromAllTraces,
                                         adjusting_mode=AdjustingMode.FromUnattachedSubTraces),
      CreateLogsForActivities(activity_level=activity_level, class_extractor=class_extractor)
    )(current_log)

    return activities_to_logs(result)

  zero_level_activities = create_logs_for_activities(0, default_class_extractor)
  assert serialize_logs(list(zero_level_activities.values())) == 'A,B,C\nA,B,C'

  zero_level_activities = create_logs_for_activities(0, _first_class_extractor)
  assert serialize_logs(list(zero_level_activities.values())) == 'A,B,C\nA,B,C'

  first_level_activities = create_logs_for_activities(1, _first_class_extractor)
  assert serialize_logs(list(first_level_activities.values())) == 'D,E,F\nD,E,F'

  first_level_activities = create_logs_for_activities(1, default_class_extractor)
  assert serialize_logs(list(first_level_activities.values())) == 'D.A,E.B,F.C\nD.C,E.A,F.A'


def test_build_event_class_graph():
  current_log = _create_log_with_two_activities_level()

  result = Pipeline(
    UseExistingLog(),
    BuildEventClassTree([lambda x: x]),
    BuildEventClassGraph(graph_name='Event class graph'),
  )(current_log)

  assert result.has_value(graph_key)
  assert graph(result).name == 'Event class graph'
