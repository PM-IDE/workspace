import pipelines_and_context_pb2 as _pipelines_and_context_pb2
import util_pb2 as _util_pb2
from google.protobuf import empty_pb2 as _empty_pb2
from google.protobuf import timestamp_pb2 as _timestamp_pb2
from google.protobuf.internal import containers as _containers
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from typing import ClassVar as _ClassVar, Iterable as _Iterable, Mapping as _Mapping, Optional as _Optional, Union as _Union

DESCRIPTOR: _descriptor.FileDescriptor

class GrpcSubscriptionAndPipelinesStateResponse(_message.Message):
    __slots__ = ["cases"]
    CASES_FIELD_NUMBER: _ClassVar[int]
    cases: _containers.RepeatedCompositeFieldContainer[GrpcProcessCaseMetadataWithStamp]
    def __init__(self, cases: _Optional[_Iterable[_Union[GrpcProcessCaseMetadataWithStamp, _Mapping]]] = ...) -> None: ...

class GrpcProcessCaseMetadataWithStamp(_message.Message):
    __slots__ = ["stamp", "metadata"]
    STAMP_FIELD_NUMBER: _ClassVar[int]
    METADATA_FIELD_NUMBER: _ClassVar[int]
    stamp: int
    metadata: GrpcProcessCaseMetadata
    def __init__(self, stamp: _Optional[int] = ..., metadata: _Optional[_Union[GrpcProcessCaseMetadata, _Mapping]] = ...) -> None: ...

class GrpcGetPipelineCaseContextValuesRequest(_message.Message):
    __slots__ = ["subscriptionId", "pipelineId", "processName", "caseName"]
    SUBSCRIPTIONID_FIELD_NUMBER: _ClassVar[int]
    PIPELINEID_FIELD_NUMBER: _ClassVar[int]
    PROCESSNAME_FIELD_NUMBER: _ClassVar[int]
    CASENAME_FIELD_NUMBER: _ClassVar[int]
    subscriptionId: _util_pb2.GrpcGuid
    pipelineId: _util_pb2.GrpcGuid
    processName: str
    caseName: GrpcCaseName
    def __init__(self, subscriptionId: _Optional[_Union[_util_pb2.GrpcGuid, _Mapping]] = ..., pipelineId: _Optional[_Union[_util_pb2.GrpcGuid, _Mapping]] = ..., processName: _Optional[str] = ..., caseName: _Optional[_Union[GrpcCaseName, _Mapping]] = ...) -> None: ...

class GrpcKafkaUpdate(_message.Message):
    __slots__ = ["processCaseMetadata", "pipelinePartInfo", "contextValues"]
    PROCESSCASEMETADATA_FIELD_NUMBER: _ClassVar[int]
    PIPELINEPARTINFO_FIELD_NUMBER: _ClassVar[int]
    CONTEXTVALUES_FIELD_NUMBER: _ClassVar[int]
    processCaseMetadata: GrpcProcessCaseMetadata
    pipelinePartInfo: GrpcPipelinePartInfo
    contextValues: _containers.RepeatedCompositeFieldContainer[_pipelines_and_context_pb2.GrpcContextValueWithKeyName]
    def __init__(self, processCaseMetadata: _Optional[_Union[GrpcProcessCaseMetadata, _Mapping]] = ..., pipelinePartInfo: _Optional[_Union[GrpcPipelinePartInfo, _Mapping]] = ..., contextValues: _Optional[_Iterable[_Union[_pipelines_and_context_pb2.GrpcContextValueWithKeyName, _Mapping]]] = ...) -> None: ...

