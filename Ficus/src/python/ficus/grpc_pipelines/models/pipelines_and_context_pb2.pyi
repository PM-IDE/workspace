import pm_models_pb2 as _pm_models_pb2
import util_pb2 as _util_pb2
from google.protobuf.internal import containers as _containers
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from typing import ClassVar as _ClassVar, Iterable as _Iterable, Mapping as _Mapping, Optional as _Optional, Union as _Union

DESCRIPTOR: _descriptor.FileDescriptor

class GrpcContextKey(_message.Message):
    __slots__ = ["name"]
    NAME_FIELD_NUMBER: _ClassVar[int]
    name: str
    def __init__(self, name: _Optional[str] = ...) -> None: ...

class GrpcContextValueWithKeyName(_message.Message):
    __slots__ = ["key_name", "value"]
    KEY_NAME_FIELD_NUMBER: _ClassVar[int]
    VALUE_FIELD_NUMBER: _ClassVar[int]
    key_name: str
    value: GrpcContextValue
    def __init__(self, key_name: _Optional[str] = ..., value: _Optional[_Union[GrpcContextValue, _Mapping]] = ...) -> None: ...

class GrpcContextValue(_message.Message):
    __slots__ = ["string", "hashes_log", "names_log", "uint32", "traces_sub_arrays", "trace_index_sub_arrays", "bool", "xes_event_log", "colors_log", "enum", "event_log_info", "strings", "pipeline", "petriNet", "graph", "float", "annotation", "dataset", "labeled_dataset", "bytes", "logTimelineDiagram"]
    STRING_FIELD_NUMBER: _ClassVar[int]
    HASHES_LOG_FIELD_NUMBER: _ClassVar[int]
    NAMES_LOG_FIELD_NUMBER: _ClassVar[int]
    UINT32_FIELD_NUMBER: _ClassVar[int]
    TRACES_SUB_ARRAYS_FIELD_NUMBER: _ClassVar[int]
    TRACE_INDEX_SUB_ARRAYS_FIELD_NUMBER: _ClassVar[int]
    BOOL_FIELD_NUMBER: _ClassVar[int]
    XES_EVENT_LOG_FIELD_NUMBER: _ClassVar[int]
    COLORS_LOG_FIELD_NUMBER: _ClassVar[int]
    ENUM_FIELD_NUMBER: _ClassVar[int]
    EVENT_LOG_INFO_FIELD_NUMBER: _ClassVar[int]
    STRINGS_FIELD_NUMBER: _ClassVar[int]
    PIPELINE_FIELD_NUMBER: _ClassVar[int]
    PETRINET_FIELD_NUMBER: _ClassVar[int]
    GRAPH_FIELD_NUMBER: _ClassVar[int]
    FLOAT_FIELD_NUMBER: _ClassVar[int]
    ANNOTATION_FIELD_NUMBER: _ClassVar[int]
    DATASET_FIELD_NUMBER: _ClassVar[int]
    LABELED_DATASET_FIELD_NUMBER: _ClassVar[int]
    BYTES_FIELD_NUMBER: _ClassVar[int]
    LOGTIMELINEDIAGRAM_FIELD_NUMBER: _ClassVar[int]
    string: str
    hashes_log: GrpcHashesEventLogContextValue
    names_log: GrpcNamesEventLogContextValue
    uint32: int
    traces_sub_arrays: GrpcEventLogTraceSubArraysContextValue
    trace_index_sub_arrays: GrpcSubArraysWithTraceIndexContextValue
    bool: bool
    xes_event_log: GrpcNamesEventLogContextValue
    colors_log: GrpcColorsEventLog
    enum: GrpcEnum
    event_log_info: GrpcEventLogInfo
    strings: GrpcStrings
    pipeline: GrpcPipeline
    petriNet: _pm_models_pb2.GrpcPetriNet
    graph: GrpcGraph
    float: float
    annotation: _pm_models_pb2.GrpcAnnotation
    dataset: _pm_models_pb2.GrpcDataset
    labeled_dataset: _pm_models_pb2.GrpcLabeledDataset
    bytes: GrpcBytes
    logTimelineDiagram: GrpcLogTimelineDiagram
    def __init__(self, string: _Optional[str] = ..., hashes_log: _Optional[_Union[GrpcHashesEventLogContextValue, _Mapping]] = ..., names_log: _Optional[_Union[GrpcNamesEventLogContextValue, _Mapping]] = ..., uint32: _Optional[int] = ..., traces_sub_arrays: _Optional[_Union[GrpcEventLogTraceSubArraysContextValue, _Mapping]] = ..., trace_index_sub_arrays: _Optional[_Union[GrpcSubArraysWithTraceIndexContextValue, _Mapping]] = ..., bool: bool = ..., xes_event_log: _Optional[_Union[GrpcNamesEventLogContextValue, _Mapping]] = ..., colors_log: _Optional[_Union[GrpcColorsEventLog, _Mapping]] = ..., enum: _Optional[_Union[GrpcEnum, _Mapping]] = ..., event_log_info: _Optional[_Union[GrpcEventLogInfo, _Mapping]] = ..., strings: _Optional[_Union[GrpcStrings, _Mapping]] = ..., pipeline: _Optional[_Union[GrpcPipeline, _Mapping]] = ..., petriNet: _Optional[_Union[_pm_models_pb2.GrpcPetriNet, _Mapping]] = ..., graph: _Optional[_Union[GrpcGraph, _Mapping]] = ..., float: _Optional[float] = ..., annotation: _Optional[_Union[_pm_models_pb2.GrpcAnnotation, _Mapping]] = ..., dataset: _Optional[_Union[_pm_models_pb2.GrpcDataset, _Mapping]] = ..., labeled_dataset: _Optional[_Union[_pm_models_pb2.GrpcLabeledDataset, _Mapping]] = ..., bytes: _Optional[_Union[GrpcBytes, _Mapping]] = ..., logTimelineDiagram: _Optional[_Union[GrpcLogTimelineDiagram, _Mapping]] = ...) -> None: ...

