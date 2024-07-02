from dataclasses import dataclass
from typing import Optional

import pandas as pd

from ..legacy.analysis.event_log_analysis import ColoredRectangle, Color
from ..legacy.discovery.petri_net import Arc, Transition, Place, PetriNet, Marking, SinglePlaceMarking
from .constants import const_cluster_labels
from .models.pipelines_and_context_pb2 import *
from .models.pm_models_pb2 import *
from .models.util_pb2 import GrpcColor
from ..legacy.discovery.graph import Graph, GraphNode, GraphEdge


@dataclass
class ContextValue:
    def to_grpc_context_value(self) -> GrpcContextValue:
        pass


@dataclass
class StringContextValue(ContextValue):
    value: str

    def to_grpc_context_value(self) -> GrpcContextValue:
        return GrpcContextValue(string=self.value)


@dataclass
class FloatContextValue(ContextValue):
    value: float

    def to_grpc_context_value(self) -> GrpcContextValue:
        return GrpcContextValue(float=self.value)


@dataclass
class Uint32ContextValue(ContextValue):
    value: int

    def to_grpc_context_value(self) -> GrpcContextValue:
        return GrpcContextValue(uint32=self.value)


@dataclass
class BoolContextValue(ContextValue):
    value: bool

    def to_grpc_context_value(self) -> GrpcContextValue:
        return GrpcContextValue(bool=self.value)


@dataclass
class HashesLogContextValue(ContextValue):
    value: list[list[int]]

    def to_grpc_context_value(self) -> GrpcContextValue:
        log = GrpcHashesEventLog()
        for trace in self.value:
            grpc_trace = GrpcHashesLogTrace()
            for event in trace:
                grpc_trace.events.append(event)

            log.traces.append(grpc_trace)

        return GrpcContextValue(hashes_log=GrpcHashesEventLogContextValue(log=log))


@dataclass
class NamesLogContextValue(ContextValue):
    value: list[list[str]]

    def to_grpc_context_value(self) -> GrpcContextValue:
        log = GrpcNamesEventLog()
        for trace in self.value:
            grpc_trace = GrpcNamesTrace()
            for event in trace:
                grpc_trace.events.append(event)

            log.traces.append(grpc_trace)

        return GrpcContextValue(names_log=GrpcNamesEventLogContextValue(log=log))


@dataclass
class EnumContextValue(ContextValue):
    enum_name: str
    value: str

    def to_grpc_context_value(self) -> GrpcContextValue:
        return GrpcContextValue(enum=GrpcEnum(enumType=self.enum_name, value=self.value))


@dataclass
class StringsContextValue(ContextValue):
    strings: list[str]

    def to_grpc_context_value(self) -> GrpcContextValue:
        strings = GrpcStrings()
        strings.strings.extend(self.strings)
        return GrpcContextValue(strings=strings)


def from_grpc_names_log(grpc_names_log: GrpcNamesEventLog) -> list[list[str]]:
    result = []
    for grpc_trace in grpc_names_log.traces:
        trace = []
        for event in grpc_trace.events:
            trace.append(event)

        result.append(trace)

    return result


@dataclass
class EventLogInfo(ContextValue):
    events_count: int
    event_classes_count: int
    traces_count: int

    def to_grpc_context_value(self) -> GrpcContextValue:
        log_info = GrpcEventLogInfo(events_count=self.events_count,
                                    traces_count=self.traces_count,
                                    event_classes_count=self.event_classes_count)

        return GrpcContextValue(event_log_info=log_info)


@dataclass
class ProxyColorRectangle:
    color_index: int
    start_index: int
    length: int


@dataclass
class ProxyColorMapping:
    name: str
    color: Color


@dataclass
class ProxyColorsTrace:
    event_colors: list[ProxyColorRectangle]
    constant_width: bool


@dataclass
class ProxyColorsEventLog(ContextValue):
    mapping: list[ProxyColorMapping]
    traces: list[ProxyColorsTrace]

    def to_grpc_context_value(self) -> GrpcContextValue:
        pass

@dataclass
class BytesContextValue(ContextValue):
    bytes: bytes

    def to_grpc_context_value(self) -> GrpcContextValue:
        return GrpcContextValue(bytes=GrpcBytes(bytes=self.bytes))


def from_grpc_colors_log_proxy(grpc_colors_log: GrpcColorsEventLog) -> ProxyColorsEventLog:
    mapping = from_grpc_color_mapping(list(grpc_colors_log.mapping))

    traces = []
    for grpc_trace in grpc_colors_log.traces:
        trace = []
        for colored_rectangle in grpc_trace.event_colors:
            trace.append(from_grpc_colored_rectangle_proxy(colored_rectangle))

        traces.append(ProxyColorsTrace(event_colors=trace, constant_width=grpc_trace.constant_width))

    return ProxyColorsEventLog(mapping, traces)


def from_grpc_colored_rectangle_proxy(grpc_color: GrpcColoredRectangle) -> ProxyColorRectangle:
    return ProxyColorRectangle(grpc_color.color_index, grpc_color.start_index, grpc_color.length)


def from_grpc_color_mapping(grpc_mapping: list[GrpcColorsEventLogMapping]) -> list[ProxyColorMapping]:
    mapping = []
    for pair in grpc_mapping:
        mapping.append(ProxyColorMapping(pair.name, from_grpc_color(pair.color)))

    return mapping