class GrpcProcessCaseMetadata(_message.Message):
    __slots__ = ["processName", "caseName", "subscriptionId", "subscriptionName", "pipelineId", "pipelineName", "metadata"]
    PROCESSNAME_FIELD_NUMBER: _ClassVar[int]
    CASENAME_FIELD_NUMBER: _ClassVar[int]
    SUBSCRIPTIONID_FIELD_NUMBER: _ClassVar[int]
    SUBSCRIPTIONNAME_FIELD_NUMBER: _ClassVar[int]
    PIPELINEID_FIELD_NUMBER: _ClassVar[int]
    PIPELINENAME_FIELD_NUMBER: _ClassVar[int]
    METADATA_FIELD_NUMBER: _ClassVar[int]
    processName: str
    caseName: GrpcCaseName
    subscriptionId: _util_pb2.GrpcGuid
    subscriptionName: str
    pipelineId: _util_pb2.GrpcGuid
    pipelineName: str
    metadata: _containers.RepeatedCompositeFieldContainer[_util_pb2.GrpcStringKeyValue]
    def __init__(self, processName: _Optional[str] = ..., caseName: _Optional[_Union[GrpcCaseName, _Mapping]] = ..., subscriptionId: _Optional[_Union[_util_pb2.GrpcGuid, _Mapping]] = ..., subscriptionName: _Optional[str] = ..., pipelineId: _Optional[_Union[_util_pb2.GrpcGuid, _Mapping]] = ..., pipelineName: _Optional[str] = ..., metadata: _Optional[_Iterable[_Union[_util_pb2.GrpcStringKeyValue, _Mapping]]] = ...) -> None: ...

class GrpcCaseName(_message.Message):
    __slots__ = ["displayName", "fullNameParts"]
    DISPLAYNAME_FIELD_NUMBER: _ClassVar[int]
    FULLNAMEPARTS_FIELD_NUMBER: _ClassVar[int]
    displayName: str
    fullNameParts: _containers.RepeatedScalarFieldContainer[str]
    def __init__(self, displayName: _Optional[str] = ..., fullNameParts: _Optional[_Iterable[str]] = ...) -> None: ...

class GrpcCaseContextValues(_message.Message):
    __slots__ = ["contextValues", "stamp"]
    CONTEXTVALUES_FIELD_NUMBER: _ClassVar[int]
    STAMP_FIELD_NUMBER: _ClassVar[int]
    contextValues: _containers.RepeatedCompositeFieldContainer[GrpcPipelinePartContextValues]
    stamp: int
    def __init__(self, contextValues: _Optional[_Iterable[_Union[GrpcPipelinePartContextValues, _Mapping]]] = ..., stamp: _Optional[int] = ...) -> None: ...

class GrpcPipelinePartContextValues(_message.Message):
    __slots__ = ["pipelinePartInfo", "stamp", "execution_results"]
    PIPELINEPARTINFO_FIELD_NUMBER: _ClassVar[int]
    STAMP_FIELD_NUMBER: _ClassVar[int]
    EXECUTION_RESULTS_FIELD_NUMBER: _ClassVar[int]
    pipelinePartInfo: GrpcPipelinePartInfo
    stamp: _timestamp_pb2.Timestamp
    execution_results: _containers.RepeatedCompositeFieldContainer[GrpcCasePipelinePartExecutionResult]
    def __init__(self, pipelinePartInfo: _Optional[_Union[GrpcPipelinePartInfo, _Mapping]] = ..., stamp: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ..., execution_results: _Optional[_Iterable[_Union[GrpcCasePipelinePartExecutionResult, _Mapping]]] = ...) -> None: ...

class GrpcCasePipelinePartExecutionResult(_message.Message):
    __slots__ = ["contextValues"]
    CONTEXTVALUES_FIELD_NUMBER: _ClassVar[int]
    contextValues: _containers.RepeatedCompositeFieldContainer[_pipelines_and_context_pb2.GrpcContextValueWithKeyName]
    def __init__(self, contextValues: _Optional[_Iterable[_Union[_pipelines_and_context_pb2.GrpcContextValueWithKeyName, _Mapping]]] = ...) -> None: ...

class GrpcPipelinePartInfo(_message.Message):
    __slots__ = ["name", "id", "execution_id"]
    NAME_FIELD_NUMBER: _ClassVar[int]
    ID_FIELD_NUMBER: _ClassVar[int]
    EXECUTION_ID_FIELD_NUMBER: _ClassVar[int]
    name: str
    id: _util_pb2.GrpcGuid
    execution_id: _util_pb2.GrpcGuid
    def __init__(self, name: _Optional[str] = ..., id: _Optional[_Union[_util_pb2.GrpcGuid, _Mapping]] = ..., execution_id: _Optional[_Union[_util_pb2.GrpcGuid, _Mapping]] = ...) -> None: ...