class GrpcContextKeyValue(_message.Message):
    __slots__ = ["key", "value"]
    KEY_FIELD_NUMBER: _ClassVar[int]
    VALUE_FIELD_NUMBER: _ClassVar[int]
    key: GrpcContextKey
    value: GrpcContextValue
    def __init__(self, key: _Optional[_Union[GrpcContextKey, _Mapping]] = ..., value: _Optional[_Union[GrpcContextValue, _Mapping]] = ...) -> None: ...

class GrpcHashesEventLogContextValue(_message.Message):
    __slots__ = ["log"]
    LOG_FIELD_NUMBER: _ClassVar[int]
    log: _pm_models_pb2.GrpcHashesEventLog
    def __init__(self, log: _Optional[_Union[_pm_models_pb2.GrpcHashesEventLog, _Mapping]] = ...) -> None: ...

class GrpcNamesEventLogContextValue(_message.Message):
    __slots__ = ["log"]
    LOG_FIELD_NUMBER: _ClassVar[int]
    log: _pm_models_pb2.GrpcNamesEventLog
    def __init__(self, log: _Optional[_Union[_pm_models_pb2.GrpcNamesEventLog, _Mapping]] = ...) -> None: ...

class GrpcEventLogTraceSubArraysContextValue(_message.Message):
    __slots__ = ["traces_sub_arrays"]
    TRACES_SUB_ARRAYS_FIELD_NUMBER: _ClassVar[int]
    traces_sub_arrays: _containers.RepeatedCompositeFieldContainer[GrpcTraceSubArrays]
    def __init__(self, traces_sub_arrays: _Optional[_Iterable[_Union[GrpcTraceSubArrays, _Mapping]]] = ...) -> None: ...

class GrpcTraceSubArray(_message.Message):
    __slots__ = ["start", "end"]
    START_FIELD_NUMBER: _ClassVar[int]
    END_FIELD_NUMBER: _ClassVar[int]
    start: int
    end: int
    def __init__(self, start: _Optional[int] = ..., end: _Optional[int] = ...) -> None: ...

class GrpcTraceSubArrays(_message.Message):
    __slots__ = ["sub_arrays"]
    SUB_ARRAYS_FIELD_NUMBER: _ClassVar[int]
    sub_arrays: _containers.RepeatedCompositeFieldContainer[GrpcTraceSubArray]
    def __init__(self, sub_arrays: _Optional[_Iterable[_Union[GrpcTraceSubArray, _Mapping]]] = ...) -> None: ...