def from_grpc_colors_log(grpc_colors_log: GrpcColorsEventLog) -> list[list[ColoredRectangle]]:
    result = []
    mapping = list(grpc_colors_log.mapping)

    for grpc_trace in grpc_colors_log.traces:
        trace = []
        for colored_rectangle in grpc_trace.event_colors:
            trace.append(from_grpc_colored_rectangle(colored_rectangle, mapping))

        result.append(trace)

    return result


def from_grpc_colored_rectangle(grpc_color: GrpcColoredRectangle,
                                mapping: list[GrpcColorsEventLogMapping]) -> ColoredRectangle:
    name, color = mapping[grpc_color.color_index].name, from_grpc_color(mapping[grpc_color.color_index].color)
    return ColoredRectangle(color, grpc_color.start_index, grpc_color.length, name)


def from_grpc_color(grpc_color: GrpcColor):
    return Color(grpc_color.red, grpc_color.green, grpc_color.blue)


def from_grpc_event_log_info(grpc_event_log_info: GrpcEventLogInfo) -> EventLogInfo:
    return EventLogInfo(events_count=grpc_event_log_info.events_count,
                        traces_count=grpc_event_log_info.traces_count,
                        event_classes_count=grpc_event_log_info.event_classes_count)


def from_grpc_petri_net(grpc_petri_net: GrpcPetriNet) -> 'PetriNet':
    petri_net = PetriNet()
    for grpc_place in grpc_petri_net.places:
        place = from_grpc_petri_net_place(grpc_place)
        petri_net.places[place.id] = place

    for grpc_transition in grpc_petri_net.transitions:
        transition = from_grpc_transition(grpc_transition)
        petri_net.transitions[transition.id] = transition

    petri_net.initial_marking = try_from_grpc_marking(grpc_petri_net.initial_marking)
    petri_net.final_marking = try_from_grpc_marking(grpc_petri_net.final_marking)

    return petri_net


def from_grpc_petri_net_place(grpc_petri_net_place: GrpcPetriNetPlace) -> 'Place':
    return Place(grpc_petri_net_place.id, grpc_petri_net_place.name)


def from_grpc_transition(grpc_petri_net_transition: GrpcPetriNetTransition) -> 'Transition':
    transition = Transition(grpc_petri_net_transition.id)
    for grpc_incoming_arc in grpc_petri_net_transition.incomingArcs:
        transition.incoming_arcs.append(from_grpc_arc(grpc_incoming_arc))

    for grpc_outgoing_arc in grpc_petri_net_transition.outgoingArcs:
        transition.outgoing_arcs.append(from_grpc_arc(grpc_outgoing_arc))

    transition.data = grpc_petri_net_transition.data
    return transition


def from_grpc_arc(grpc_arc: GrpcPetriNetArc) -> 'Arc':
    return Arc(grpc_arc.id, grpc_arc.placeId, grpc_arc.tokens_count)


def try_from_grpc_marking(grpc_marking: Optional[GrpcPetriNetMarking]) -> Optional[Marking]:
    if grpc_marking is None:
        return None

    return Marking(list(map(from_grpc_single_marking, grpc_marking.markings)))


def from_grpc_single_marking(grpc_marking: GrpcPetriNetSinglePlaceMarking) -> SinglePlaceMarking:
    return SinglePlaceMarking(grpc_marking.placeId, grpc_marking.tokensCount)


def from_grpc_count_annotation(grpc_annotation: GrpcCountAnnotation) -> dict[int, str]:
    map = dict()
    for annotation in grpc_annotation.annotations:
        map[annotation.entityId] = str(annotation.count)

    return map


def from_grpc_frequency_annotation(grpc_annotation: GrpcFrequenciesAnnotation) -> dict[int, str]:
    map = dict()
    for annotation in grpc_annotation.annotations:
        map[annotation.entityId] = f'{annotation.frequency:.3f}'

    return map


def from_grpc_ficus_dataset(grpc_dataset: GrpcDataset) -> pd.DataFrame:
    data = []
    for row in grpc_dataset.matrix.rows:
        row_vec = []
        for value in row.values:
            row_vec.append(value)

        data.append(row_vec)

    columns = []
    for column_name in grpc_dataset.columnsNames:
        columns.append(column_name)

    index = []
    for row_name in grpc_dataset.rowNames:
        index.append(row_name)

    return pd.DataFrame(data, columns=columns, index=index)


def from_grpc_labeled_dataset(grpc_dataset: GrpcLabeledDataset):
    df = from_grpc_ficus_dataset(grpc_dataset.dataset)
    labels = []
    for label in grpc_dataset.labels:
        labels.append(label)

    df[const_cluster_labels] = labels
    return df


def from_grpc_graph(grpc_graph: GrpcGraph) -> Graph:
    graph = Graph()
    for node in grpc_graph.nodes:
        graph.nodes.append(GraphNode(id=node.id, data=node.data))

    for edge in grpc_graph.edges:
        graph.edges.append(GraphEdge(from_node=edge.from_node, to_node=edge.to_node, data=edge.data))

    return graph


def read_file_bytes(log_path: str) -> bytes:
    with open(log_path, 'rb') as fin:
        return fin.read()
