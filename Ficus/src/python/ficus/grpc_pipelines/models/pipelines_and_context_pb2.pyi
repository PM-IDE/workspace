import pm_models_pb2 as _pm_models_pb2
import util_pb2 as _util_pb2
from google.protobuf import empty_pb2 as _empty_pb2
from google.protobuf.internal import containers as _containers
from google.protobuf.internal import enum_type_wrapper as _enum_type_wrapper
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from typing import ClassVar as _ClassVar, Iterable as _Iterable, Mapping as _Mapping, Optional as _Optional, Union as _Union

DESCRIPTOR: _descriptor.FileDescriptor

class GrpcGraphKind(int, metaclass=_enum_type_wrapper.EnumTypeWrapper):
    __slots__ = []
    None: _ClassVar[GrpcGraphKind]
    DAG: _ClassVar[GrpcGraphKind]

class GrpcThreadEventKind(int, metaclass=_enum_type_wrapper.EnumTypeWrapper):
    __slots__ = []
    Created: _ClassVar[GrpcThreadEventKind]
    Terminated: _ClassVar[GrpcThreadEventKind]

class GrpcAssemblyEventKind(int, metaclass=_enum_type_wrapper.EnumTypeWrapper):
    __slots__ = []
    Loaded: _ClassVar[GrpcAssemblyEventKind]
    Unloaded: _ClassVar[GrpcAssemblyEventKind]

class GrpcUnderlyingPatternKind(int, metaclass=_enum_type_wrapper.EnumTypeWrapper):
    __slots__ = []
    StrictLoop: _ClassVar[GrpcUnderlyingPatternKind]
    PrimitiveTandemArray: _ClassVar[GrpcUnderlyingPatternKind]
    MaximalTandemArray: _ClassVar[GrpcUnderlyingPatternKind]
    MaximalRepeat: _ClassVar[GrpcUnderlyingPatternKind]
    SuperMaximalRepeat: _ClassVar[GrpcUnderlyingPatternKind]
    NearSuperMaximalRepeat: _ClassVar[GrpcUnderlyingPatternKind]
    Unknown: _ClassVar[GrpcUnderlyingPatternKind]
None: GrpcGraphKind
DAG: GrpcGraphKind
Created: GrpcThreadEventKind
Terminated: GrpcThreadEventKind
Loaded: GrpcAssemblyEventKind
Unloaded: GrpcAssemblyEventKind
StrictLoop: GrpcUnderlyingPatternKind
PrimitiveTandemArray: GrpcUnderlyingPatternKind
MaximalTandemArray: GrpcUnderlyingPatternKind
MaximalRepeat: GrpcUnderlyingPatternKind
SuperMaximalRepeat: GrpcUnderlyingPatternKind
NearSuperMaximalRepeat: GrpcUnderlyingPatternKind
Unknown: GrpcUnderlyingPatternKind

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
    __slots__ = ["string", "hashes_log", "names_log", "uint32", "traces_sub_arrays", "trace_index_sub_arrays", "bool", "xes_event_log", "colors_log", "enum", "event_log_info", "strings", "pipeline", "petriNet", "graph", "float", "annotation", "dataset", "labeled_dataset", "bytes", "logTimelineDiagram", "float_array", "int_array", "uint_array", "json"]
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
    FLOAT_ARRAY_FIELD_NUMBER: _ClassVar[int]
    INT_ARRAY_FIELD_NUMBER: _ClassVar[int]
    UINT_ARRAY_FIELD_NUMBER: _ClassVar[int]
    JSON_FIELD_NUMBER: _ClassVar[int]
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
    float_array: GrpcFloatArray
    int_array: GrpcIntArray
    uint_array: GrpcUintArray
    json: str
    def __init__(self, string: _Optional[str] = ..., hashes_log: _Optional[_Union[GrpcHashesEventLogContextValue, _Mapping]] = ..., names_log: _Optional[_Union[GrpcNamesEventLogContextValue, _Mapping]] = ..., uint32: _Optional[int] = ..., traces_sub_arrays: _Optional[_Union[GrpcEventLogTraceSubArraysContextValue, _Mapping]] = ..., trace_index_sub_arrays: _Optional[_Union[GrpcSubArraysWithTraceIndexContextValue, _Mapping]] = ..., bool: bool = ..., xes_event_log: _Optional[_Union[GrpcNamesEventLogContextValue, _Mapping]] = ..., colors_log: _Optional[_Union[GrpcColorsEventLog, _Mapping]] = ..., enum: _Optional[_Union[GrpcEnum, _Mapping]] = ..., event_log_info: _Optional[_Union[GrpcEventLogInfo, _Mapping]] = ..., strings: _Optional[_Union[GrpcStrings, _Mapping]] = ..., pipeline: _Optional[_Union[GrpcPipeline, _Mapping]] = ..., petriNet: _Optional[_Union[_pm_models_pb2.GrpcPetriNet, _Mapping]] = ..., graph: _Optional[_Union[GrpcGraph, _Mapping]] = ..., float: _Optional[float] = ..., annotation: _Optional[_Union[_pm_models_pb2.GrpcAnnotation, _Mapping]] = ..., dataset: _Optional[_Union[_pm_models_pb2.GrpcDataset, _Mapping]] = ..., labeled_dataset: _Optional[_Union[_pm_models_pb2.GrpcLabeledDataset, _Mapping]] = ..., bytes: _Optional[_Union[GrpcBytes, _Mapping]] = ..., logTimelineDiagram: _Optional[_Union[GrpcLogTimelineDiagram, _Mapping]] = ..., float_array: _Optional[_Union[GrpcFloatArray, _Mapping]] = ..., int_array: _Optional[_Union[GrpcIntArray, _Mapping]] = ..., uint_array: _Optional[_Union[GrpcUintArray, _Mapping]] = ..., json: _Optional[str] = ...) -> None: ...