class GrpcSubArrayWithTraceIndex(_message.Message):
    __slots__ = ["sub_array", "trace_index"]
    SUB_ARRAY_FIELD_NUMBER: _ClassVar[int]
    TRACE_INDEX_FIELD_NUMBER: _ClassVar[int]
    sub_array: GrpcTraceSubArray
    trace_index: int
    def __init__(self, sub_array: _Optional[_Union[GrpcTraceSubArray, _Mapping]] = ..., trace_index: _Optional[int] = ...) -> None: ...

class GrpcSubArraysWithTraceIndexContextValue(_message.Message):
    __slots__ = ["sub_arrays"]
    SUB_ARRAYS_FIELD_NUMBER: _ClassVar[int]
    sub_arrays: _containers.RepeatedCompositeFieldContainer[GrpcSubArrayWithTraceIndex]
    def __init__(self, sub_arrays: _Optional[_Iterable[_Union[GrpcSubArrayWithTraceIndex, _Mapping]]] = ...) -> None: ...

class GrpcColorsEventLog(_message.Message):
    __slots__ = ["mapping", "traces", "adjustments"]
    MAPPING_FIELD_NUMBER: _ClassVar[int]
    TRACES_FIELD_NUMBER: _ClassVar[int]
    ADJUSTMENTS_FIELD_NUMBER: _ClassVar[int]
    mapping: _containers.RepeatedCompositeFieldContainer[GrpcColorsEventLogMapping]
    traces: _containers.RepeatedCompositeFieldContainer[GrpcColorsTrace]
    adjustments: _containers.RepeatedCompositeFieldContainer[GrpcColorsLogAdjustment]
    def __init__(self, mapping: _Optional[_Iterable[_Union[GrpcColorsEventLogMapping, _Mapping]]] = ..., traces: _Optional[_Iterable[_Union[GrpcColorsTrace, _Mapping]]] = ..., adjustments: _Optional[_Iterable[_Union[GrpcColorsLogAdjustment, _Mapping]]] = ...) -> None: ...

class GrpcColorsLogAdjustment(_message.Message):
    __slots__ = ["rectangle_adjustment", "axis_after_trace"]
    RECTANGLE_ADJUSTMENT_FIELD_NUMBER: _ClassVar[int]
    AXIS_AFTER_TRACE_FIELD_NUMBER: _ClassVar[int]
    rectangle_adjustment: GrpcColorsLogRectangleAdjustment
    axis_after_trace: GrpcColorsLogXAxisAfterTraceAdjustment
    def __init__(self, rectangle_adjustment: _Optional[_Union[GrpcColorsLogRectangleAdjustment, _Mapping]] = ..., axis_after_trace: _Optional[_Union[GrpcColorsLogXAxisAfterTraceAdjustment, _Mapping]] = ...) -> None: ...

class GrpcColorsLogRectangleAdjustment(_message.Message):
    __slots__ = ["up_left_point", "down_right_point"]
    UP_LEFT_POINT_FIELD_NUMBER: _ClassVar[int]
    DOWN_RIGHT_POINT_FIELD_NUMBER: _ClassVar[int]
    up_left_point: GrpcLogPoint
    down_right_point: GrpcLogPoint
    def __init__(self, up_left_point: _Optional[_Union[GrpcLogPoint, _Mapping]] = ..., down_right_point: _Optional[_Union[GrpcLogPoint, _Mapping]] = ...) -> None: ...

class GrpcLogPoint(_message.Message):
    __slots__ = ["traces_index", "event_index"]
    TRACES_INDEX_FIELD_NUMBER: _ClassVar[int]
    EVENT_INDEX_FIELD_NUMBER: _ClassVar[int]
    traces_index: int
    event_index: int
    def __init__(self, traces_index: _Optional[int] = ..., event_index: _Optional[int] = ...) -> None: ...

class GrpcColorsLogXAxisAfterTraceAdjustment(_message.Message):
    __slots__ = ["trace_index"]
    TRACE_INDEX_FIELD_NUMBER: _ClassVar[int]
    trace_index: int
    def __init__(self, trace_index: _Optional[int] = ...) -> None: ...

