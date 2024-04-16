from enum import Enum

from ..analysis.event_log_analysis_entropy import *
from ..analysis.event_log_info import calculate_events_in_each_trace
from ..mutations.event_log_mutations import remove_event_from_log


default_untouchable_func = lambda name: False


def next_removal_default_entropy_indirect(log: MyEventLog,
                                          laplace: bool = True,
                                          top_percent: float = 0.05,
                                          untouchable_event_func=default_untouchable_func) -> (str, float):
    min_event_name = None
    min_sum_entropy = 1e10

    entropies = calculate_laplace_or_default_entropies(log, laplace=laplace)
    sorted_entropies = list(sorted(entropies.items(), key=lambda x: x[1]))
    top = sorted_entropies[len(sorted_entropies) - int(len(sorted_entropies) * top_percent):]

    for key, _ in top:
        if untouchable_event_func(key):
            continue

        entropy = calculate_default_entropy_average(log, laplace, ignored_events={key})
        if entropy < min_sum_entropy:
            min_sum_entropy = entropy
            min_event_name = key

    return min_event_name, min_sum_entropy


def filter_events_default_entropy_indirect(log: MyEventLog,
                                           laplace: bool = True,
                                           threshold: float = 5,
                                           top_percent: float = 0.05,
                                           max_events_to_remove: int = int(1e9),
                                           untouchable_event_func=default_untouchable_func) -> MyEventLog:
    removed_events_count = 0
    while True:
        if len(log) == 0:
            return log

        if calculate_default_entropy_average(log, laplace) <= threshold:
            return log

        min_event_name, min_sum_entropy = next_removal_default_entropy_indirect(log, laplace, top_percent,
                                                                                untouchable_event_func)
        print(f'Removing {min_event_name} with entropy {min_sum_entropy}')
        log = remove_event_from_log(log, min_event_name)

        removed_events_count += 1
        if removed_events_count >= max_events_to_remove:
            return log


def _determine_next_event_to_remove_direct(log: MyEventLog,
                                           metric_func: Callable[[MyEventLog], dict[str, float]]) -> (str, float):
    metrics_for_events = metric_func(log)
    event_name = max(metrics_for_events, key=metrics_for_events.get)
    return event_name, metrics_for_events[event_name]


def _do_filter_by_metric_direct(log: MyEventLog,
                                threshold: float,
                                metric_func: Callable[[MyEventLog], dict[str, float]],
                                max_events_to_delete: int = 1e9) -> MyEventLog:
    removed_events_count = 0
    while True:
        if len(log) == 0:
            return log

        event_name, metric = _determine_next_event_to_remove_direct(log, metric_func)

        if metric > threshold:
            print(f'Removing {event_name} with metric {metric}')
            log = remove_event_from_log(log, event_name)
            removed_events_count += 1
            if removed_events_count == max_events_to_delete:
                return log
        else:
            return log


class TillEndFilteringMetricKind(Enum):
    BeforeOnly = 1
    AfterBeforeDelta = 2


def _do_filter_by_metric_direct_till_end(log: MyEventLog,
                                         metric_kind: TillEndFilteringMetricKind,
                                         metric_func: Callable[[MyEventLog], dict[str, float]]) -> list[(str, float)]:
    removals = []
    while True:
        if len(log) == 0:
            break

        metric_before_removal = sum(metric_func(log).values())
        event_name, metric = _determine_next_event_to_remove_direct(log, metric_func)

        if metric_kind == TillEndFilteringMetricKind.BeforeOnly:
            removals.append((event_name, metric_before_removal))

        print(f'Removing {event_name} with metric {metric}')
        log = remove_event_from_log(log, event_name)

        if metric_kind == TillEndFilteringMetricKind.AfterBeforeDelta:
            metric_after_removal = sum(metric_func(log).values())
            removals.append((event_name, metric_before_removal - metric_after_removal))
        elif metric_kind != TillEndFilteringMetricKind.BeforeOnly:
            raise ValueError()

    return removals


def next_event_to_remove_default_entropy_direct(log: MyEventLog,
                                                laplace: bool = True,
                                                untouchable_event_func=default_untouchable_func) -> (str, float):
    def metric_func(event_log: MyEventLog):
        events_to_metric = calculate_laplace_or_default_entropies(event_log, laplace)
        return _remove_untouchable_events_from(events_to_metric, untouchable_event_func)

    return _determine_next_event_to_remove_direct(log, metric_func)


def _remove_untouchable_events_from(events_to_metric: dict[str, float],
                                    untouchable_event_func: Callable[[str], bool]) -> dict[str, float]:
    keys_to_remove = [key for key in events_to_metric.keys() if untouchable_event_func(key)]
    for key in keys_to_remove:
        del events_to_metric[key]

    return events_to_metric


