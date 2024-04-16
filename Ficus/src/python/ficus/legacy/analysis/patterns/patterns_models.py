from dataclasses import dataclass
from enum import Enum

from ..type_aliases import ClassExtractor
from ...analysis.common.common_models import SubArrayInEventLog, GraphNode
from ...util import calculate_string_poly_hash


@dataclass
class TandemArrayInfo(SubArrayInEventLog):
    repeat_count: int


@dataclass
class MaximalRepeatInfo(SubArrayInEventLog):
    pass


@dataclass
class SubArrayWithTraceIndex(SubArrayInEventLog):
    trace_index: int


unique_activity_node_index = 0

class ActivityNode(GraphNode):
    def __init__(self,
                 name: str,
                 repeat_set: SubArrayWithTraceIndex,
                 set_of_events: set[int],
                 class_extractor: ClassExtractor,
                 activity_level: int):
        super().__init__(name)
        self.activity_level = activity_level
        self.name = name
        self.class_extractor = class_extractor
        self.repeat_set: SubArrayWithTraceIndex = repeat_set
        self.set_of_events: set[int] = set_of_events
        self.length = len(self.set_of_events)

        global unique_activity_node_index
        self._unique_index = unique_activity_node_index
        unique_activity_node_index += 1

    def contains_other(self, other: 'ActivityNode') -> bool:
        return self.set_of_events.issuperset(other.set_of_events)

    def __hash__(self):
        return calculate_string_poly_hash(self.name)

    def _serialize(self):
        return str(sorted(list(self.set_of_events)))

    def unique_name(self):
        return f'Activity_{self._unique_index}'


@dataclass
class ActivityInTraceInfo:
    node: ActivityNode
    start_pos: int
    length: int

    def __str__(self):
        return f'({str(self.node)}, {self.start_pos}, {self.length})'


class UndefinedActivityHandlingStrategy(Enum):
    DontInsert = 0
    InsertAsSingleEvent = 1
    InsertAllEvents = 2


@dataclass
class NewUnattachedActivities:
    activities_in_traces: list[list[ActivityInTraceInfo]]
    activities_nodes: list[ActivityNode]


class EventClassNode(GraphNode):
    def __init__(self, name: str):
        super().__init__(name)

    def _serialize(self):
        return self.name