class GrpcFloatArray(_message.Message):
    __slots__ = ["items"]
    ITEMS_FIELD_NUMBER: _ClassVar[int]
    items: _containers.RepeatedScalarFieldContainer[float]
    def __init__(self, items: _Optional[_Iterable[float]] = ...) -> None: ...

class GrpcIntArray(_message.Message):
    __slots__ = ["items"]
    ITEMS_FIELD_NUMBER: _ClassVar[int]
    items: _containers.RepeatedScalarFieldContainer[int]
    def __init__(self, items: _Optional[_Iterable[int]] = ...) -> None: ...

class GrpcUintArray(_message.Message):
    __slots__ = ["items"]
    ITEMS_FIELD_NUMBER: _ClassVar[int]
    items: _containers.RepeatedScalarFieldContainer[int]
    def __init__(self, items: _Optional[_Iterable[int]] = ...) -> None: ...

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
    __slots__ = ["up_left_point", "down_right_point", "extend_to_nearest_vertical_borders"]
    UP_LEFT_POINT_FIELD_NUMBER: _ClassVar[int]
    DOWN_RIGHT_POINT_FIELD_NUMBER: _ClassVar[int]
    EXTEND_TO_NEAREST_VERTICAL_BORDERS_FIELD_NUMBER: _ClassVar[int]
    up_left_point: GrpcLogPoint
    down_right_point: GrpcLogPoint
    extend_to_nearest_vertical_borders: bool
    def __init__(self, up_left_point: _Optional[_Union[GrpcLogPoint, _Mapping]] = ..., down_right_point: _Optional[_Union[GrpcLogPoint, _Mapping]] = ..., extend_to_nearest_vertical_borders: bool = ...) -> None: ...

class GrpcLogPoint(_message.Message):
    __slots__ = ["trace_index", "event_index"]
    TRACE_INDEX_FIELD_NUMBER: _ClassVar[int]
    EVENT_INDEX_FIELD_NUMBER: _ClassVar[int]
    trace_index: int
    event_index: int
    def __init__(self, trace_index: _Optional[int] = ..., event_index: _Optional[int] = ...) -> None: ...

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
    __slots__ = ["nodes", "edges", "kind"]
    NODES_FIELD_NUMBER: _ClassVar[int]
    EDGES_FIELD_NUMBER: _ClassVar[int]
    KIND_FIELD_NUMBER: _ClassVar[int]
    nodes: _containers.RepeatedCompositeFieldContainer[GrpcGraphNode]
    edges: _containers.RepeatedCompositeFieldContainer[GrpcGraphEdge]
    kind: GrpcGraphKind
    def __init__(self, nodes: _Optional[_Iterable[_Union[GrpcGraphNode, _Mapping]]] = ..., edges: _Optional[_Iterable[_Union[GrpcGraphEdge, _Mapping]]] = ..., kind: _Optional[_Union[GrpcGraphKind, str]] = ...) -> None: ...

