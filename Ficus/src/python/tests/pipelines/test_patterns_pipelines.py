import os

import pytest

from ..test_data_provider import console_app_method2_bxes_log_path, data_dir, gold_dir
from ...ficus.legacy.analysis.patterns.event_log_patterns import *
from ...ficus.legacy.pipelines.analysis.high_level import DiscoverActivitiesFromTandemArrays, \
    DiscoverActivitiesForSeveralLevels
from ...ficus.legacy.pipelines.analysis.patterns.patterns_graph_parts import SerializeGraph, BuildActivityGraph, \
    BuildEventClassGraph, BuildEventClassTree
from ...ficus.legacy.pipelines.analysis.patterns.patterns_parts import TandemArrayKind, ClearActivities, \
    ActivitiesDiscoveryStrategy
from ...ficus.legacy.pipelines.contexts.accessors import serialized_graph
from ...ficus.legacy.pipelines.filtering.filter_parts import PosEntropyDirectFilter
from ...ficus.legacy.pipelines.mutations.mutations_parts import RemoveEventsFromLogPredicate
from ...ficus.legacy.pipelines.pipelines import Pipeline
from ...ficus.legacy.pipelines.start.start_parts import ReadLogFromXes
from ...tests.core.gold_based_test import execute_test_with_gold


def filter_predicate(event: MyEvent):
    return event[concept_name].startswith('Procfiler/')


def _do_test_with_activities_graph(gold_path: str, path_to_xes: str):
    result = Pipeline(
        ReadLogFromXes(),
        RemoveEventsFromLogPredicate(filter_predicate),
        PosEntropyDirectFilter(0),
        DiscoverActivitiesFromTandemArrays(array_kind=TandemArrayKind.PrimitiveArray,
                                           activity_level=0),
        ClearActivities(),
        DiscoverActivitiesForSeveralLevels([default_class_extractor],
                                           discovering_strategy=ActivitiesDiscoveryStrategy.DiscoverFromAllTraces),
        BuildActivityGraph(activity_level=0, graph_name='Activities Graph'),
        SerializeGraph()
    )(path_to_xes)

    execute_test_with_gold(gold_path, serialized_graph(result))


def _do_test_with_event_graph(gold_path: str, path_to_xes: str):
    def first_class_extractor(name: str) -> str:
        return name[:name.index('_')] if '_' in name else name

    def second_class_extractor(name: str) -> str:
        return name[:name.index('/')] if '/' in name else name

    result = Pipeline(
        ReadLogFromXes(),
        RemoveEventsFromLogPredicate(filter_predicate),
        PosEntropyDirectFilter(0),
        BuildEventClassTree([first_class_extractor, second_class_extractor]),
        BuildEventClassGraph(graph_name='Events Graph'),
        SerializeGraph()
    )(path_to_xes)

    execute_test_with_gold(gold_path, serialized_graph(result))


@pytest.mark.skip(reason="unreadable gold")
def test_creating_graph_of_activities():
    log_path = console_app_method2_bxes_log_path()
    gold_path = os.path.join(gold_dir(), 'test_pipelines', 'test_creating_graph_of_activities.gold')
    _do_test_with_activities_graph(gold_path, log_path)


@pytest.mark.skip(reason="unreadable gold")
def test_creating_event_class_graph():
    log_path = console_app_method2_bxes_log_path()
    gold_path = os.path.join(gold_dir(), 'test_pipelines', 'test_creating_event_class_graph.gold')
    _do_test_with_event_graph(gold_path, log_path)
