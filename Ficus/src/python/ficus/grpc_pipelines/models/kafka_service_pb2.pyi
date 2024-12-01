import pipelines_and_context_pb2 as _pipelines_and_context_pb2
import util_pb2 as _util_pb2
import backend_service_pb2 as _backend_service_pb2
from google.protobuf import empty_pb2 as _empty_pb2
from google.protobuf.internal import containers as _containers
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from typing import ClassVar as _ClassVar, Iterable as _Iterable, Mapping as _Mapping, Optional as _Optional, Union as _Union

DESCRIPTOR: _descriptor.FileDescriptor

class GrpcExecutePipelineAndProduceKafkaRequest(_message.Message):
    __slots__ = ["pipelineRequest", "producerMetadata", "caseInfo"]
    PIPELINEREQUEST_FIELD_NUMBER: _ClassVar[int]
    PRODUCERMETADATA_FIELD_NUMBER: _ClassVar[int]
    CASEINFO_FIELD_NUMBER: _ClassVar[int]
    pipelineRequest: _backend_service_pb2.GrpcProxyPipelineExecutionRequest
    producerMetadata: GrpcKafkaConnectionMetadata
    caseInfo: GrpcProcessInfo
    def __init__(self, pipelineRequest: _Optional[_Union[_backend_service_pb2.GrpcProxyPipelineExecutionRequest, _Mapping]] = ..., producerMetadata: _Optional[_Union[GrpcKafkaConnectionMetadata, _Mapping]] = ..., caseInfo: _Optional[_Union[GrpcProcessInfo, _Mapping]] = ...) -> None: ...

class GrpcProcessInfo(_message.Message):
    __slots__ = ["processName", "caseName"]
    PROCESSNAME_FIELD_NUMBER: _ClassVar[int]
    CASENAME_FIELD_NUMBER: _ClassVar[int]
    processName: str
    caseName: str
    def __init__(self, processName: _Optional[str] = ..., caseName: _Optional[str] = ...) -> None: ...

class GrpcSubscribeToKafkaRequest(_message.Message):
    __slots__ = ["connectionMetadata"]
    CONNECTIONMETADATA_FIELD_NUMBER: _ClassVar[int]
    connectionMetadata: GrpcKafkaConnectionMetadata
    def __init__(self, connectionMetadata: _Optional[_Union[GrpcKafkaConnectionMetadata, _Mapping]] = ...) -> None: ...

class GrpcAddPipelineRequest(_message.Message):
    __slots__ = ["subscriptionId", "pipelineRequest", "resultsToKafkaTopic"]
    SUBSCRIPTIONID_FIELD_NUMBER: _ClassVar[int]
    PIPELINEREQUEST_FIELD_NUMBER: _ClassVar[int]
    RESULTSTOKAFKATOPIC_FIELD_NUMBER: _ClassVar[int]
    subscriptionId: _util_pb2.GrpcGuid
    pipelineRequest: _backend_service_pb2.GrpcPipelineExecutionRequest
    resultsToKafkaTopic: GrpcKafkaConnectionMetadata
    def __init__(self, subscriptionId: _Optional[_Union[_util_pb2.GrpcGuid, _Mapping]] = ..., pipelineRequest: _Optional[_Union[_backend_service_pb2.GrpcPipelineExecutionRequest, _Mapping]] = ..., resultsToKafkaTopic: _Optional[_Union[GrpcKafkaConnectionMetadata, _Mapping]] = ...) -> None: ...

class GrpcAddPipelineStreamRequest(_message.Message):
    __slots__ = ["subscriptionId", "pipelineRequest"]
    SUBSCRIPTIONID_FIELD_NUMBER: _ClassVar[int]
    PIPELINEREQUEST_FIELD_NUMBER: _ClassVar[int]
    subscriptionId: _util_pb2.GrpcGuid
    pipelineRequest: _backend_service_pb2.GrpcPipelineExecutionRequest
    def __init__(self, subscriptionId: _Optional[_Union[_util_pb2.GrpcGuid, _Mapping]] = ..., pipelineRequest: _Optional[_Union[_backend_service_pb2.GrpcPipelineExecutionRequest, _Mapping]] = ...) -> None: ...