class GrpcGraphNode(_message.Message):
    __slots__ = ["id", "data", "additional_data", "inner_graph"]
    ID_FIELD_NUMBER: _ClassVar[int]
    DATA_FIELD_NUMBER: _ClassVar[int]
    ADDITIONAL_DATA_FIELD_NUMBER: _ClassVar[int]
    INNER_GRAPH_FIELD_NUMBER: _ClassVar[int]
    id: int
    data: str
    additional_data: _containers.RepeatedCompositeFieldContainer[GrpcNodeAdditionalData]
    inner_graph: GrpcGraph
    def __init__(self, id: _Optional[int] = ..., data: _Optional[str] = ..., additional_data: _Optional[_Iterable[_Union[GrpcNodeAdditionalData, _Mapping]]] = ..., inner_graph: _Optional[_Union[GrpcGraph, _Mapping]] = ...) -> None: ...

class GrpcNodeAdditionalData(_message.Message):
    __slots__ = ["none", "software_data", "pattern_info", "trace_data", "time_data", "multithreaded_fragment", "original_event_coordinates"]
    NONE_FIELD_NUMBER: _ClassVar[int]
    SOFTWARE_DATA_FIELD_NUMBER: _ClassVar[int]
    PATTERN_INFO_FIELD_NUMBER: _ClassVar[int]
    TRACE_DATA_FIELD_NUMBER: _ClassVar[int]
    TIME_DATA_FIELD_NUMBER: _ClassVar[int]
    MULTITHREADED_FRAGMENT_FIELD_NUMBER: _ClassVar[int]
    ORIGINAL_EVENT_COORDINATES_FIELD_NUMBER: _ClassVar[int]
    none: _empty_pb2.Empty
    software_data: GrpcSoftwareData
    pattern_info: GrpcUnderlyingPatternInfo
    trace_data: GrpcNodeCorrespondingTraceData
    time_data: GrpcActivityStartEndData
    multithreaded_fragment: GrpcMultithreadedFragment
    original_event_coordinates: GrpcEventCoordinates
    def __init__(self, none: _Optional[_Union[_empty_pb2.Empty, _Mapping]] = ..., software_data: _Optional[_Union[GrpcSoftwareData, _Mapping]] = ..., pattern_info: _Optional[_Union[GrpcUnderlyingPatternInfo, _Mapping]] = ..., trace_data: _Optional[_Union[GrpcNodeCorrespondingTraceData, _Mapping]] = ..., time_data: _Optional[_Union[GrpcActivityStartEndData, _Mapping]] = ..., multithreaded_fragment: _Optional[_Union[GrpcMultithreadedFragment, _Mapping]] = ..., original_event_coordinates: _Optional[_Union[GrpcEventCoordinates, _Mapping]] = ...) -> None: ...

class GrpcMultithreadedFragment(_message.Message):
    __slots__ = ["multithreaded_log"]
    MULTITHREADED_LOG_FIELD_NUMBER: _ClassVar[int]
    multithreaded_log: _pm_models_pb2.GrpcSimpleEventLog
    def __init__(self, multithreaded_log: _Optional[_Union[_pm_models_pb2.GrpcSimpleEventLog, _Mapping]] = ...) -> None: ...

class GrpcActivityStartEndData(_message.Message):
    __slots__ = ["start_time", "end_time"]
    START_TIME_FIELD_NUMBER: _ClassVar[int]
    END_TIME_FIELD_NUMBER: _ClassVar[int]
    start_time: int
    end_time: int
    def __init__(self, start_time: _Optional[int] = ..., end_time: _Optional[int] = ...) -> None: ...

class GrpcEventCoordinates(_message.Message):
    __slots__ = ["trace_id", "event_index"]
    TRACE_ID_FIELD_NUMBER: _ClassVar[int]
    EVENT_INDEX_FIELD_NUMBER: _ClassVar[int]
    trace_id: int
    event_index: int
    def __init__(self, trace_id: _Optional[int] = ..., event_index: _Optional[int] = ...) -> None: ...

class GrpcNodeCorrespondingTraceData(_message.Message):
    __slots__ = ["belongs_to_root_sequence"]
    BELONGS_TO_ROOT_SEQUENCE_FIELD_NUMBER: _ClassVar[int]
    belongs_to_root_sequence: bool
    def __init__(self, belongs_to_root_sequence: bool = ...) -> None: ...

