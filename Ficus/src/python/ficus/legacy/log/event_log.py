from typing import Sequence, Union

from ..util import *


class MyEvent:
    def __init__(self, existing_values: dict[Any, Any] = None):
        self.payload: dict[Any, Any] = existing_values if existing_values is not None else {}

    def payload_items(self) -> list[(Any, Any)]:
        return list(self.payload.items())

    def __getitem__(self, key: Any):
        return self.payload[key]

    def __setitem__(self, key: Any, value: Any):
        self.payload[key] = value

    def __len__(self):
        return len(self.payload)

    def __delitem__(self, key):
        del self.payload[key]

    def __iter__(self):
        return iter(self.payload)

    def __str__(self):
        return str(self.payload)

    def __hash__(self):
        return calculate_dict_hash(self.payload)

    def __eq__(self, other):
        return hash(self) == hash(other)

    def __copy__(self):
        return MyEvent(dict(self.payload))

    def __deepcopy__(self, memodict={}):
        return MyEvent(deep_copy_dict(self.payload))


class MyEventsSequence(Sequence):
    def __init__(self, raw_events: Union[list[MyEvent]]):
        self.raw_events: list[MyEvent] = raw_events if raw_events is not None else []

    def __getitem__(self, index: int):
        return self.raw_events[index]

    def __iter__(self):
        return iter(self.raw_events)

    def __setitem__(self, key: int, value: MyEvent):
        self.raw_events[key] = value

    def __len__(self):
        return len(self.raw_events)

    def append(self, new_event: MyEvent):
        self.raw_events.append(new_event)

    def insert(self, new_event: MyEvent, index: int):
        self.raw_events.insert(index, new_event)

    def __hash__(self):
        return calculate_poly_hash_for_collection(self.raw_events)

    def __eq__(self, other):
        return hash(self) == hash(other)

    def __delitem__(self, key: int):
        del self.raw_events[key]


class MyTrace(MyEventsSequence):
    def __init__(self, events: Union[list[MyEvent]] = None, attributes: dict[str, Any] = None):
        super().__init__(events)
        self.attributes = attributes if attributes is not None else {}

    def __copy__(self):
        return MyTrace(copy.copy(self.raw_events))

    def __deepcopy__(self, memodict={}):
        return MyTrace(copy.deepcopy(self.raw_events))


class MyEventLog(Sequence):
    def __init__(self,
                 traces: list[MyTrace] = None,
                 attributes: dict[str, str] = None,
                 extensions: dict[str, dict[str, str]] = None,
                 classifiers: dict[str, list[str]] = None,
                 properties: dict[str, Any] = None,
                 global_values: dict[str, dict[str, Any]] = None):
        self.global_values = global_values if global_values is not None else {}
        self.properties = properties if properties is not None else {}
        self.classifiers = classifiers if classifiers is not None else {}
        self.extensions = extensions if extensions is not None else {}
        self.attributes = attributes if attributes is not None else {}
        self.traces = traces if traces is not None else []

    def append(self, trace: MyTrace):
        self.traces.append(trace)

    def __hash__(self):
        current_hash = combine_list_of_hashes(list(map(hash, self.traces)))
        current_hash = combine_two_hashes(current_hash, calculate_dict_hash(self.attributes))
        current_hash = combine_two_hashes(current_hash, calculate_dict_hash(self.extensions))
        current_hash = combine_two_hashes(current_hash, calculate_dict_hash(self.classifiers))
        current_hash = combine_two_hashes(current_hash, calculate_dict_hash(self.properties))
        return current_hash

    def __eq__(self, other):
        return hash(self) == hash(other)

    def __getitem__(self, item: int) -> MyTrace:
        return self.traces[item]

    def __setitem__(self, key: int, value: MyTrace):
        self.traces[key] = value

    def __len__(self):
        return len(self.traces)

    def __delitem__(self, key: int):
        del self.traces[key]

    def __copy__(self):
        return MyEventLog(traces=[trace for trace in self.traces],
                          attributes=dict(self.attributes),
                          extensions=dict(self.extensions),
                          classifiers=dict(self.classifiers),
                          properties=dict(self.properties))

    def __deepcopy__(self, memodict={}):
        return MyEventLog(traces=[copy.deepcopy(trace) for trace in self.traces],
                          attributes=deep_copy_dict(self.attributes),
                          extensions=deep_copy_dict(self.extensions),
                          classifiers=deep_copy_dict(self.classifiers),
                          properties=deep_copy_dict(self.properties))
