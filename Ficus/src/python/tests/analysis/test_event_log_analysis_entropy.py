import os.path

from ..test_data_provider import sources_dir
from ...ficus.legacy.analysis.event_log_analysis_entropy import calculate_default_entropies, calculate_laplace_entropies
from ...ficus.legacy.log.functions import read_log_from_xes
from ...tests import log_creators


def test_log_from_paper_with_ignore():
  log = log_creators.create_log_from_filter_out_chaotic_events_with_noise()
  entropy = calculate_default_entropies(log, ignored_events={'d', 'v'})
  expected_result = {
    'a': 0.9182958340544896,
    'b': 1.8365916681089791,
    'c': 1.8365916681089791,
    'x': 3.169925001442312
  }

  assert entropy == expected_result


def test_log_from_paper():
  log = log_creators.create_log_from_filter_out_chaotic_events()
  entropy = calculate_default_entropies(log)
  expected_result = {
    'a': 0.9182958340544896,
    'b': 1.8365916681089791,
    'c': 1.8365916681089791,
    'x': 3.169925001442312
  }

  assert len(expected_result) == len(entropy)
  assert entropy == expected_result


def test_log_from_paper_laplace():
  log = log_creators.create_log_from_filter_out_chaotic_events()
  entropy = calculate_laplace_entropies(log)
  expected_result = {
    'a': 1.002726083454847,
    'b': 1.905904975406124,
    'c': 1.905904975406124,
    'x': 3.2127002996007796
  }

  assert len(expected_result) == len(entropy)
  assert entropy == expected_result


def test():
  source = [
    ('exercise1.xes', {'A': 1.584962500721156, 'E': 0.0, 'D': 1.584962500721156, 'C': 2.0, 'B': 2.0}),
    ('exercise2.xes', {'B': 0.0, 'C': 2.0, 'E': 0.0, 'A': 0.0, 'D': 0.0}),
    ('exercise3.xes', {'B': 1.0, 'D': 2.0, 'F': 2.0, 'G': 2.0, 'A': 1.0, 'E': 2.0, 'C': 2.0}),
  ]

  for (log_file, expected_entropy) in source:
    log = read_log_from_xes(os.path.join(sources_dir(), 'example_logs', log_file))
    entropy = calculate_default_entropies(log)
    assert entropy == expected_entropy
