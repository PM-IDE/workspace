import uuid

from ..log.event_log import *
from ..util import *


def split_log_by_traces(log: MyEventLog) -> list[list[MyTrace]]:
    traces_by_length = dict()
    result = []

    for trace in log:
        if len(trace) in traces_by_length:
            traces_by_length[len(trace)].append(trace)
        else:
            traces_by_length[len(trace)] = [trace]

    for _, traces in traces_by_length.items():
        if len(traces) == 1:
            result.append([traces[0]])
            continue

        groups = [[trace for trace in traces]]
        index = 0
        while True:
            if index >= len(traces[0]):
                break

            new_groups = []
            all_groups_has_one_trace = True
            for group in groups:
                if len(group) == 1:
                    new_groups.append(group)
                    continue

                all_groups_has_one_trace = False
                hashes2traces = dict()
                for trace in group:
                    hash_code = calculate_string_poly_hash(trace[index][concept_name])
                    if hash_code in hashes2traces:
                        hashes2traces[hash_code].append(trace)
                    else:
                        hashes2traces[hash_code] = [trace]

                for _, new_group in hashes2traces.items():
                    new_groups.append(new_group)

            if all_groups_has_one_trace:
                break

            index += 1
            groups = new_groups

        for group in groups:
            result.append(group)

    return result


def merge_all_traces_in_one_log(log: MyEventLog) -> MyEventLog:
    new_log = MyEventLog()
    new_trace = MyTrace()
    new_log.append(new_trace)

    for trace in log:
        for event in trace:
            new_trace.append(copy.copy(event))
        new_trace.append(create_unique_event())

    return new_log


def create_unique_event() -> MyEvent:
    unique_event = MyEvent()
    unique_event[concept_name] = str(uuid.uuid4())
    return unique_event