class GrpcSoftwareData(_message.Message):
    __slots__ = ["histogram", "timeline_diagram_fragment", "allocations_info", "execution_suspension_info", "thread_events", "methods_inlining_events", "array_pool_events", "exception_events", "http_events", "contention_events", "socket_event", "methods_load_unload_events", "histogram_data", "simple_counter_data", "activities_durations_data"]
    HISTOGRAM_FIELD_NUMBER: _ClassVar[int]
    TIMELINE_DIAGRAM_FRAGMENT_FIELD_NUMBER: _ClassVar[int]
    ALLOCATIONS_INFO_FIELD_NUMBER: _ClassVar[int]
    EXECUTION_SUSPENSION_INFO_FIELD_NUMBER: _ClassVar[int]
    THREAD_EVENTS_FIELD_NUMBER: _ClassVar[int]
    METHODS_INLINING_EVENTS_FIELD_NUMBER: _ClassVar[int]
    ARRAY_POOL_EVENTS_FIELD_NUMBER: _ClassVar[int]
    EXCEPTION_EVENTS_FIELD_NUMBER: _ClassVar[int]
    HTTP_EVENTS_FIELD_NUMBER: _ClassVar[int]
    CONTENTION_EVENTS_FIELD_NUMBER: _ClassVar[int]
    SOCKET_EVENT_FIELD_NUMBER: _ClassVar[int]
    METHODS_LOAD_UNLOAD_EVENTS_FIELD_NUMBER: _ClassVar[int]
    HISTOGRAM_DATA_FIELD_NUMBER: _ClassVar[int]
    SIMPLE_COUNTER_DATA_FIELD_NUMBER: _ClassVar[int]
    ACTIVITIES_DURATIONS_DATA_FIELD_NUMBER: _ClassVar[int]
    histogram: _containers.RepeatedCompositeFieldContainer[GrpcHistogramEntry]
    timeline_diagram_fragment: GrpcTimelineDiagramFragment
    allocations_info: _containers.RepeatedCompositeFieldContainer[GrpcAllocationInfo]
    execution_suspension_info: _containers.RepeatedCompositeFieldContainer[GrpcExecutionSuspensionInfo]
    thread_events: _containers.RepeatedCompositeFieldContainer[GrpcThreadEventInfo]
    methods_inlining_events: _containers.RepeatedCompositeFieldContainer[GrpcMethodInliningEvent]
    array_pool_events: _containers.RepeatedCompositeFieldContainer[GrpcArrayPoolEvent]
    exception_events: _containers.RepeatedCompositeFieldContainer[GrpcExceptionEvent]
    http_events: _containers.RepeatedCompositeFieldContainer[GrpcHTTPEvent]
    contention_events: _containers.RepeatedCompositeFieldContainer[GrpcContentionEvent]
    socket_event: _containers.RepeatedCompositeFieldContainer[GrpcSocketEvent]
    methods_load_unload_events: _containers.RepeatedCompositeFieldContainer[GrpcMethodLoadUnloadEvent]
    histogram_data: _containers.RepeatedCompositeFieldContainer[GrpcGeneralHistogramData]
    simple_counter_data: _containers.RepeatedCompositeFieldContainer[GrpcSimpleCounterData]
    activities_durations_data: _containers.RepeatedCompositeFieldContainer[GrpcActivityDurationData]
    def __init__(self, histogram: _Optional[_Iterable[_Union[GrpcHistogramEntry, _Mapping]]] = ..., timeline_diagram_fragment: _Optional[_Union[GrpcTimelineDiagramFragment, _Mapping]] = ..., allocations_info: _Optional[_Iterable[_Union[GrpcAllocationInfo, _Mapping]]] = ..., execution_suspension_info: _Optional[_Iterable[_Union[GrpcExecutionSuspensionInfo, _Mapping]]] = ..., thread_events: _Optional[_Iterable[_Union[GrpcThreadEventInfo, _Mapping]]] = ..., methods_inlining_events: _Optional[_Iterable[_Union[GrpcMethodInliningEvent, _Mapping]]] = ..., array_pool_events: _Optional[_Iterable[_Union[GrpcArrayPoolEvent, _Mapping]]] = ..., exception_events: _Optional[_Iterable[_Union[GrpcExceptionEvent, _Mapping]]] = ..., http_events: _Optional[_Iterable[_Union[GrpcHTTPEvent, _Mapping]]] = ..., contention_events: _Optional[_Iterable[_Union[GrpcContentionEvent, _Mapping]]] = ..., socket_event: _Optional[_Iterable[_Union[GrpcSocketEvent, _Mapping]]] = ..., methods_load_unload_events: _Optional[_Iterable[_Union[GrpcMethodLoadUnloadEvent, _Mapping]]] = ..., histogram_data: _Optional[_Iterable[_Union[GrpcGeneralHistogramData, _Mapping]]] = ..., simple_counter_data: _Optional[_Iterable[_Union[GrpcSimpleCounterData, _Mapping]]] = ..., activities_durations_data: _Optional[_Iterable[_Union[GrpcActivityDurationData, _Mapping]]] = ...) -> None: ...

