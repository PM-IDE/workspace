from google.protobuf import timestamp_pb2 as _timestamp_pb2
from google.protobuf import empty_pb2 as _empty_pb2
import util_pb2 as _util_pb2
from google.protobuf.internal import containers as _containers
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from typing import ClassVar as _ClassVar, Iterable as _Iterable, Mapping as _Mapping, Optional as _Optional, Union as _Union

DESCRIPTOR: _descriptor.FileDescriptor

class GrpcSimpleEventLog(_message.Message):
    __slots__ = ["traces"]
    TRACES_FIELD_NUMBER: _ClassVar[int]
    traces: _containers.RepeatedCompositeFieldContainer[GrpcSimpleTrace]
    def __init__(self, traces: _Optional[_Iterable[_Union[GrpcSimpleTrace, _Mapping]]] = ...) -> None: ...

class GrpcSimpleTrace(_message.Message):
    __slots__ = ["events"]
    EVENTS_FIELD_NUMBER: _ClassVar[int]
    events: _containers.RepeatedCompositeFieldContainer[GrpcEvent]
    def __init__(self, events: _Optional[_Iterable[_Union[GrpcEvent, _Mapping]]] = ...) -> None: ...

class GrpcEvent(_message.Message):
    __slots__ = ["name", "stamp", "attributes"]
    NAME_FIELD_NUMBER: _ClassVar[int]
    STAMP_FIELD_NUMBER: _ClassVar[int]
    ATTRIBUTES_FIELD_NUMBER: _ClassVar[int]
    name: str
    stamp: GrpcEventStamp
    attributes: _containers.RepeatedCompositeFieldContainer[GrpcEventAttribute]
    def __init__(self, name: _Optional[str] = ..., stamp: _Optional[_Union[GrpcEventStamp, _Mapping]] = ..., attributes: _Optional[_Iterable[_Union[GrpcEventAttribute, _Mapping]]] = ...) -> None: ...

class GrpcEventStamp(_message.Message):
    __slots__ = ["date", "order"]
    DATE_FIELD_NUMBER: _ClassVar[int]
    ORDER_FIELD_NUMBER: _ClassVar[int]
    date: _timestamp_pb2.Timestamp
    order: int
    def __init__(self, date: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ..., order: _Optional[int] = ...) -> None: ...

class GrpcEventAttribute(_message.Message):
    __slots__ = ["key", "int", "string", "bool", "double", "guid", "null", "stamp", "uint"]
    KEY_FIELD_NUMBER: _ClassVar[int]
    INT_FIELD_NUMBER: _ClassVar[int]
    STRING_FIELD_NUMBER: _ClassVar[int]
    BOOL_FIELD_NUMBER: _ClassVar[int]
    DOUBLE_FIELD_NUMBER: _ClassVar[int]
    GUID_FIELD_NUMBER: _ClassVar[int]
    NULL_FIELD_NUMBER: _ClassVar[int]
    STAMP_FIELD_NUMBER: _ClassVar[int]
    UINT_FIELD_NUMBER: _ClassVar[int]
    key: str
    int: int
    string: str
    bool: bool
    double: float
    guid: _util_pb2.GrpcGuid
    null: _empty_pb2.Empty
    stamp: _timestamp_pb2.Timestamp
    uint: int
    def __init__(self, key: _Optional[str] = ..., int: _Optional[int] = ..., string: _Optional[str] = ..., bool: bool = ..., double: _Optional[float] = ..., guid: _Optional[_Union[_util_pb2.GrpcGuid, _Mapping]] = ..., null: _Optional[_Union[_empty_pb2.Empty, _Mapping]] = ..., stamp: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ..., uint: _Optional[int] = ...) -> None: ...

class GrpcHashesEventLog(_message.Message):
    __slots__ = ["traces"]
    TRACES_FIELD_NUMBER: _ClassVar[int]
    traces: _containers.RepeatedCompositeFieldContainer[GrpcHashesLogTrace]
    def __init__(self, traces: _Optional[_Iterable[_Union[GrpcHashesLogTrace, _Mapping]]] = ...) -> None: ...

class GrpcHashesLogTrace(_message.Message):
    __slots__ = ["events"]
    EVENTS_FIELD_NUMBER: _ClassVar[int]
    events: _containers.RepeatedScalarFieldContainer[int]
    def __init__(self, events: _Optional[_Iterable[int]] = ...) -> None: ...

class GrpcNamesEventLog(_message.Message):
    __slots__ = ["traces"]
    TRACES_FIELD_NUMBER: _ClassVar[int]
    traces: _containers.RepeatedCompositeFieldContainer[GrpcNamesTrace]
    def __init__(self, traces: _Optional[_Iterable[_Union[GrpcNamesTrace, _Mapping]]] = ...) -> None: ...

class GrpcNamesTrace(_message.Message):
    __slots__ = ["events"]
    EVENTS_FIELD_NUMBER: _ClassVar[int]
    events: _containers.RepeatedScalarFieldContainer[str]
    def __init__(self, events: _Optional[_Iterable[str]] = ...) -> None: ...

