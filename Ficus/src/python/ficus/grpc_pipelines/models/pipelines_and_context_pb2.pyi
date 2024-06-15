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

class GrpcContextValue(_message.Message):
    __slots__ = ["string", "hashes_log", "names_log", "uint32", "traces_sub_arrays", "trace_index_sub_arrays", "bool", "xes_event_log", "colors_log", "enum", "event_log_info", "strings", "pipeline", "petriNet", "graph", "float", "count_annotation", "frequency_annotation", "dataset", "labeled_dataset"]
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
    COUNT_ANNOTATION_FIELD_NUMBER: _ClassVar[int]
    FREQUENCY_ANNOTATION_FIELD_NUMBER: _ClassVar[int]
    DATASET_FIELD_NUMBER: _ClassVar[int]
    LABELED_DATASET_FIELD_NUMBER: _ClassVar[int]
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
    count_annotation: _pm_models_pb2.GrpcCountAnnotation
    frequency_annotation: _pm_models_pb2.GrpcFrequenciesAnnotation
    dataset: _pm_models_pb2.GrpcDataset
    labeled_dataset: _pm_models_pb2.GrpcLabeledDataset
    def __init__(self, string: _Optional[str] = ..., hashes_log: _Optional[_Union[GrpcHashesEventLogContextValue, _Mapping]] = ..., names_log: _Optional[_Union[GrpcNamesEventLogContextValue, _Mapping]] = ..., uint32: _Optional[int] = ..., traces_sub_arrays: _Optional[_Union[GrpcEventLogTraceSubArraysContextValue, _Mapping]] = ..., trace_index_sub_arrays: _Optional[_Union[GrpcSubArraysWithTraceIndexContextValue, _Mapping]] = ..., bool: bool = ..., xes_event_log: _Optional[_Union[GrpcNamesEventLogContextValue, _Mapping]] = ..., colors_log: _Optional[_Union[GrpcColorsEventLog, _Mapping]] = ..., enum: _Optional[_Union[GrpcEnum, _Mapping]] = ..., event_log_info: _Optional[_Union[GrpcEventLogInfo, _Mapping]] = ..., strings: _Optional[_Union[GrpcStrings, _Mapping]] = ..., pipeline: _Optional[_Union[GrpcPipeline, _Mapping]] = ..., petriNet: _Optional[_Union[_pm_models_pb2.GrpcPetriNet, _Mapping]] = ..., graph: _Optional[_Union[GrpcGraph, _Mapping]] = ..., float: _Optional[float] = ..., count_annotation: _Optional[_Union[_pm_models_pb2.GrpcCountAnnotation, _Mapping]] = ..., frequency_annotation: _Optional[_Union[_pm_models_pb2.GrpcFrequenciesAnnotation, _Mapping]] = ..., dataset: _Optional[_Union[_pm_models_pb2.GrpcDataset, _Mapping]] = ..., labeled_dataset: _Optional[_Union[_pm_models_pb2.GrpcLabeledDataset, _Mapping]] = ...) -> None: ...

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
    __slots__ = ["mapping", "traces"]
    MAPPING_FIELD_NUMBER: _ClassVar[int]
    TRACES_FIELD_NUMBER: _ClassVar[int]
    mapping: _containers.RepeatedCompositeFieldContainer[GrpcColorsEventLogMapping]
    traces: _containers.RepeatedCompositeFieldContainer[GrpcColorsTrace]
    def __init__(self, mapping: _Optional[_Iterable[_Union[GrpcColorsEventLogMapping, _Mapping]]] = ..., traces: _Optional[_Iterable[_Union[GrpcColorsTrace, _Mapping]]] = ...) -> None: ...

class GrpcColorsEventLogMapping(_message.Message):
    __slots__ = ["name", "color"]
    NAME_FIELD_NUMBER: _ClassVar[int]
    COLOR_FIELD_NUMBER: _ClassVar[int]
    name: str
    color: _util_pb2.GrpcColor
    def __init__(self, name: _Optional[str] = ..., color: _Optional[_Union[_util_pb2.GrpcColor, _Mapping]] = ...) -> None: ...

class GrpcColorsTrace(_message.Message):
    __slots__ = ["event_colors"]
    EVENT_COLORS_FIELD_NUMBER: _ClassVar[int]
    event_colors: _containers.RepeatedCompositeFieldContainer[GrpcColoredRectangle]
    def __init__(self, event_colors: _Optional[_Iterable[_Union[GrpcColoredRectangle, _Mapping]]] = ...) -> None: ...

class GrpcColoredRectangle(_message.Message):
    __slots__ = ["color_index", "start_index", "length"]
    COLOR_INDEX_FIELD_NUMBER: _ClassVar[int]
    START_INDEX_FIELD_NUMBER: _ClassVar[int]
    LENGTH_FIELD_NUMBER: _ClassVar[int]
    color_index: int
    start_index: int
    length: int
    def __init__(self, color_index: _Optional[int] = ..., start_index: _Optional[int] = ..., length: _Optional[int] = ...) -> None: ...

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
    __slots__ = ["key", "frontendPartUuid"]
    KEY_FIELD_NUMBER: _ClassVar[int]
    FRONTENDPARTUUID_FIELD_NUMBER: _ClassVar[int]
    key: GrpcContextKey
    frontendPartUuid: _util_pb2.GrpcUuid
    def __init__(self, key: _Optional[_Union[GrpcContextKey, _Mapping]] = ..., frontendPartUuid: _Optional[_Union[_util_pb2.GrpcUuid, _Mapping]] = ...) -> None: ...

class GrpcComplexContextRequestPipelinePart(_message.Message):
    __slots__ = ["keys", "beforePipelinePart", "frontendPartUuid"]
    KEYS_FIELD_NUMBER: _ClassVar[int]
    BEFOREPIPELINEPART_FIELD_NUMBER: _ClassVar[int]
    FRONTENDPARTUUID_FIELD_NUMBER: _ClassVar[int]
    keys: _containers.RepeatedCompositeFieldContainer[GrpcContextKey]
    beforePipelinePart: GrpcPipelinePart
    frontendPartUuid: _util_pb2.GrpcUuid
    def __init__(self, keys: _Optional[_Iterable[_Union[GrpcContextKey, _Mapping]]] = ..., beforePipelinePart: _Optional[_Union[GrpcPipelinePart, _Mapping]] = ..., frontendPartUuid: _Optional[_Union[_util_pb2.GrpcUuid, _Mapping]] = ...) -> None: ...

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
    __slots__ = ["from_node", "to_node", "data"]
    FROM_NODE_FIELD_NUMBER: _ClassVar[int]
    TO_NODE_FIELD_NUMBER: _ClassVar[int]
    DATA_FIELD_NUMBER: _ClassVar[int]
    from_node: int
    to_node: int
    data: str
    def __init__(self, from_node: _Optional[int] = ..., to_node: _Optional[int] = ..., data: _Optional[str] = ...) -> None: ...