class GrpcActivityDurationData(_message.Message):
    __slots__ = ["name", "duration", "units"]
    NAME_FIELD_NUMBER: _ClassVar[int]
    DURATION_FIELD_NUMBER: _ClassVar[int]
    UNITS_FIELD_NUMBER: _ClassVar[int]
    name: str
    duration: float
    units: str
    def __init__(self, name: _Optional[str] = ..., duration: _Optional[float] = ..., units: _Optional[str] = ...) -> None: ...

class GrpcGeneralHistogramData(_message.Message):
    __slots__ = ["name", "entries", "units"]
    NAME_FIELD_NUMBER: _ClassVar[int]
    ENTRIES_FIELD_NUMBER: _ClassVar[int]
    UNITS_FIELD_NUMBER: _ClassVar[int]
    name: str
    entries: _containers.RepeatedCompositeFieldContainer[GrpcHistogramEntry]
    units: str
    def __init__(self, name: _Optional[str] = ..., entries: _Optional[_Iterable[_Union[GrpcHistogramEntry, _Mapping]]] = ..., units: _Optional[str] = ...) -> None: ...

class GrpcSimpleCounterData(_message.Message):
    __slots__ = ["name", "count", "units"]
    NAME_FIELD_NUMBER: _ClassVar[int]
    COUNT_FIELD_NUMBER: _ClassVar[int]
    UNITS_FIELD_NUMBER: _ClassVar[int]
    name: str
    count: float
    units: str
    def __init__(self, name: _Optional[str] = ..., count: _Optional[float] = ..., units: _Optional[str] = ...) -> None: ...

class GrpcMethodLoadUnloadEvent(_message.Message):
    __slots__ = ["method_name_parts", "load", "unload"]
    METHOD_NAME_PARTS_FIELD_NUMBER: _ClassVar[int]
    LOAD_FIELD_NUMBER: _ClassVar[int]
    UNLOAD_FIELD_NUMBER: _ClassVar[int]
    method_name_parts: GrpcMethodNameParts
    load: _empty_pb2.Empty
    unload: _empty_pb2.Empty
    def __init__(self, method_name_parts: _Optional[_Union[GrpcMethodNameParts, _Mapping]] = ..., load: _Optional[_Union[_empty_pb2.Empty, _Mapping]] = ..., unload: _Optional[_Union[_empty_pb2.Empty, _Mapping]] = ...) -> None: ...

class GrpcExecutionSuspensionInfo(_message.Message):
    __slots__ = ["reason", "start_time", "end_time"]
    REASON_FIELD_NUMBER: _ClassVar[int]
    START_TIME_FIELD_NUMBER: _ClassVar[int]
    END_TIME_FIELD_NUMBER: _ClassVar[int]
    reason: str
    start_time: int
    end_time: int
    def __init__(self, reason: _Optional[str] = ..., start_time: _Optional[int] = ..., end_time: _Optional[int] = ...) -> None: ...

class GrpcMethodInliningEvent(_message.Message):
    __slots__ = ["inlining_info", "succeeded", "failed"]
    INLINING_INFO_FIELD_NUMBER: _ClassVar[int]
    SUCCEEDED_FIELD_NUMBER: _ClassVar[int]
    FAILED_FIELD_NUMBER: _ClassVar[int]
    inlining_info: GrpcMethodInliningInfo
    succeeded: _empty_pb2.Empty
    failed: GrpcMethodInliningFailedEvent
    def __init__(self, inlining_info: _Optional[_Union[GrpcMethodInliningInfo, _Mapping]] = ..., succeeded: _Optional[_Union[_empty_pb2.Empty, _Mapping]] = ..., failed: _Optional[_Union[GrpcMethodInliningFailedEvent, _Mapping]] = ...) -> None: ...