class GrpcColorsEventLogMapping(_message.Message):
    __slots__ = ["name", "color"]
    NAME_FIELD_NUMBER: _ClassVar[int]
    COLOR_FIELD_NUMBER: _ClassVar[int]
    name: str
    color: _util_pb2.GrpcColor
    def __init__(self, name: _Optional[str] = ..., color: _Optional[_Union[_util_pb2.GrpcColor, _Mapping]] = ...) -> None: ...

class GrpcColorsTrace(_message.Message):
    __slots__ = ["event_colors", "constant_width"]
    EVENT_COLORS_FIELD_NUMBER: _ClassVar[int]
    CONSTANT_WIDTH_FIELD_NUMBER: _ClassVar[int]
    event_colors: _containers.RepeatedCompositeFieldContainer[GrpcColoredRectangle]
    constant_width: bool
    def __init__(self, event_colors: _Optional[_Iterable[_Union[GrpcColoredRectangle, _Mapping]]] = ..., constant_width: bool = ...) -> None: ...

class GrpcColoredRectangle(_message.Message):
    __slots__ = ["color_index", "start_x", "length"]
    COLOR_INDEX_FIELD_NUMBER: _ClassVar[int]
    START_X_FIELD_NUMBER: _ClassVar[int]
    LENGTH_FIELD_NUMBER: _ClassVar[int]
    color_index: int
    start_x: float
    length: float
    def __init__(self, color_index: _Optional[int] = ..., start_x: _Optional[float] = ..., length: _Optional[float] = ...) -> None: ...

class GrpcEnum(_message.Message):
    __slots__ = ["enumType", "value"]
    ENUMTYPE_FIELD_NUMBER: _ClassVar[int]
    VALUE_FIELD_NUMBER: _ClassVar[int]
    enumType: str
    value: str
    def __init__(self, enumType: _Optional[str] = ..., value: _Optional[str] = ...) -> None: ...

class GrpcEventLogInfo(_message.Message):
    __slots__ = ["events_count", "traces_count", "event_classes_count"]
    EVENTS_COUNT_FIELD_NUMBER: _ClassVar[int]
    TRACES_COUNT_FIELD_NUMBER: _ClassVar[int]
    EVENT_CLASSES_COUNT_FIELD_NUMBER: _ClassVar[int]
    events_count: int
    traces_count: int
    event_classes_count: int
    def __init__(self, events_count: _Optional[int] = ..., traces_count: _Optional[int] = ..., event_classes_count: _Optional[int] = ...) -> None: ...

class GrpcStrings(_message.Message):
    __slots__ = ["strings"]
    STRINGS_FIELD_NUMBER: _ClassVar[int]
    strings: _containers.RepeatedScalarFieldContainer[str]
    def __init__(self, strings: _Optional[_Iterable[str]] = ...) -> None: ...

class GrpcPipeline(_message.Message):
    __slots__ = ["parts"]
    PARTS_FIELD_NUMBER: _ClassVar[int]
    parts: _containers.RepeatedCompositeFieldContainer[GrpcPipelinePartBase]
    def __init__(self, parts: _Optional[_Iterable[_Union[GrpcPipelinePartBase, _Mapping]]] = ...) -> None: ...

class GrpcPipelinePartBase(_message.Message):
    __slots__ = ["defaultPart", "parallelPart", "simpleContextRequestPart", "complexContextRequestPart"]
    DEFAULTPART_FIELD_NUMBER: _ClassVar[int]
    PARALLELPART_FIELD_NUMBER: _ClassVar[int]
    SIMPLECONTEXTREQUESTPART_FIELD_NUMBER: _ClassVar[int]
    COMPLEXCONTEXTREQUESTPART_FIELD_NUMBER: _ClassVar[int]
    defaultPart: GrpcPipelinePart
    parallelPart: GrpcParallelPipelinePart
    simpleContextRequestPart: GrpcSimpleContextRequestPipelinePart
    complexContextRequestPart: GrpcComplexContextRequestPipelinePart
    def __init__(self, defaultPart: _Optional[_Union[GrpcPipelinePart, _Mapping]] = ..., parallelPart: _Optional[_Union[GrpcParallelPipelinePart, _Mapping]] = ..., simpleContextRequestPart: _Optional[_Union[GrpcSimpleContextRequestPipelinePart, _Mapping]] = ..., complexContextRequestPart: _Optional[_Union[GrpcComplexContextRequestPipelinePart, _Mapping]] = ...) -> None: ...

