from ...ficus.legacy.pipelines.contexts.accessors import *
from ...ficus.legacy.pipelines.contexts.keys import *
from ...ficus.legacy.pipelines.contexts.part_results import PipelinePartResult


def test_log():
    part_result = PipelinePartResult()
    empty_log = MyEventLog()
    part_result.with_log(empty_log)
    _assert(part_result, empty_log, log, log_key)


def _assert(part_result: PipelinePartResult, obj, accessor, key):
    assert part_result.has_value(key)
    assert accessor(part_result) == obj
    assert part_result.get_value_or_throw(key) == obj


def test_activities():
    part_result = PipelinePartResult()
    lst = []
    part_result.with_activities(lst)
    _assert(part_result, lst, activities, activities_key)


def test_repeat_sets():
    part_result = PipelinePartResult()
    obj = []
    part_result.with_repeat_sets(obj)
    _assert(part_result, obj, repeat_sets, repeat_sets_key)


def test_traces_activities():
    part_result = PipelinePartResult()
    obj = []
    part_result.with_trace_activities(obj)
    _assert(part_result, obj, traces_activities, traces_activities_key)


def test_patterns():
    part_result = PipelinePartResult()
    obj = [[]]
    part_result.with_patterns(obj)
    _assert(part_result, obj, patterns, patterns_key)


def test_petri_net():
    part_result = PipelinePartResult()
    obj = PetriNetWrapper(None, None, None)
    part_result.with_petri_net(obj)
    _assert(part_result, obj, petri_net, petri_net_key)


def test_event_class_tree():
    part_result = PipelinePartResult()
    obj = []
    part_result.with_event_class_tree(obj)
    _assert(part_result, obj, event_class_tree, event_class_tree_key)


def test_activities_to_logs():
    part_result = PipelinePartResult()
    obj = dict()
    part_result.with_activities_logs(obj)
    _assert(part_result, obj, activities_to_logs, activities_to_logs_key)


def test_serialized_graph():
    part_result = PipelinePartResult()
    obj = 'asdasd'
    part_result.with_serialized_graph(obj)
    _assert(part_result, obj, serialized_graph, serialized_graph_key)


def test_graph():
    part_result = PipelinePartResult()
    obj = graphviz.Digraph()
    part_result.with_graph(obj)
    _assert(part_result, obj, graph, graph_key)
