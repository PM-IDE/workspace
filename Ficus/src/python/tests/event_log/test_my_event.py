import copy

from ...ficus.legacy.log.event_log import MyEvent


def test_my_event_empty():
  event = MyEvent()
  _assert_empty_event(event)


def _assert_empty_event(event: MyEvent):
  assert len(event) == 0
  assert str(event) == str({})
  assert hash(event) == 0
  assert event == MyEvent()
  assert MyEvent() == MyEvent()


key = 'key'
value = 'value'


def _create_kv_event():
  return MyEvent({key: value})


def test_my_event_with_kv():
  event = _create_kv_event()
  assert len(event) == 1
  assert 'key' in event
  assert 'asdasd' not in event
  assert event[key] == value
  assert hash(event) == hash(MyEvent({key: value}))
  assert hash(MyEvent({key: value})) == hash(event)
  assert event == MyEvent({key: value})
  assert MyEvent({key: value}) == event
  assert str(event) == str({key: value})

  del event[key]
  _assert_empty_event(event)


def test_my_event_copy():
  event = _create_kv_event()
  event_copy = copy.copy(event)
  new_value = 'asdasdas'
  event_copy[key] = new_value
  assert event_copy[key] == new_value
  assert event[key] == value


def test_my_event_copy2():
  event = _create_kv_event()
  event[key] = {1: 2}
  event_copy = copy.copy(event)
  assert event[key] == event_copy[key]
  assert hash(event) == hash(event_copy)
  assert id(event[key]) == id(event_copy[key])
  assert event[key] is event_copy[key]


def test_my_event_copy3():
  event = _create_kv_event()
  event[key] = {1: {1: 2}}
  event_copy = copy.copy(event)
  event_copy[key][1][1] = 3
  assert event[key] == event_copy[key]
  assert hash(event) == hash(event_copy)
  assert id(event[key]) == id(event_copy[key])
  assert event[key] is event_copy[key]
  assert str(event) == "{'key': {1: {1: 3}}}"
  assert str(event_copy) == "{'key': {1: {1: 3}}}"


def test_my_event_deep_copy():
  event = _create_kv_event()
  event[key] = {1: 2}
  event_copy = copy.deepcopy(event)
  assert event[key] == event_copy[key]
  assert hash(event) == hash(event_copy)
  assert id(event[key]) != id(event_copy[key])
  assert event[key] is not event_copy[key]


def test_my_event_deepcopy2():
  event = _create_kv_event()
  event[key] = {1: {1: 2}}
  event_copy = copy.deepcopy(event)
  event_copy[key][1][1] = 3
  assert event[key] != event_copy[key]
  assert hash(event) != hash(event_copy)
  assert id(event[key]) != id(event_copy[key])
  assert event[key] is not event_copy[key]
  assert str(event) == "{'key': {1: {1: 2}}}"
  assert str(event_copy) == "{'key': {1: {1: 3}}}"