def filter_events_default_entropy_direct(log: MyEventLog,
                                         laplace: bool = True,
                                         threshold: float = 0,
                                         max_events_to_remove: int = int(1e9),
                                         untouchable_event_func=default_untouchable_func) -> MyEventLog:
    def metric_func(event_log: MyEventLog):
        events_to_metric = calculate_laplace_or_default_entropies(event_log, laplace)
        return _remove_untouchable_events_from(events_to_metric, untouchable_event_func)

    return _do_filter_by_metric_direct(log, threshold, metric_func, max_events_to_remove)


def next_event_remove_pos_entropy_direct(log: MyEventLog) -> (str, float):
    return _determine_next_event_to_remove_direct(log, calculate_position_entropies)


def filter_events_pos_entropy_direct(log: MyEventLog,
                                     threshold: float = 0,
                                     untouchable_event_func=default_untouchable_func,
                                     max_events_to_remove: int = int(1e9)) -> MyEventLog:
    def metric_func(event_log: MyEventLog):
        events_to_metric = calculate_position_entropies_fast(event_log)
        return _remove_untouchable_events_from(events_to_metric, untouchable_event_func)

    return _do_filter_by_metric_direct(log, threshold, metric_func, max_events_to_remove)


def next_event_remove_pos_entropy_indirect(log: MyEventLog,
                                           trace_counts: list[TraceCounts] = None,
                                           log_info: LogInformation = None,
                                           positions: PositionEntropyEventsPositions = None,
                                           untouchable_event_func=default_untouchable_func) -> (str, float):
    trace_counts = calculate_events_in_each_trace(log) if trace_counts is None else trace_counts
    log_info = create_log_information(log) if log_info is None else log_info
    positions = create_events_positions(log) if positions is None else positions

    min_entropy = 1e10
    min_event_name = None

    for event_name in log_info.events_count.keys():
        if untouchable_event_func(event_name):
            continue

        ignored_events = PositionEntropyIgnoredEvents({event_name}, trace_counts)
        pos_entropy_without_event = calculate_position_entropies_fast_sum(log, ignored_events, positions)
        if pos_entropy_without_event < min_entropy:
            min_entropy = pos_entropy_without_event
            min_event_name = event_name

    return min_event_name, min_entropy


def filter_events_pos_entropy_indirect(log: MyEventLog,
                                       threshold: float = 0,
                                       max_events_to_remove: int = int(1e9),
                                       untouchable_event_func=default_untouchable_func) -> MyEventLog:
    removed_events_count = 0
    while True:
        trace_counts = calculate_events_in_each_trace(log)
        info = create_log_information(log)
        positions = create_events_positions(log)

        if calculate_position_entropies_fast_sum(log, positions=positions) <= threshold:
            return log

        min_event_name, min_entropy = next_event_remove_pos_entropy_indirect(log, trace_counts, info, positions,
                                                                             untouchable_event_func)

        print(f'Removing event {min_event_name} with resulting log entropy {min_entropy}')
        log = remove_event_from_log(log, min_event_name)

        removed_events_count += 1
        if removed_events_count >= max_events_to_remove:
            return log


def filter_events_pos_entropy_indirect_till_end(log: MyEventLog,
                                                metric_kind: TillEndFilteringMetricKind,
                                                untouchable_event_func=default_untouchable_func) -> list[(str, float)]:
    removals = []
    while True:
        if len(log) == 0:
            break

        trace_counts = calculate_events_in_each_trace(log)
        info = create_log_information(log)
        positions = create_events_positions(log)
        pos_entropy_before_removal = calculate_position_entropies_fast_sum(log, positions=positions)
        min_event_name, min_entropy = next_event_remove_pos_entropy_indirect(log, trace_counts, info, positions,
                                                                             untouchable_event_func)

        print(f'Removing event {min_event_name} with resulting log entropy {min_entropy}')
        if metric_kind == TillEndFilteringMetricKind.BeforeOnly:
            removals.append((min_event_name, pos_entropy_before_removal))
        elif metric_kind == TillEndFilteringMetricKind.AfterBeforeDelta:
            removals.append((min_event_name, pos_entropy_before_removal - min_entropy))
        else:
            raise ValueError()

        log = remove_event_from_log(log, min_event_name)

    return removals


def filter_events_pos_entropy_direct_till_end(log: MyEventLog,
                                              metric_kind: TillEndFilteringMetricKind,
                                              untouchable_event_func=default_untouchable_func) -> list[(str, float)]:
    def metric_func(event_log: MyEventLog):
        events_to_metric = calculate_position_entropies_fast(event_log)
        return _remove_untouchable_events_from(events_to_metric, untouchable_event_func)

    return _do_filter_by_metric_direct_till_end(log, metric_kind, metric_func)