class GrpcMethodInliningInfo(_message.Message):
    __slots__ = ["inlinee_info", "inliner_info"]
    INLINEE_INFO_FIELD_NUMBER: _ClassVar[int]
    INLINER_INFO_FIELD_NUMBER: _ClassVar[int]
    inlinee_info: GrpcMethodNameParts
    inliner_info: GrpcMethodNameParts
    def __init__(self, inlinee_info: _Optional[_Union[GrpcMethodNameParts, _Mapping]] = ..., inliner_info: _Optional[_Union[GrpcMethodNameParts, _Mapping]] = ...) -> None: ...

class GrpcMethodNameParts(_message.Message):
    __slots__ = ["name", "namespace", "signature"]
    NAME_FIELD_NUMBER: _ClassVar[int]
    NAMESPACE_FIELD_NUMBER: _ClassVar[int]
    SIGNATURE_FIELD_NUMBER: _ClassVar[int]
    name: str
    namespace: str
    signature: str
    def __init__(self, name: _Optional[str] = ..., namespace: _Optional[str] = ..., signature: _Optional[str] = ...) -> None: ...

class GrpcMethodInliningFailedEvent(_message.Message):
    __slots__ = ["reason"]
    REASON_FIELD_NUMBER: _ClassVar[int]
    reason: str
    def __init__(self, reason: _Optional[str] = ...) -> None: ...

class GrpcThreadEventInfo(_message.Message):
    __slots__ = ["thread_id", "created", "terminated"]
    THREAD_ID_FIELD_NUMBER: _ClassVar[int]
    CREATED_FIELD_NUMBER: _ClassVar[int]
    TERMINATED_FIELD_NUMBER: _ClassVar[int]
    thread_id: int
    created: _empty_pb2.Empty
    terminated: _empty_pb2.Empty
    def __init__(self, thread_id: _Optional[int] = ..., created: _Optional[_Union[_empty_pb2.Empty, _Mapping]] = ..., terminated: _Optional[_Union[_empty_pb2.Empty, _Mapping]] = ...) -> None: ...

class GrpcAssemblyEventInfo(_message.Message):
    __slots__ = ["assembly_name", "event_kind"]
    ASSEMBLY_NAME_FIELD_NUMBER: _ClassVar[int]
    EVENT_KIND_FIELD_NUMBER: _ClassVar[int]
    assembly_name: str
    event_kind: GrpcAssemblyEventKind
    def __init__(self, assembly_name: _Optional[str] = ..., event_kind: _Optional[_Union[GrpcAssemblyEventKind, str]] = ...) -> None: ...

class GrpcArrayPoolEvent(_message.Message):
    __slots__ = ["buffer_id", "buffer_size_bytes", "buffer_allocated", "buffer_rented", "buffer_returned", "buffer_trimmed"]
    BUFFER_ID_FIELD_NUMBER: _ClassVar[int]
    BUFFER_SIZE_BYTES_FIELD_NUMBER: _ClassVar[int]
    BUFFER_ALLOCATED_FIELD_NUMBER: _ClassVar[int]
    BUFFER_RENTED_FIELD_NUMBER: _ClassVar[int]
    BUFFER_RETURNED_FIELD_NUMBER: _ClassVar[int]
    BUFFER_TRIMMED_FIELD_NUMBER: _ClassVar[int]
    buffer_id: int
    buffer_size_bytes: int
    buffer_allocated: _empty_pb2.Empty
    buffer_rented: _empty_pb2.Empty
    buffer_returned: _empty_pb2.Empty
    buffer_trimmed: _empty_pb2.Empty
    def __init__(self, buffer_id: _Optional[int] = ..., buffer_size_bytes: _Optional[int] = ..., buffer_allocated: _Optional[_Union[_empty_pb2.Empty, _Mapping]] = ..., buffer_rented: _Optional[_Union[_empty_pb2.Empty, _Mapping]] = ..., buffer_returned: _Optional[_Union[_empty_pb2.Empty, _Mapping]] = ..., buffer_trimmed: _Optional[_Union[_empty_pb2.Empty, _Mapping]] = ...) -> None: ...