class GrpcPipelinePart(_message.Message):
    __slots__ = ["name", "configuration"]
    NAME_FIELD_NUMBER: _ClassVar[int]
    CONFIGURATION_FIELD_NUMBER: _ClassVar[int]
    name: str
    configuration: GrpcPipelinePartConfiguration
    def __init__(self, name: _Optional[str] = ..., configuration: _Optional[_Union[GrpcPipelinePartConfiguration, _Mapping]] = ...) -> None: ...

class GrpcPipelinePartConfiguration(_message.Message):
    __slots__ = ["configurationParameters"]
    CONFIGURATIONPARAMETERS_FIELD_NUMBER: _ClassVar[int]
    configurationParameters: _containers.RepeatedCompositeFieldContainer[GrpcContextKeyValue]
    def __init__(self, configurationParameters: _Optional[_Iterable[_Union[GrpcContextKeyValue, _Mapping]]] = ...) -> None: ...

class GrpcParallelPipelinePart(_message.Message):
    __slots__ = ["pipelineParts"]
    PIPELINEPARTS_FIELD_NUMBER: _ClassVar[int]
    pipelineParts: _containers.RepeatedCompositeFieldContainer[GrpcPipelinePartBase]
    def __init__(self, pipelineParts: _Optional[_Iterable[_Union[GrpcPipelinePartBase, _Mapping]]] = ...) -> None: ...

class GrpcParallelPipelineParts(_message.Message):
    __slots__ = ["pipeline"]
    PIPELINE_FIELD_NUMBER: _ClassVar[int]
    pipeline: _containers.RepeatedCompositeFieldContainer[GrpcParallelPipelinePart]
    def __init__(self, pipeline: _Optional[_Iterable[_Union[GrpcParallelPipelinePart, _Mapping]]] = ...) -> None: ...

class GrpcSimpleContextRequestPipelinePart(_message.Message):
    __slots__ = ["key", "frontendPartUuid", "frontendPipelinePartName"]
    KEY_FIELD_NUMBER: _ClassVar[int]
    FRONTENDPARTUUID_FIELD_NUMBER: _ClassVar[int]
    FRONTENDPIPELINEPARTNAME_FIELD_NUMBER: _ClassVar[int]
    key: GrpcContextKey
    frontendPartUuid: _util_pb2.GrpcUuid
    frontendPipelinePartName: str
    def __init__(self, key: _Optional[_Union[GrpcContextKey, _Mapping]] = ..., frontendPartUuid: _Optional[_Union[_util_pb2.GrpcUuid, _Mapping]] = ..., frontendPipelinePartName: _Optional[str] = ...) -> None: ...

class GrpcComplexContextRequestPipelinePart(_message.Message):
    __slots__ = ["keys", "beforePipelinePart", "frontendPartUuid", "frontendPipelinePartName"]
    KEYS_FIELD_NUMBER: _ClassVar[int]
    BEFOREPIPELINEPART_FIELD_NUMBER: _ClassVar[int]
    FRONTENDPARTUUID_FIELD_NUMBER: _ClassVar[int]
    FRONTENDPIPELINEPARTNAME_FIELD_NUMBER: _ClassVar[int]
    keys: _containers.RepeatedCompositeFieldContainer[GrpcContextKey]
    beforePipelinePart: GrpcPipelinePart
    frontendPartUuid: _util_pb2.GrpcUuid
    frontendPipelinePartName: str
    def __init__(self, keys: _Optional[_Iterable[_Union[GrpcContextKey, _Mapping]]] = ..., beforePipelinePart: _Optional[_Union[GrpcPipelinePart, _Mapping]] = ..., frontendPartUuid: _Optional[_Union[_util_pb2.GrpcUuid, _Mapping]] = ..., frontendPipelinePartName: _Optional[str] = ...) -> None: ...