class GrpcPetriNet(_message.Message):
    __slots__ = ["places", "transitions", "initial_marking", "final_marking"]
    PLACES_FIELD_NUMBER: _ClassVar[int]
    TRANSITIONS_FIELD_NUMBER: _ClassVar[int]
    INITIAL_MARKING_FIELD_NUMBER: _ClassVar[int]
    FINAL_MARKING_FIELD_NUMBER: _ClassVar[int]
    places: _containers.RepeatedCompositeFieldContainer[GrpcPetriNetPlace]
    transitions: _containers.RepeatedCompositeFieldContainer[GrpcPetriNetTransition]
    initial_marking: GrpcPetriNetMarking
    final_marking: GrpcPetriNetMarking
    def __init__(self, places: _Optional[_Iterable[_Union[GrpcPetriNetPlace, _Mapping]]] = ..., transitions: _Optional[_Iterable[_Union[GrpcPetriNetTransition, _Mapping]]] = ..., initial_marking: _Optional[_Union[GrpcPetriNetMarking, _Mapping]] = ..., final_marking: _Optional[_Union[GrpcPetriNetMarking, _Mapping]] = ...) -> None: ...

class GrpcPetriNetPlace(_message.Message):
    __slots__ = ["id", "name"]
    ID_FIELD_NUMBER: _ClassVar[int]
    NAME_FIELD_NUMBER: _ClassVar[int]
    id: int
    name: str
    def __init__(self, id: _Optional[int] = ..., name: _Optional[str] = ...) -> None: ...

class GrpcPetriNetTransition(_message.Message):
    __slots__ = ["id", "incomingArcs", "outgoingArcs", "data"]
    ID_FIELD_NUMBER: _ClassVar[int]
    INCOMINGARCS_FIELD_NUMBER: _ClassVar[int]
    OUTGOINGARCS_FIELD_NUMBER: _ClassVar[int]
    DATA_FIELD_NUMBER: _ClassVar[int]
    id: int
    incomingArcs: _containers.RepeatedCompositeFieldContainer[GrpcPetriNetArc]
    outgoingArcs: _containers.RepeatedCompositeFieldContainer[GrpcPetriNetArc]
    data: str
    def __init__(self, id: _Optional[int] = ..., incomingArcs: _Optional[_Iterable[_Union[GrpcPetriNetArc, _Mapping]]] = ..., outgoingArcs: _Optional[_Iterable[_Union[GrpcPetriNetArc, _Mapping]]] = ..., data: _Optional[str] = ...) -> None: ...

class GrpcPetriNetArc(_message.Message):
    __slots__ = ["id", "placeId", "tokens_count"]
    ID_FIELD_NUMBER: _ClassVar[int]
    PLACEID_FIELD_NUMBER: _ClassVar[int]
    TOKENS_COUNT_FIELD_NUMBER: _ClassVar[int]
    id: int
    placeId: int
    tokens_count: int
    def __init__(self, id: _Optional[int] = ..., placeId: _Optional[int] = ..., tokens_count: _Optional[int] = ...) -> None: ...

class GrpcPetriNetMarking(_message.Message):
    __slots__ = ["markings"]
    MARKINGS_FIELD_NUMBER: _ClassVar[int]
    markings: _containers.RepeatedCompositeFieldContainer[GrpcPetriNetSinglePlaceMarking]
    def __init__(self, markings: _Optional[_Iterable[_Union[GrpcPetriNetSinglePlaceMarking, _Mapping]]] = ...) -> None: ...

class GrpcPetriNetSinglePlaceMarking(_message.Message):
    __slots__ = ["placeId", "tokensCount"]
    PLACEID_FIELD_NUMBER: _ClassVar[int]
    TOKENSCOUNT_FIELD_NUMBER: _ClassVar[int]
    placeId: int
    tokensCount: int
    def __init__(self, placeId: _Optional[int] = ..., tokensCount: _Optional[int] = ...) -> None: ...

class GrpcAnnotation(_message.Message):
    __slots__ = ["countAnnotation", "frequencyAnnotation", "timeAnnotation"]
    COUNTANNOTATION_FIELD_NUMBER: _ClassVar[int]
    FREQUENCYANNOTATION_FIELD_NUMBER: _ClassVar[int]
    TIMEANNOTATION_FIELD_NUMBER: _ClassVar[int]
    countAnnotation: GrpcCountAnnotation
    frequencyAnnotation: GrpcFrequenciesAnnotation
    timeAnnotation: GrpcTimePerformanceAnnotation
    def __init__(self, countAnnotation: _Optional[_Union[GrpcCountAnnotation, _Mapping]] = ..., frequencyAnnotation: _Optional[_Union[GrpcFrequenciesAnnotation, _Mapping]] = ..., timeAnnotation: _Optional[_Union[GrpcTimePerformanceAnnotation, _Mapping]] = ...) -> None: ...