class GrpcExceptionEvent(_message.Message):
    __slots__ = ["exception_type"]
    EXCEPTION_TYPE_FIELD_NUMBER: _ClassVar[int]
    exception_type: str
    def __init__(self, exception_type: _Optional[str] = ...) -> None: ...

class GrpcHTTPEvent(_message.Message):
    __slots__ = ["host", "port", "scheme", "path_and_query"]
    HOST_FIELD_NUMBER: _ClassVar[int]
    PORT_FIELD_NUMBER: _ClassVar[int]
    SCHEME_FIELD_NUMBER: _ClassVar[int]
    PATH_AND_QUERY_FIELD_NUMBER: _ClassVar[int]
    host: str
    port: str
    scheme: str
    path_and_query: str
    def __init__(self, host: _Optional[str] = ..., port: _Optional[str] = ..., scheme: _Optional[str] = ..., path_and_query: _Optional[str] = ...) -> None: ...

class GrpcContentionEvent(_message.Message):
    __slots__ = ["start_time", "end_time"]
    START_TIME_FIELD_NUMBER: _ClassVar[int]
    END_TIME_FIELD_NUMBER: _ClassVar[int]
    start_time: int
    end_time: int
    def __init__(self, start_time: _Optional[int] = ..., end_time: _Optional[int] = ...) -> None: ...

class GrpcSocketEvent(_message.Message):
    __slots__ = ["connect_start", "accept_start", "connect_stop", "accept_stop", "connect_failed", "accept_failed"]
    CONNECT_START_FIELD_NUMBER: _ClassVar[int]
    ACCEPT_START_FIELD_NUMBER: _ClassVar[int]
    CONNECT_STOP_FIELD_NUMBER: _ClassVar[int]
    ACCEPT_STOP_FIELD_NUMBER: _ClassVar[int]
    CONNECT_FAILED_FIELD_NUMBER: _ClassVar[int]
    ACCEPT_FAILED_FIELD_NUMBER: _ClassVar[int]
    connect_start: GrpcSocketConnectStart
    accept_start: GrpcSocketAcceptStart
    connect_stop: GrpcSocketConnectStop
    accept_stop: GrpcSocketAcceptStop
    connect_failed: GrpcSocketConnectFailed
    accept_failed: GrpcSocketAcceptFailed
    def __init__(self, connect_start: _Optional[_Union[GrpcSocketConnectStart, _Mapping]] = ..., accept_start: _Optional[_Union[GrpcSocketAcceptStart, _Mapping]] = ..., connect_stop: _Optional[_Union[GrpcSocketConnectStop, _Mapping]] = ..., accept_stop: _Optional[_Union[GrpcSocketAcceptStop, _Mapping]] = ..., connect_failed: _Optional[_Union[GrpcSocketConnectFailed, _Mapping]] = ..., accept_failed: _Optional[_Union[GrpcSocketAcceptFailed, _Mapping]] = ...) -> None: ...

class GrpcSocketAcceptStart(_message.Message):
    __slots__ = ["address"]
    ADDRESS_FIELD_NUMBER: _ClassVar[int]
    address: str
    def __init__(self, address: _Optional[str] = ...) -> None: ...

class GrpcSocketConnectStart(_message.Message):
    __slots__ = ["address"]
    ADDRESS_FIELD_NUMBER: _ClassVar[int]
    address: str
    def __init__(self, address: _Optional[str] = ...) -> None: ...

class GrpcSocketConnectStop(_message.Message):
    __slots__ = []
    def __init__(self) -> None: ...

class GrpcSocketAcceptStop(_message.Message):
    __slots__ = []
    def __init__(self) -> None: ...

class GrpcSocketConnectFailed(_message.Message):
    __slots__ = ["error_code", "error_message"]
    ERROR_CODE_FIELD_NUMBER: _ClassVar[int]
    ERROR_MESSAGE_FIELD_NUMBER: _ClassVar[int]
    error_code: str
    error_message: str
    def __init__(self, error_code: _Optional[str] = ..., error_message: _Optional[str] = ...) -> None: ...

class GrpcSocketAcceptFailed(_message.Message):
    __slots__ = ["error_code", "error_message"]
    ERROR_CODE_FIELD_NUMBER: _ClassVar[int]
    ERROR_MESSAGE_FIELD_NUMBER: _ClassVar[int]
    error_code: str
    error_message: str
    def __init__(self, error_code: _Optional[str] = ..., error_message: _Optional[str] = ...) -> None: ...