class GrpcGraph(_message.Message):
    __slots__ = ["nodes", "edges"]
    NODES_FIELD_NUMBER: _ClassVar[int]
    EDGES_FIELD_NUMBER: _ClassVar[int]
    nodes: _containers.RepeatedCompositeFieldContainer[GrpcGraphNode]
    edges: _containers.RepeatedCompositeFieldContainer[GrpcGraphEdge]
    def __init__(self, nodes: _Optional[_Iterable[_Union[GrpcGraphNode, _Mapping]]] = ..., edges: _Optional[_Iterable[_Union[GrpcGraphEdge, _Mapping]]] = ...) -> None: ...

class GrpcGraphNode(_message.Message):
    __slots__ = ["id", "data"]
    ID_FIELD_NUMBER: _ClassVar[int]
    DATA_FIELD_NUMBER: _ClassVar[int]
    id: int
    data: str
    def __init__(self, id: _Optional[int] = ..., data: _Optional[str] = ...) -> None: ...

class GrpcGraphEdge(_message.Message):
    __slots__ = ["id", "from_node", "to_node", "weight", "data"]
    ID_FIELD_NUMBER: _ClassVar[int]
    FROM_NODE_FIELD_NUMBER: _ClassVar[int]
    TO_NODE_FIELD_NUMBER: _ClassVar[int]
    WEIGHT_FIELD_NUMBER: _ClassVar[int]
    DATA_FIELD_NUMBER: _ClassVar[int]
    id: int
    from_node: int
    to_node: int
    weight: float
    data: str
    def __init__(self, id: _Optional[int] = ..., from_node: _Optional[int] = ..., to_node: _Optional[int] = ..., weight: _Optional[float] = ..., data: _Optional[str] = ...) -> None: ...

class GrpcBytes(_message.Message):
    __slots__ = ["bytes"]
    BYTES_FIELD_NUMBER: _ClassVar[int]
    bytes: bytes
    def __init__(self, bytes: _Optional[bytes] = ...) -> None: ...

class GrpcLogTimelineDiagram(_message.Message):
    __slots__ = ["traces"]
    TRACES_FIELD_NUMBER: _ClassVar[int]
    traces: _containers.RepeatedCompositeFieldContainer[GrpcTraceTimelineDiagram]
    def __init__(self, traces: _Optional[_Iterable[_Union[GrpcTraceTimelineDiagram, _Mapping]]] = ...) -> None: ...

class GrpcTimelineTraceEventsGroup(_message.Message):
    __slots__ = ["start_point", "end_point"]
    START_POINT_FIELD_NUMBER: _ClassVar[int]
    END_POINT_FIELD_NUMBER: _ClassVar[int]
    start_point: GrpcLogPoint
    end_point: GrpcLogPoint
    def __init__(self, start_point: _Optional[_Union[GrpcLogPoint, _Mapping]] = ..., end_point: _Optional[_Union[GrpcLogPoint, _Mapping]] = ...) -> None: ...

class GrpcTraceTimelineDiagram(_message.Message):
    __slots__ = ["threads", "events_groups"]
    THREADS_FIELD_NUMBER: _ClassVar[int]
    EVENTS_GROUPS_FIELD_NUMBER: _ClassVar[int]
    threads: _containers.RepeatedCompositeFieldContainer[GrpcThread]
    events_groups: _containers.RepeatedCompositeFieldContainer[GrpcTimelineTraceEventsGroup]
    def __init__(self, threads: _Optional[_Iterable[_Union[GrpcThread, _Mapping]]] = ..., events_groups: _Optional[_Iterable[_Union[GrpcTimelineTraceEventsGroup, _Mapping]]] = ...) -> None: ...

class GrpcThread(_message.Message):
    __slots__ = ["events"]
    EVENTS_FIELD_NUMBER: _ClassVar[int]
    events: _containers.RepeatedCompositeFieldContainer[GrpcThreadEvent]
    def __init__(self, events: _Optional[_Iterable[_Union[GrpcThreadEvent, _Mapping]]] = ...) -> None: ...

class GrpcThreadEvent(_message.Message):
    __slots__ = ["name", "stamp"]
    NAME_FIELD_NUMBER: _ClassVar[int]
    STAMP_FIELD_NUMBER: _ClassVar[int]
    name: str
    stamp: int
    def __init__(self, name: _Optional[str] = ..., stamp: _Optional[int] = ...) -> None: ...
