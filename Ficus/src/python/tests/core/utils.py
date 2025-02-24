from ...ficus.legacy.analysis.common.common_models import SubArrayInEventLog


def maximal_repeats_to_substrings(raw_events: list[str],
                                  repeat_set_by_traces: list[list[SubArrayInEventLog]]) -> list[list[str]]:
  substrings = []
  for raw_event_string, trace_repeats in zip(raw_events, repeat_set_by_traces):
    trace_substrings = []
    for repeat in trace_repeats:
      trace_substrings.append(raw_event_string[repeat.first_pos:(repeat.first_pos + repeat.length)])

    substrings.append(trace_substrings)

  return substrings
