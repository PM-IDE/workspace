import pipelines_and_context_pb2 as _pipelines_and_context_pb2
import util_pb2 as _util_pb2
from google.protobuf import empty_pb2 as _empty_pb2
from google.protobuf.internal import containers as _containers
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from typing import ClassVar as _ClassVar, Iterable as _Iterable, Mapping as _Mapping, Optional as _Optional, Union as _Union

DESCRIPTOR: _descriptor.FileDescriptor

class GrpcFicusBackendInfo(_message.Message):
    __slots__ = ["name", "pipeline_parts"]
    NAME_FIELD_NUMBER: _ClassVar[int]
    PIPELINE_PARTS_FIELD_NUMBER: _ClassVar[int]
    name: str
    pipeline_parts: _containers.RepeatedCompositeFieldContainer[GrpcPipelinePartDescriptor]
    def __init__(self, name: _Optional[str] = ..., pipeline_parts: _Optional[_Iterable[_Union[GrpcPipelinePartDescriptor, _Mapping]]] = ...) -> None: ...

class GrpcPipelinePartDescriptor(_message.Message):
    __slots__ = ["name"]
    NAME_FIELD_NUMBER: _ClassVar[int]
    name: str
    def __init__(self, name: _Optional[str] = ...) -> None: ...

class GrpcGetContextValueRequest(_message.Message):
    __slots__ = ["executionId", "key"]
    EXECUTIONID_FIELD_NUMBER: _ClassVar[int]
    KEY_FIELD_NUMBER: _ClassVar[int]
    executionId: _util_pb2.GrpcGuid
    key: _pipelines_and_context_pb2.GrpcContextKey
    def __init__(self, executionId: _Optional[_Union[_util_pb2.GrpcGuid, _Mapping]] = ..., key: _Optional[_Union[_pipelines_and_context_pb2.GrpcContextKey, _Mapping]] = ...) -> None: ...

class GrpcPipelineExecutionRequest(_message.Message):
    __slots__ = ["pipeline", "initialContext"]
    PIPELINE_FIELD_NUMBER: _ClassVar[int]
    INITIALCONTEXT_FIELD_NUMBER: _ClassVar[int]
    pipeline: _pipelines_and_context_pb2.GrpcPipeline
    initialContext: _containers.RepeatedCompositeFieldContainer[_pipelines_and_context_pb2.GrpcContextKeyValue]
    def __init__(self, pipeline: _Optional[_Union[_pipelines_and_context_pb2.GrpcPipeline, _Mapping]] = ..., initialContext: _Optional[_Iterable[_Union[_pipelines_and_context_pb2.GrpcContextKeyValue, _Mapping]]] = ...) -> None: ...

class GrpcProxyPipelineExecutionRequest(_message.Message):
    __slots__ = ["pipeline", "contextValuesIds"]
    PIPELINE_FIELD_NUMBER: _ClassVar[int]
    CONTEXTVALUESIDS_FIELD_NUMBER: _ClassVar[int]
    pipeline: _pipelines_and_context_pb2.GrpcPipeline
    contextValuesIds: _containers.RepeatedCompositeFieldContainer[_util_pb2.GrpcGuid]
    def __init__(self, pipeline: _Optional[_Union[_pipelines_and_context_pb2.GrpcPipeline, _Mapping]] = ..., contextValuesIds: _Optional[_Iterable[_Union[_util_pb2.GrpcGuid, _Mapping]]] = ...) -> None: ...

class GrpcPipelinePartExecutionResult(_message.Message):
    __slots__ = ["finalResult", "pipelinePartResult", "logMessage"]
    FINALRESULT_FIELD_NUMBER: _ClassVar[int]
    PIPELINEPARTRESULT_FIELD_NUMBER: _ClassVar[int]
    LOGMESSAGE_FIELD_NUMBER: _ClassVar[int]
    finalResult: GrpcPipelineFinalResult
    pipelinePartResult: GrpcPipelinePartResult
    logMessage: GrpcPipelinePartLogMessage
    def __init__(self, finalResult: _Optional[_Union[GrpcPipelineFinalResult, _Mapping]] = ..., pipelinePartResult: _Optional[_Union[GrpcPipelinePartResult, _Mapping]] = ..., logMessage: _Optional[_Union[GrpcPipelinePartLogMessage, _Mapping]] = ...) -> None: ...

class GrpcPipelinePartLogMessage(_message.Message):
    __slots__ = ["message"]
    MESSAGE_FIELD_NUMBER: _ClassVar[int]
    message: str
    def __init__(self, message: _Optional[str] = ...) -> None: ...

class GrpcPipelinePartResult(_message.Message):
    __slots__ = ["contextValues", "uuid"]
    CONTEXTVALUES_FIELD_NUMBER: _ClassVar[int]
    UUID_FIELD_NUMBER: _ClassVar[int]
    contextValues: _containers.RepeatedCompositeFieldContainer[_pipelines_and_context_pb2.GrpcContextValueWithKeyName]
    uuid: _util_pb2.GrpcUuid
    def __init__(self, contextValues: _Optional[_Iterable[_Union[_pipelines_and_context_pb2.GrpcContextValueWithKeyName, _Mapping]]] = ..., uuid: _Optional[_Union[_util_pb2.GrpcUuid, _Mapping]] = ...) -> None: ...

class GrpcPipelineFinalResult(_message.Message):
    __slots__ = ["success", "error"]
    SUCCESS_FIELD_NUMBER: _ClassVar[int]
    ERROR_FIELD_NUMBER: _ClassVar[int]
    success: _util_pb2.GrpcGuid
    error: str
    def __init__(self, success: _Optional[_Union[_util_pb2.GrpcGuid, _Mapping]] = ..., error: _Optional[str] = ...) -> None: ...

class GrpcGetContextValueResult(_message.Message):
    __slots__ = ["key", "value", "error"]
    KEY_FIELD_NUMBER: _ClassVar[int]
    VALUE_FIELD_NUMBER: _ClassVar[int]
    ERROR_FIELD_NUMBER: _ClassVar[int]
    key: str
    value: _pipelines_and_context_pb2.GrpcContextValue
    error: str
    def __init__(self, key: _Optional[str] = ..., value: _Optional[_Union[_pipelines_and_context_pb2.GrpcContextValue, _Mapping]] = ..., error: _Optional[str] = ...) -> None: ...

class GrpcGetAllContextValuesResult(_message.Message):
    __slots__ = ["context_values"]
    CONTEXT_VALUES_FIELD_NUMBER: _ClassVar[int]
    context_values: _containers.RepeatedCompositeFieldContainer[GrpcGetContextValueResult]
    def __init__(self, context_values: _Optional[_Iterable[_Union[GrpcGetContextValueResult, _Mapping]]] = ...) -> None: ...
