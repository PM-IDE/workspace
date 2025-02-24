from ...ficus.legacy.analysis.event_log_analysis_entropy import *
from ...ficus.legacy.analysis.event_log_info import *
from ...ficus.legacy.mutations.event_log_mutations import remove_events_from_log
from ...tests import log_creators


def test_log_from_paper_brute_force():
  log = log_creators.create_log_from_filter_out_chaotic_events()
  entropy = calculate_position_entropies(log)
  expected_result = {
    'a': 0.0,
    'b': 0.2211099839259014,
    'c': 0.2211099839259014,
    'x': 0.3230075074711545
  }

  assert len(expected_result) == len(entropy)
  assert entropy == expected_result


def test_log_from_paper_fast():
  log = log_creators.create_log_from_filter_out_chaotic_events()
  entropy = calculate_position_entropies_fast(log)
  expected_result = {
    'a': 0.0,
    'b': 0.2211099839259014,
    'c': 0.2211099839259014,
    'x': 0.3230075074711545
  }

  assert len(expected_result) == len(entropy)
  assert entropy == expected_result


def test_log_from_paper_fast_with_noise():
  log = log_creators.create_log_from_filter_out_chaotic_events_with_noise()
  ignored_events = PositionEntropyIgnoredEvents({'d', 'v'}, calculate_events_in_each_trace(log))
  entropy = calculate_position_entropies_fast(log, ignored_events)
  expected_result = {
    'a': 0.0,
    'b': 0.2211099839259014,
    'c': 0.2211099839259014,
    'x': 0.3230075074711545
  }

  assert len(expected_result) == len(entropy)
  assert entropy == expected_result


def test_fast_and_default_entropy_equivalence():
  for log in log_creators.enumerate_example_logs():
    event_counts = create_log_information(log)
    traces_count = calculate_events_in_each_trace(log)

    for first_key in event_counts.events_count.keys():
      for second_key in event_counts.events_count.keys():
        if first_key != second_key:
          new_log = remove_events_from_log(log, {first_key, second_key})
          default_entropies_new_log = calculate_position_entropies(new_log)
          ignored_events = PositionEntropyIgnoredEvents({first_key, second_key}, traces_count)

          default_entropies = calculate_position_entropies(log, ignored_events)
          assert len(default_entropies) == len(default_entropies_new_log)
          assert default_entropies == default_entropies_new_log

          fast_entropies = calculate_position_entropies_fast(log, ignored_events)
          assert len(fast_entropies) == len(default_entropies_new_log)
          assert fast_entropies == default_entropies_new_log

          assert len(default_entropies) == len(fast_entropies)
          assert default_entropies == fast_entropies
