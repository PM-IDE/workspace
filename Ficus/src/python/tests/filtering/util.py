from ...ficus.legacy.log.event_log import MyEventLog
from ...ficus.legacy.analysis.event_log_info import create_log_information
from ...ficus.legacy.mutations.event_log_mutations import remove_event_from_log


def create_sequence_of_removals(log: MyEventLog, next_event_to_remove_func) -> list[str]:
  info = create_log_information(log)
  sequence_of_removed_events = []

  for _ in range(len(info.events_count.keys())):
    event_to_remove, entropy = next_event_to_remove_func(log)
    sequence_of_removed_events.append(event_to_remove)
    log = remove_event_from_log(log, event_to_remove)

  return sequence_of_removed_events
