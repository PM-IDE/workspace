import math
from dataclasses import dataclass
from typing import Callable

import numpy
import numpy as np
import pandas as pd
from matplotlib import pyplot as plt

from .event_log_info import TraceCounts, create_log_information, LogInformation
from ..log.event_log import MyEventLog
from ..util import concept_name, fake_end_name, fake_start_name


def calculate_laplace_or_default_entropies(log: MyEventLog,
                                           laplace: bool,
                                           ignored_events: set[str] = None) -> dict[str, float]:
    if laplace:
        return calculate_laplace_entropies(log, ignored_events=ignored_events)

    return calculate_default_entropies(log, ignored_events=ignored_events)


def _do_calculate_entropy_sum(log: MyEventLog, entropy_func: Callable[[MyEventLog], dict[str, float]]) -> float:
    return sum(entropy_func(log).values())


def _do_calculate_entropy_avg(log: MyEventLog, entropy_func: Callable[[MyEventLog], dict[str, float]]) -> float:
    entropies = entropy_func(log)
    if len(entropies) == 0:
        return 0

    return sum(entropies.values()) / len(entropies)


def calculate_default_entropy_sum(log: MyEventLog,
                                  laplace: bool = True,
                                  ignored_events: set[str] = None) -> float:
    func = lambda evt_log: calculate_laplace_or_default_entropies(evt_log, laplace, ignored_events)
    return _do_calculate_entropy_sum(log, func)


def calculate_default_entropy_average(log: MyEventLog,
                                      laplace: bool = True,
                                      ignored_events: set[str] = None) -> float:
    func = lambda evt_log: calculate_laplace_or_default_entropies(evt_log, laplace, ignored_events)
    return _do_calculate_entropy_avg(log, func)


def create_entropies_df(log: MyEventLog) -> pd.DataFrame:
    entropy = calculate_default_entropies(log)
    df = pd.DataFrame(entropy.values(), index=list(entropy.keys()), columns=['Entropy'])
    return df.sort_values(by='Entropy', ascending=False)


def calculate_default_entropies(log: MyEventLog,
                                ignored_events: set[str] = None) -> dict[str, float]:
    def dfr(first: str, second: str, context: LogInformation) -> float:
        return context.dfg.pairs.get((first, second), 0) / context.events_count[first]

    def dpr(first: str, second: str, context: LogInformation) -> float:
        return context.dfg.pairs.get((second, first), 0) / context.events_count[first]

    return _calculate_entropies(log, dfr, dpr, ignored_events)


def calculate_laplace_entropies(log: MyEventLog,
                                ignored_events: set[str] = None) -> dict[str, float]:
    def dfr_or_dpr(pair_count: int, events_count: int, event_count: int) -> float:
        alpha = 1 / event_count
        x = alpha + pair_count
        y = alpha * (events_count + 1) + event_count
        return x / y

    def dfr(first: str, second: str, context: LogInformation) -> float:
        pair_count = context.dfg.pairs.get((first, second), 0)
        return dfr_or_dpr(pair_count, len(context.events_count), context.events_count[first])

    def dpr(first: str, second: str, context: LogInformation) -> float:
        pair_count = context.dfg.pairs.get((second, first), 0)
        return dfr_or_dpr(pair_count, len(context.events_count), context.events_count[first])

    return _calculate_entropies(log, dfr, dpr, ignored_events)


def _calculate_entropy(vector: list[float]) -> float:
    e = 0.0
    for item in vector:
        if item != 0:
            e -= (item * math.log2(item))

    return e


def _calculate_entropies(log: MyEventLog,
                         dfr_func: Callable[[str, str, LogInformation], float],
                         dpr_func: Callable[[str, str, LogInformation], float],
                         ignored_events: set[str] = None) -> dict[str, float]:
    log_info = create_log_information(log, ignored_events)
    (pairs, events_count) = (log_info.dfg.pairs, log_info.events_count)

    entropy = dict()
    list_of_events = list(events_count.keys())
    dfr_list_of_events = list_of_events + [fake_end_name]
    dpr_list_of_events = list_of_events + [fake_start_name]

    for (event_name, count) in events_count.items():
        dfr_vector = [dfr_func(event_name, x, log_info) for x in dfr_list_of_events]
        dpr_vector = [dpr_func(event_name, x, log_info) for x in dpr_list_of_events]

        entropy[event_name] = _calculate_entropy(dfr_vector) + _calculate_entropy(dpr_vector)

    return entropy


@dataclass
class PositionEntropyIgnoredEvents:
    ignored_events: set[str]
    traces_counts: list[TraceCounts]

    def calculate_max_vector_length(self) -> int:
        max_trace_length = 0
        for trace_counts in self.traces_counts:
            number_of_removed_events = sum(map(lambda x: trace_counts.counts.get(x, 0), self.ignored_events))
            max_trace_length = max(max_trace_length, trace_counts.overall_count - number_of_removed_events)

        return max_trace_length


@dataclass
class TraceEventsPositionInfo:
    positions: dict[str, list[int]]
    length: int


@dataclass
class PositionEntropyEventsPositions:
    traces_positions: list[TraceEventsPositionInfo]


def create_events_positions(log: MyEventLog):
    traces_positions = []
    for trace in log:
        event_positions = dict()
        for index, event in enumerate(trace):
            event_name = event[concept_name]
            if event_name in event_positions:
                event_positions[event_name].append(index)
            else:
                event_positions[event_name] = [index]

        traces_positions.append(TraceEventsPositionInfo(event_positions, len(trace)))

    return PositionEntropyEventsPositions(traces_positions)


