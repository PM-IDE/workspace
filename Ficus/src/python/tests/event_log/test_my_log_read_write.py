import pm4py
import pytest

from ..core.gold_based_test import execute_test_with_gold
from ..log_creators import *
from ..test_data_provider import all_test_split_traces, gold_dir
from ...ficus.legacy.log.functions import *
from ...ficus.legacy.log.pm4py_converters import *


@pytest.mark.parametrize("log_path", all_example_logs(), indirect=True)
def test_read_examples_logs(log_path):
    _do_read_log_test(log_path)


def _do_read_log_test(log_path):
    my_log = read_log_from_xes(log_path)
    pm4py_log = pm4py.read_xes(log_path, return_legacy_log_object=True)
    converted_pm4py_log = from_pm4py_log(pm4py_log)
    assert my_log.classifiers == converted_pm4py_log.classifiers
    assert my_log.extensions == converted_pm4py_log.extensions
    assert my_log.global_values == converted_pm4py_log.global_values
    assert my_log.traces == converted_pm4py_log.traces


@pytest.mark.parametrize("log_path", all_test_split_traces(), indirect=True)
@pytest.mark.skip(reason="seems like pm4py dont parse UTC dates")
def test_read_test_split_traces_logs(log_path):
    _do_read_log_test(log_path)


@pytest.mark.parametrize("log_path", all_example_logs(), indirect=True)
def test_write_example_logs(log_path):
    my_log = read_log_from_xes(log_path)
    serialized_log = serialize_log_to_xes_beautiful(my_log)
    file_name = os.path.splitext(os.path.basename(log_path))[0]
    gold_path = os.path.join(gold_dir(), 'serialized_example_logs', f'{file_name}.gold')
    execute_test_with_gold(gold_path, serialized_log)


@pytest.mark.parametrize("log_path", all_test_split_traces(), indirect=True)
def test_write_test_split_logs(log_path):
    my_log = read_log_from_xes(log_path)
    serialized_log = serialize_log_to_xes_beautiful(my_log)
    file_name = os.path.splitext(os.path.basename(log_path))[0]
    folder_name = os.path.normpath(log_path).split(os.sep)[-2]
    gold_path = os.path.join(gold_dir(), 'serialized_test_split_logs', folder_name, f'{file_name}.gold')
    execute_test_with_gold(gold_path, serialized_log)


@pytest.fixture
def log_path(request):
    return request.param
