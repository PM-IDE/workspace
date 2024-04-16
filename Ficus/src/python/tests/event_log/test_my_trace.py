import copy

from ...ficus.legacy.log.event_log import MyTrace, MyEvent


def test_my_trace_empty():
    trace = _create_empty_trace()
    _assert_empty_trace(trace)


def _assert_empty_trace(trace: MyTrace):
    assert len(trace) == 0
    assert hash(trace) == 0
    assert trace == MyTrace()
    assert MyTrace() == trace
    assert hash(MyTrace()) == hash(trace)


def _create_empty_trace() -> MyTrace:
    return MyTrace()


def test_my_trace_append():
    trace = _create_empty_trace()
    _assert_empty_trace(trace)
    event = MyEvent({'asdasd': 'asdsadasdasdsasd'})
    trace.append(event)

    second_trace = MyTrace([event])
    assert len(trace) == 1
    assert trace[0] == event
    assert hash(trace) == hash(second_trace)
    assert hash(trace) != 0
    assert hash(second_trace) != 0
    assert trace == second_trace
    assert len(second_trace) == 1


def _create_trace_with_one_event() -> MyTrace:
    return MyTrace([MyEvent({'asdaads': 'asdsadadaadaasdsd'})])


def test_my_trace_clone():
    trace = _create_trace_with_one_event()
    trace_copy = copy.copy(trace)

    assert trace[0] == trace_copy[0]
    assert trace[0] is trace_copy[0]
    assert hash(trace) == hash(trace_copy)
    assert len(trace) == len(trace_copy)

    new_key = 'asdadasdasd'
    new_value = 'asdasdasdasdasdasd'
    trace[0][new_key] = new_value

    assert trace[0][new_key] == new_value
    assert trace_copy[0][new_key] == new_value


def test_my_trace_deep_clone():
    trace = _create_trace_with_one_event()
    trace_copy = copy.deepcopy(trace)

    assert trace[0] == trace_copy[0]
    assert trace[0] is not trace_copy[0]
    assert hash(trace[0]) == hash(trace_copy[0])
    assert hash(trace) == hash(trace_copy)
    assert len(trace) == len(trace_copy)

    new_key = 'asdadasdasd'
    new_value = 'asdasdasdasdasdasd'
    trace[0][new_key] = new_value

    assert trace[0][new_key] == new_value
    assert new_key not in trace_copy[0]


def test_my_trace_deletion():
    trace = _create_trace_with_one_event()
    del trace[0]
    _assert_empty_trace(trace)


def test_my_trace_set_item():
    trace = _create_trace_with_one_event()
    event = MyEvent({'asdasd': 1})
    trace[0] = event
    assert trace[0] == event
    assert trace[0] is event
    assert len(trace) == 1
