import pipelines_and_context_pb2 as _pipelines_and_context_pb2
import util_pb2 as _util_pb2
from google.protobuf import empty_pb2 as _empty_pb2
from google.protobuf import timestamp_pb2 as _timestamp_pb2
from google.protobuf.internal import containers as _containers
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from typing import ClassVar as _ClassVar, Iterable as _Iterable, Mapping as _Mapping, Optional as _Optional, Union as _Union

DESCRIPTOR: _descriptor.FileDescriptor

class GrpcKafkaUpdate(_message.Message):
    __slots__ = ["caseName", "contextValues"]
    CASENAME_FIELD_NUMBER: _ClassVar[int]
    CONTEXTVALUES_FIELD_NUMBER: _ClassVar[int]
    caseName: str
    contextValues: _containers.RepeatedCompositeFieldContainer[_pipelines_and_context_pb2.GrpcContextValueWithKeyName]
    def __init__(self, caseName: _Optional[str] = ..., contextValues: _Optional[_Iterable[_Union[_pipelines_and_context_pb2.GrpcContextValueWithKeyName, _Mapping]]] = ...) -> None: ...

class GrpcCurrentCasesResponse(_message.Message):
    __slots__ = ["cases"]
    CASES_FIELD_NUMBER: _ClassVar[int]
    cases: _containers.RepeatedCompositeFieldContainer[GrpcCase]
    def __init__(self, cases: _Optional[_Iterable[_Union[GrpcCase, _Mapping]]] = ...) -> None: ...

class GrpcCase(_message.Message):
    __slots__ = ["caseName", "contextValues"]
    CASENAME_FIELD_NUMBER: _ClassVar[int]
    CONTEXTVALUES_FIELD_NUMBER: _ClassVar[int]
    caseName: str
    contextValues: _containers.RepeatedCompositeFieldContainer[GrpcPipelinePartContextValues]
    def __init__(self, caseName: _Optional[str] = ..., contextValues: _Optional[_Iterable[_Union[GrpcPipelinePartContextValues, _Mapping]]] = ...) -> None: ...

class GrpcPipelinePartContextValues(_message.Message):
    __slots__ = ["pipelinePartInfo", "stamp", "contextValues"]
    PIPELINEPARTINFO_FIELD_NUMBER: _ClassVar[int]
    STAMP_FIELD_NUMBER: _ClassVar[int]
    CONTEXTVALUES_FIELD_NUMBER: _ClassVar[int]
    pipelinePartInfo: GrpcPipelinePartInfo
    stamp: _timestamp_pb2.Timestamp
    contextValues: _containers.RepeatedCompositeFieldContainer[_pipelines_and_context_pb2.GrpcContextValueWithKeyName]
    def __init__(self, pipelinePartInfo: _Optional[_Union[GrpcPipelinePartInfo, _Mapping]] = ..., stamp: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ..., contextValues: _Optional[_Iterable[_Union[_pipelines_and_context_pb2.GrpcContextValueWithKeyName, _Mapping]]] = ...) -> None: ...

class GrpcPipelinePartInfo(_message.Message):
    __slots__ = ["name", "id"]
    NAME_FIELD_NUMBER: _ClassVar[int]
    ID_FIELD_NUMBER: _ClassVar[int]
    name: str
    id: _util_pb2.GrpcGuid
    def __init__(self, name: _Optional[str] = ..., id: _Optional[_Union[_util_pb2.GrpcGuid, _Mapping]] = ...) -> None: ...
