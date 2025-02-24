import os.path
from os.path import *
from typing import Iterable


def data_dir() -> str:
  return os.path.join(dirname(dirname(abspath(os.path.curdir))), 'test_data')


def sources_dir() -> str:
  return os.path.join(data_dir(), 'source')


def gold_dir() -> str:
  return os.path.join(data_dir(), 'gold', 'python')


def petri_nets_gold_dir() -> str:
  return os.path.join(gold_dir(), 'petri_nets')


def petri_net_test_gold_dir(test_name: str) -> str:
  return os.path.join(petri_nets_gold_dir(), test_name)


def example_logs_dir() -> str:
  return os.path.join(sources_dir(), 'example_logs')


def get_example_log_path(log_name: str) -> str:
  return os.path.join(example_logs_dir(), log_name)


def all_example_logs() -> list[str]:
  return [join(example_logs_dir(), f) for f in os.listdir(example_logs_dir()) if
          isfile(join(example_logs_dir(), f)) and f.endswith('.xes')]


def repair_logs_dir() -> str:
  return os.path.join(sources_dir(), 'repair_logs')


def get_repair_example_path() -> str:
  return os.path.join(repair_logs_dir(), 'repairExample.xes')


def console_app_method2_bxes_log_path() -> str:
  return os.path.join(data_dir(), 'source', 'solutions_logs', 'consoleapp1.bxes')


def array_pooling_bxes_log_path() -> str:
  return os.path.join(data_dir(), 'source', 'solutions_logs', 'arraypooling.bxes')
