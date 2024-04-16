from dataclasses import dataclass

from ..log.event_log import MyEventLog, MyEvent
from ..util import *


class DirectlyFollowsGraph:
    def __init__(self, pairs: dict[(str, str), int]):
        self.pairs: dict[(str, str), int] = pairs
        self.followed_events: dict[str, dict[str, int]] = dict()
        self.events_with_single_follower: set[str] = set()

        for first, second in pairs.keys():
            if first in self.followed_events:
                if first in self.events_with_single_follower:
                    self.events_with_single_follower.remove(first)

                if second not in self.followed_events[first]:
                    self.followed_events[first][second] = pairs[(first, second)]
            else:
                self.followed_events[first] = {second: pairs[(first, second)]}
                self.events_with_single_follower.add(first)


@dataclass
class LogInformation:
    dfg: DirectlyFollowsGraph
    events_count: dict[str, int]


def create_dfg(log: MyEventLog) -> DirectlyFollowsGraph:
    return create_log_information(log, add_fake_start_end_events=False).dfg


def create_log_information(log: MyEventLog,
                           ignored_events: set[str] = None,
                           add_fake_start_end_events: bool = True) -> LogInformation:
    pairs = dict()
    events_count = dict()

    def update_event_count(current_event_name):
        increase_in_int_map(events_count, current_event_name)

    def update_pairs_count(first_event_name, second_event_name):
        names_tuple = (first_event_name, second_event_name)
        increase_in_int_map(pairs, names_tuple)

    for trace in log:
        prev_event = None
        for event in trace:
            current_name = event[concept_name]
            if ignored_events is not None and current_name in ignored_events:
                continue

            update_event_count(current_name)

            if prev_event is None:
                prev_event = event
                if add_fake_start_end_events:
                    update_pairs_count(fake_start_name, current_name)

                continue

            prev_name = prev_event[concept_name]

            update_pairs_count(prev_name, current_name)
            prev_event = event

        if add_fake_start_end_events and prev_event is not None:
            update_pairs_count(prev_event[concept_name], fake_end_name)

    return LogInformation(DirectlyFollowsGraph(pairs), events_count)


@dataclass
class TraceCounts:
    counts: dict[str, int]
    overall_count: int


def calculate_events_in_each_trace(log: MyEventLog) -> list[TraceCounts]:
    counts = []
    for trace in log:
        trace_counts = dict()
        for event in trace:
            increase_in_int_map(trace_counts, event[concept_name])

        counts.append(TraceCounts(trace_counts, len(trace)))

    return counts


def apply_to_all_events(log: MyEventLog, func_with_event: Callable[[MyEvent], None]):
    for trace in log:
        for event in trace:
            func_with_event(event)


def find_all_events_names(log: MyEventLog, predicate: Callable[[MyEvent], bool]) -> set[str]:
    predicate_not_hold = set()
    predicate_hold = set()

    for trace in log:
        for event in trace:
            event_name = event[concept_name]
            if event_name in predicate_not_hold:
                continue

            if not predicate(event):
                if event_name in predicate_hold:
                    predicate_hold.remove(event_name)

                predicate_not_hold.add(event_name)
                continue

            predicate_hold.add(event_name)

    return predicate_hold


def calculate_events_count(log: MyEventLog) -> int:
    return sum(map(len, log))