class GrpcRemovePipelineRequest(_message.Message):
    __slots__ = ["subscriptionId", "pipelineId"]
    SUBSCRIPTIONID_FIELD_NUMBER: _ClassVar[int]
    PIPELINEID_FIELD_NUMBER: _ClassVar[int]
    subscriptionId: _util_pb2.GrpcGuid
    pipelineId: _util_pb2.GrpcGuid
    def __init__(self, subscriptionId: _Optional[_Union[_util_pb2.GrpcGuid, _Mapping]] = ..., pipelineId: _Optional[_Union[_util_pb2.GrpcGuid, _Mapping]] = ...) -> None: ...

class GrpcRemoveAllPipelinesRequest(_message.Message):
    __slots__ = ["subscriptionId"]
    SUBSCRIPTIONID_FIELD_NUMBER: _ClassVar[int]
    subscriptionId: _util_pb2.GrpcGuid
    def __init__(self, subscriptionId: _Optional[_Union[_util_pb2.GrpcGuid, _Mapping]] = ...) -> None: ...

class GrpcKafkaConnectionMetadata(_message.Message):
    __slots__ = ["topicName", "metadata"]
    TOPICNAME_FIELD_NUMBER: _ClassVar[int]
    METADATA_FIELD_NUMBER: _ClassVar[int]
    topicName: str
    metadata: _containers.RepeatedCompositeFieldContainer[GrpcKafkaConsumerMetadata]
    def __init__(self, topicName: _Optional[str] = ..., metadata: _Optional[_Iterable[_Union[GrpcKafkaConsumerMetadata, _Mapping]]] = ...) -> None: ...

class GrpcKafkaConsumerMetadata(_message.Message):
    __slots__ = ["key", "value"]
    KEY_FIELD_NUMBER: _ClassVar[int]
    VALUE_FIELD_NUMBER: _ClassVar[int]
    key: str
    value: str
    def __init__(self, key: _Optional[str] = ..., value: _Optional[str] = ...) -> None: ...

class GrpcKafkaResult(_message.Message):
    __slots__ = ["success", "failure"]
    SUCCESS_FIELD_NUMBER: _ClassVar[int]
    FAILURE_FIELD_NUMBER: _ClassVar[int]
    success: GrpcKafkaSuccessResult
    failure: GrpcKafkaFailedResult
    def __init__(self, success: _Optional[_Union[GrpcKafkaSuccessResult, _Mapping]] = ..., failure: _Optional[_Union[GrpcKafkaFailedResult, _Mapping]] = ...) -> None: ...

class GrpcKafkaSuccessResult(_message.Message):
    __slots__ = ["id"]
    ID_FIELD_NUMBER: _ClassVar[int]
    id: _util_pb2.GrpcGuid
    def __init__(self, id: _Optional[_Union[_util_pb2.GrpcGuid, _Mapping]] = ...) -> None: ...

class GrpcKafkaFailedResult(_message.Message):
    __slots__ = ["errorMessage"]
    ERRORMESSAGE_FIELD_NUMBER: _ClassVar[int]
    errorMessage: str
    def __init__(self, errorMessage: _Optional[str] = ...) -> None: ...

class GrpcUnsubscribeFromKafkaRequest(_message.Message):
    __slots__ = ["subscriptionId"]
    SUBSCRIPTIONID_FIELD_NUMBER: _ClassVar[int]
    subscriptionId: _util_pb2.GrpcGuid
    def __init__(self, subscriptionId: _Optional[_Union[_util_pb2.GrpcGuid, _Mapping]] = ...) -> None: ...
