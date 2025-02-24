from _pytest.outcomes import fail

from ..test_data_provider import get_example_log_path
from ...ficus.legacy.pipelines.contexts.accessors import log
from ...ficus.legacy.pipelines.contexts.keys import log_key
from ...ficus.legacy.pipelines.pipelines import Pipeline
from ...ficus.legacy.pipelines.start.start_parts import ReadLogFromXes


def test_read_log_from_xes():
  result = Pipeline(
    ReadLogFromXes()
  )(get_example_log_path('exercise1.xes'))

  assert result.has_value(log_key)
  assert log(result) is not None


def test_read_not_existing_log():
  try:
    Pipeline(
      ReadLogFromXes()
    )('not_existing_path')
  except FileNotFoundError:
    return

  fail()
