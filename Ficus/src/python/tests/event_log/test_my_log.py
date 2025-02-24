import copy

from ...ficus.legacy.log.event_log import MyEventLog, MyTrace, MyEvent


def test_my_log_empty():
  log = _create_empty_log()
  _assert_empty_log(log)


def _assert_empty_log(log: MyEventLog):
  assert len(log) == 0
  assert log == _create_empty_log()
  assert MyEventLog() == log
  assert hash(MyEventLog()) == hash(MyEventLog())
  assert hash(log) == hash(MyEventLog())


def _create_empty_log() -> MyEventLog:
  return MyEventLog()


def _create_log_with_one_trace():
  return MyEventLog([MyTrace([MyEvent({1: 2})])])


def test_my_log_append():
  log = _create_empty_log()
  _assert_empty_log(log)
  trace = MyTrace([MyEvent({1: 2})])
  log.append(trace)

  assert log[0] == trace
  assert log[0] is trace
  assert len(log) == 1
  assert log != _create_empty_log()
  assert log is not _create_empty_log()


def test_my_log_delete():
  log = _create_log_with_one_trace()
  assert len(log) == 1
  del log[0]
  assert len(log) == 0
  assert log == _create_empty_log()
  assert hash(log) == hash(_create_empty_log())


def test_my_log_copy():
  log = _create_log_with_one_trace()
  log_copy = copy.copy(log)

  assert log == log_copy
  assert log is not log_copy
  assert log[0] is log_copy[0]
  assert hash(log) == hash(log_copy)
  assert log[0][0] is log_copy[0][0]
  assert hash(log[0][0]) == hash(log_copy[0][0])

  del log[0]

  assert len(log) == 0
  assert len(log_copy) == 1


def test_my_log_copy2():
  log = _create_log_with_one_trace()
  log_copy = copy.copy(log)

  assert log == log_copy
  assert hash(log) == hash(log_copy)
  log[0].append(MyEvent())

  assert len(log[0]) == 2
  assert len(log_copy[0]) == 2
  assert len(log) == 1
  assert len(log_copy) == 1
  assert hash(log) == hash(log_copy)
  assert log == log_copy
  assert log is not log_copy


def test_my_log_copy3():
  log = _create_log_with_one_trace()
  log_copy = copy.copy(log)

  assert log == log_copy
  assert hash(log) == hash(log_copy)
  assert log is not log_copy

  new_key = 'asdasd'
  new_value = 'asdas'
  log[0][0][new_key] = new_value

  assert log[0][0] == log_copy[0][0]
  assert log[0][0] is log_copy[0][0]
  assert hash(log[0][0]) == hash(log_copy[0][0])
  assert new_key in log[0][0]
  assert new_key in log_copy[0][0]
  assert log_copy[0][0][new_key] == new_value


def test_my_log_deepcopy():
  log = _create_log_with_one_trace()
  log_copy = copy.deepcopy(log)

  assert log == log_copy
  assert log is not log_copy
  assert hash(log) == hash(log_copy)
  assert log[0] is not log_copy[0]
  assert log[0] == log_copy[0]
  assert hash(log[0]) == hash(log[0])
  assert log[0][0] is not log_copy[0][0]
  assert log[0][0] == log[0][0]
  assert hash(log[0][0]) == hash(log[0][0])

  del log[0]

  assert len(log) == 0
  assert len(log_copy) == 1


def test_my_log_deepcopy2():
  log = _create_log_with_one_trace()
  log_copy = copy.deepcopy(log)

  assert log == log_copy
  assert hash(log) == hash(log_copy)
  log[0].append(MyEvent())

  assert len(log[0]) == 2
  assert len(log_copy[0]) == 1
  assert len(log) == 1
  assert len(log_copy) == 1
  assert hash(log) != hash(log_copy)
  assert log != log_copy
  assert log is not log_copy


def test_my_log_deepcopy3():
  log = _create_log_with_one_trace()
  log_copy = copy.deepcopy(log)

  assert log == log_copy
  assert hash(log) == hash(log_copy)
  assert log is not log_copy

  new_key = 'asdasd'
  new_value = 'asdas'
  log[0][0][new_key] = new_value

  assert log[0][0] != log_copy[0][0]
  assert log[0][0] is not log_copy[0][0]
  assert hash(log[0][0]) != hash(log_copy[0][0])
  assert new_key in log[0][0]
  assert new_key not in log_copy[0][0]