class GrpcCountAnnotation(_message.Message):
    __slots__ = ["annotations"]
    ANNOTATIONS_FIELD_NUMBER: _ClassVar[int]
    annotations: _containers.RepeatedCompositeFieldContainer[GrpcEntityCountAnnotation]
    def __init__(self, annotations: _Optional[_Iterable[_Union[GrpcEntityCountAnnotation, _Mapping]]] = ...) -> None: ...

class GrpcEntityCountAnnotation(_message.Message):
    __slots__ = ["entityId", "count"]
    ENTITYID_FIELD_NUMBER: _ClassVar[int]
    COUNT_FIELD_NUMBER: _ClassVar[int]
    entityId: int
    count: int
    def __init__(self, entityId: _Optional[int] = ..., count: _Optional[int] = ...) -> None: ...

class GrpcFrequenciesAnnotation(_message.Message):
    __slots__ = ["annotations"]
    ANNOTATIONS_FIELD_NUMBER: _ClassVar[int]
    annotations: _containers.RepeatedCompositeFieldContainer[GrpcEntityFrequencyAnnotation]
    def __init__(self, annotations: _Optional[_Iterable[_Union[GrpcEntityFrequencyAnnotation, _Mapping]]] = ...) -> None: ...

class GrpcEntityFrequencyAnnotation(_message.Message):
    __slots__ = ["entityId", "frequency"]
    ENTITYID_FIELD_NUMBER: _ClassVar[int]
    FREQUENCY_FIELD_NUMBER: _ClassVar[int]
    entityId: int
    frequency: float
    def __init__(self, entityId: _Optional[int] = ..., frequency: _Optional[float] = ...) -> None: ...

class GrpcTimePerformanceAnnotation(_message.Message):
    __slots__ = ["annotations"]
    ANNOTATIONS_FIELD_NUMBER: _ClassVar[int]
    annotations: _containers.RepeatedCompositeFieldContainer[GrpcEntityTimeAnnotation]
    def __init__(self, annotations: _Optional[_Iterable[_Union[GrpcEntityTimeAnnotation, _Mapping]]] = ...) -> None: ...

class GrpcEntityTimeAnnotation(_message.Message):
    __slots__ = ["entityId", "interval"]
    ENTITYID_FIELD_NUMBER: _ClassVar[int]
    INTERVAL_FIELD_NUMBER: _ClassVar[int]
    entityId: int
    interval: _util_pb2.GrpcTimeSpan
    def __init__(self, entityId: _Optional[int] = ..., interval: _Optional[_Union[_util_pb2.GrpcTimeSpan, _Mapping]] = ...) -> None: ...

class GrpcMatrix(_message.Message):
    __slots__ = ["rows"]
    ROWS_FIELD_NUMBER: _ClassVar[int]
    rows: _containers.RepeatedCompositeFieldContainer[GrpcMatrixRow]
    def __init__(self, rows: _Optional[_Iterable[_Union[GrpcMatrixRow, _Mapping]]] = ...) -> None: ...

class GrpcMatrixRow(_message.Message):
    __slots__ = ["values"]
    VALUES_FIELD_NUMBER: _ClassVar[int]
    values: _containers.RepeatedScalarFieldContainer[float]
    def __init__(self, values: _Optional[_Iterable[float]] = ...) -> None: ...

class GrpcDataset(_message.Message):
    __slots__ = ["matrix", "columnsNames", "rowNames"]
    MATRIX_FIELD_NUMBER: _ClassVar[int]
    COLUMNSNAMES_FIELD_NUMBER: _ClassVar[int]
    ROWNAMES_FIELD_NUMBER: _ClassVar[int]
    matrix: GrpcMatrix
    columnsNames: _containers.RepeatedScalarFieldContainer[str]
    rowNames: _containers.RepeatedScalarFieldContainer[str]
    def __init__(self, matrix: _Optional[_Union[GrpcMatrix, _Mapping]] = ..., columnsNames: _Optional[_Iterable[str]] = ..., rowNames: _Optional[_Iterable[str]] = ...) -> None: ...

class GrpcLabeledDataset(_message.Message):
    __slots__ = ["dataset", "labels", "labelsColors"]
    DATASET_FIELD_NUMBER: _ClassVar[int]
    LABELS_FIELD_NUMBER: _ClassVar[int]
    LABELSCOLORS_FIELD_NUMBER: _ClassVar[int]
    dataset: GrpcDataset
    labels: _containers.RepeatedScalarFieldContainer[int]
    labelsColors: _containers.RepeatedCompositeFieldContainer[_util_pb2.GrpcColor]
    def __init__(self, dataset: _Optional[_Union[GrpcDataset, _Mapping]] = ..., labels: _Optional[_Iterable[int]] = ..., labelsColors: _Optional[_Iterable[_Union[_util_pb2.GrpcColor, _Mapping]]] = ...) -> None: ...