def calculate_position_entropy_fast(log: MyEventLog,
                                    event_name: str,
                                    events_to_ignore: PositionEntropyIgnoredEvents = None,
                                    cached_positions: PositionEntropyEventsPositions = None) -> float:
    if cached_positions is None:
        cached_positions = create_events_positions(log)

    max_vector_length = _calculate_max_vector_length(log, events_to_ignore)
    probabilities = [0] * max_vector_length

    non_empty_traces_count = 0
    ordered_positions_of_ignored_events = []
    for trace_positions in cached_positions.traces_positions:
        ordered_positions_of_ignored_events.clear()

        if events_to_ignore is not None:
            for ignored_event in events_to_ignore.ignored_events:
                positions_of_ignored_event = trace_positions.positions.get(ignored_event, None)
                if positions_of_ignored_event is not None:
                    ordered_positions_of_ignored_events.extend(positions_of_ignored_event)

        if len(ordered_positions_of_ignored_events) == trace_positions.length:
            continue

        non_empty_traces_count += 1

        positions_of_our_event = trace_positions.positions.get(event_name, None)
        if positions_of_our_event is None:
            continue

        ordered_positions_of_ignored_events.sort()

        our_idx = 0
        ignored_idx = 0
        while our_idx != len(positions_of_our_event) or ignored_idx != len(ordered_positions_of_ignored_events):
            if our_idx >= len(positions_of_our_event):
                break

            while ignored_idx < len(ordered_positions_of_ignored_events) and positions_of_our_event[our_idx] > ordered_positions_of_ignored_events[ignored_idx]:
                ignored_idx += 1

            probabilities[positions_of_our_event[our_idx] - ignored_idx] += 1
            our_idx += 1

    probabilities = np.array(probabilities) / non_empty_traces_count
    entropy = _do_calculate_position_entropy(probabilities, non_empty_traces_count)
    return entropy


def calculate_position_entropies_fast(log: MyEventLog,
                                      events_to_ignore: PositionEntropyIgnoredEvents = None,
                                      positions: PositionEntropyEventsPositions = None) -> dict[str, float]:
    if positions is None:
        positions = create_events_positions(log)

    result = dict()
    ignored_events = events_to_ignore.ignored_events if events_to_ignore is not None else None

    for key in create_log_information(log, ignored_events).events_count.keys():
        result[key] = calculate_position_entropy_fast(log, key, events_to_ignore, positions)

    return result


def _do_calculate_position_entropy(probabilities: np.array, traces_count: int) -> float:
    probabilities = probabilities[probabilities != 0]
    logs = np.log(probabilities) / np.log(traces_count)
    return np.sum(-logs) / len(probabilities)


def _calculate_max_vector_length(log: MyEventLog, events_to_ignore: PositionEntropyIgnoredEvents = None) -> int:
    if events_to_ignore is not None:
        return events_to_ignore.calculate_max_vector_length()

    return max(list(map(lambda trace: len(trace), log)))


def calculate_position_entropy(log: MyEventLog,
                               event_name: str,
                               events_to_ignore: PositionEntropyIgnoredEvents = None) -> float:
    max_vector_length = _calculate_max_vector_length(log, events_to_ignore)
    vectors = [np.zeros(max_vector_length) for _ in range(len(log))]
    vector_index = 0
    non_empty_traces_count = 0

    for trace in log:
        index = 0
        empty_trace = True
        for event in trace:
            if events_to_ignore is not None and event[concept_name] in events_to_ignore.ignored_events:
                continue

            empty_trace = False
            if event[concept_name] == event_name:
                vectors[vector_index][index] = 1

            index += 1

        if not empty_trace:
            non_empty_traces_count += 1

        vector_index += 1

    probabilities = np.sum(vectors, axis=0) / non_empty_traces_count
    return _do_calculate_position_entropy(probabilities, non_empty_traces_count)


def calculate_position_entropies(log: MyEventLog,
                                 events_to_ignore: PositionEntropyIgnoredEvents = None) -> dict[str, float]:
    result = dict()
    ignored_events = events_to_ignore.ignored_events if events_to_ignore is not None else None

    for key in create_log_information(log, ignored_events).events_count.keys():
        result[key] = calculate_position_entropy(log, key, events_to_ignore)

    return result


def calculate_position_entropies_sum(log: MyEventLog,
                                     ignored_events: PositionEntropyIgnoredEvents = None) -> float:
    return _do_calculate_entropy_sum(log, lambda evt_log: calculate_position_entropies(log, ignored_events))


def calculate_position_entropies_fast_sum(log: MyEventLog,
                                          ignored_events: PositionEntropyIgnoredEvents = None,
                                          positions: PositionEntropyEventsPositions = None) -> float:
    func = lambda evt_log: calculate_position_entropies_fast(evt_log, ignored_events, positions)
    return _do_calculate_entropy_sum(log, func)


def draw_position_entropies_histogram(log: MyEventLog):
    position_entropies = calculate_position_entropies_fast(log)
    plt.hist(position_entropies.values())


def calculate_pos_entropy_var(log: MyEventLog):
    return numpy.var(list(calculate_position_entropies_fast(log).values()))


def calculate_pos_entropy_std(log: MyEventLog):
    return numpy.std(list(calculate_position_entropies_fast(log).values()))