class GrpcHistogramEntry(_message.Message):
    __slots__ = ["name", "count"]
    NAME_FIELD_NUMBER: _ClassVar[int]
    COUNT_FIELD_NUMBER: _ClassVar[int]
    name: str
    count: float
    def __init__(self, name: _Optional[str] = ..., count: _Optional[float] = ...) -> None: ...

class GrpcTimelineDiagramFragment(_message.Message):
    __slots__ = ["threads"]
    THREADS_FIELD_NUMBER: _ClassVar[int]
    threads: _containers.RepeatedCompositeFieldContainer[GrpcThread]
    def __init__(self, threads: _Optional[_Iterable[_Union[GrpcThread, _Mapping]]] = ...) -> None: ...

class GrpcAllocationInfo(_message.Message):
    __slots__ = ["type_name", "allocated_objects_count", "allocated_bytes"]
    TYPE_NAME_FIELD_NUMBER: _ClassVar[int]
    ALLOCATED_OBJECTS_COUNT_FIELD_NUMBER: _ClassVar[int]
    ALLOCATED_BYTES_FIELD_NUMBER: _ClassVar[int]
    type_name: str
    allocated_objects_count: int
    allocated_bytes: int
    def __init__(self, type_name: _Optional[str] = ..., allocated_objects_count: _Optional[int] = ..., allocated_bytes: _Optional[int] = ...) -> None: ...

class GrpcUnderlyingPatternInfo(_message.Message):
    __slots__ = ["pattern_kind", "base_sequence", "graph"]
    PATTERN_KIND_FIELD_NUMBER: _ClassVar[int]
    BASE_SEQUENCE_FIELD_NUMBER: _ClassVar[int]
    GRAPH_FIELD_NUMBER: _ClassVar[int]
    pattern_kind: GrpcUnderlyingPatternKind
    base_sequence: _containers.RepeatedScalarFieldContainer[str]
    graph: GrpcGraph
    def __init__(self, pattern_kind: _Optional[_Union[GrpcUnderlyingPatternKind, str]] = ..., base_sequence: _Optional[_Iterable[str]] = ..., graph: _Optional[_Union[GrpcGraph, _Mapping]] = ...) -> None: ...

class GrpcGraphEdge(_message.Message):
    __slots__ = ["id", "from_node", "to_node", "weight", "data", "additional_data"]
    ID_FIELD_NUMBER: _ClassVar[int]
    FROM_NODE_FIELD_NUMBER: _ClassVar[int]
    TO_NODE_FIELD_NUMBER: _ClassVar[int]
    WEIGHT_FIELD_NUMBER: _ClassVar[int]
    DATA_FIELD_NUMBER: _ClassVar[int]
    ADDITIONAL_DATA_FIELD_NUMBER: _ClassVar[int]
    id: int
    from_node: int
    to_node: int
    weight: float
    data: str
    additional_data: _containers.RepeatedCompositeFieldContainer[GrpcGraphEdgeAdditionalData]
    def __init__(self, id: _Optional[int] = ..., from_node: _Optional[int] = ..., to_node: _Optional[int] = ..., weight: _Optional[float] = ..., data: _Optional[str] = ..., additional_data: _Optional[_Iterable[_Union[GrpcGraphEdgeAdditionalData, _Mapping]]] = ...) -> None: ...

class GrpcGraphEdgeAdditionalData(_message.Message):
    __slots__ = ["software_data", "execution_info", "time_data"]
    SOFTWARE_DATA_FIELD_NUMBER: _ClassVar[int]
    EXECUTION_INFO_FIELD_NUMBER: _ClassVar[int]
    TIME_DATA_FIELD_NUMBER: _ClassVar[int]
    software_data: GrpcSoftwareData
    execution_info: GrpcEdgeExecutionInfo
    time_data: GrpcActivityStartEndData
    def __init__(self, software_data: _Optional[_Union[GrpcSoftwareData, _Mapping]] = ..., execution_info: _Optional[_Union[GrpcEdgeExecutionInfo, _Mapping]] = ..., time_data: _Optional[_Union[GrpcActivityStartEndData, _Mapping]] = ...) -> None: ...

class GrpcEdgeExecutionInfo(_message.Message):
    __slots__ = ["traceId"]
    TRACEID_FIELD_NUMBER: _ClassVar[int]
    traceId: int
    def __init__(self, traceId: _Optional[int] = ...) -> None: ...

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
