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
    __slots__ = ["connectionMetadata", "subscriptionMetadata"]
    CONNECTIONMETADATA_FIELD_NUMBER: _ClassVar[int]
    SUBSCRIPTIONMETADATA_FIELD_NUMBER: _ClassVar[int]
    connectionMetadata: GrpcKafkaConnectionMetadata
    subscriptionMetadata: GrpcKafkaSubscriptionMetadata
    def __init__(self, connectionMetadata: _Optional[_Union[GrpcKafkaConnectionMetadata, _Mapping]] = ..., subscriptionMetadata: _Optional[_Union[GrpcKafkaSubscriptionMetadata, _Mapping]] = ...) -> None: ...

class GrpcKafkaSubscriptionMetadata(_message.Message):
    __slots__ = ["subscriptionName"]
    SUBSCRIPTIONNAME_FIELD_NUMBER: _ClassVar[int]
    subscriptionName: str
    def __init__(self, subscriptionName: _Optional[str] = ...) -> None: ...

class GrpcKafkaPipelineExecutionRequest(_message.Message):
    __slots__ = ["subscriptionId", "pipelineRequest", "pipelineMetadata"]
    SUBSCRIPTIONID_FIELD_NUMBER: _ClassVar[int]
    PIPELINEREQUEST_FIELD_NUMBER: _ClassVar[int]
    PIPELINEMETADATA_FIELD_NUMBER: _ClassVar[int]
    subscriptionId: _util_pb2.GrpcGuid
    pipelineRequest: _backend_service_pb2.GrpcPipelineExecutionRequest
    pipelineMetadata: GrpcPipelineMetadata
    def __init__(self, subscriptionId: _Optional[_Union[_util_pb2.GrpcGuid, _Mapping]] = ..., pipelineRequest: _Optional[_Union[_backend_service_pb2.GrpcPipelineExecutionRequest, _Mapping]] = ..., pipelineMetadata: _Optional[_Union[GrpcPipelineMetadata, _Mapping]] = ...) -> None: ...

class GrpcPipelineMetadata(_message.Message):
    __slots__ = ["name"]
    NAME_FIELD_NUMBER: _ClassVar[int]
    name: str
    def __init__(self, name: _Optional[str] = ...) -> None: ...

class GrpcAddPipelineRequest(_message.Message):
    __slots__ = ["pipelineRequest", "producerKafkaMetadata"]
    PIPELINEREQUEST_FIELD_NUMBER: _ClassVar[int]
    PRODUCERKAFKAMETADATA_FIELD_NUMBER: _ClassVar[int]
    pipelineRequest: GrpcKafkaPipelineExecutionRequest
    producerKafkaMetadata: GrpcKafkaConnectionMetadata
    def __init__(self, pipelineRequest: _Optional[_Union[GrpcKafkaPipelineExecutionRequest, _Mapping]] = ..., producerKafkaMetadata: _Optional[_Union[GrpcKafkaConnectionMetadata, _Mapping]] = ...) -> None: ...

class GrpcAddPipelineStreamRequest(_message.Message):
    __slots__ = ["pipelineRequest"]
    PIPELINEREQUEST_FIELD_NUMBER: _ClassVar[int]
    pipelineRequest: GrpcKafkaPipelineExecutionRequest
    def __init__(self, pipelineRequest: _Optional[_Union[GrpcKafkaPipelineExecutionRequest, _Mapping]] = ...) -> None: ...

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

class GrpcGetAllSubscriptionsAndPipelinesResponse(_message.Message):
    __slots__ = ["subscriptions"]
    SUBSCRIPTIONS_FIELD_NUMBER: _ClassVar[int]
    subscriptions: _containers.RepeatedCompositeFieldContainer[GrpcKafkaSubscription]
    def __init__(self, subscriptions: _Optional[_Iterable[_Union[GrpcKafkaSubscription, _Mapping]]] = ...) -> None: ...

class GrpcKafkaSubscription(_message.Message):
    __slots__ = ["id", "metadata", "pipelines"]
    ID_FIELD_NUMBER: _ClassVar[int]
    METADATA_FIELD_NUMBER: _ClassVar[int]
    PIPELINES_FIELD_NUMBER: _ClassVar[int]
    id: _util_pb2.GrpcGuid
    metadata: GrpcKafkaSubscriptionMetadata
    pipelines: _containers.RepeatedCompositeFieldContainer[GrpcSubscriptionPipeline]
    def __init__(self, id: _Optional[_Union[_util_pb2.GrpcGuid, _Mapping]] = ..., metadata: _Optional[_Union[GrpcKafkaSubscriptionMetadata, _Mapping]] = ..., pipelines: _Optional[_Iterable[_Union[GrpcSubscriptionPipeline, _Mapping]]] = ...) -> None: ...

class GrpcSubscriptionPipeline(_message.Message):
    __slots__ = ["id", "metadata"]
    ID_FIELD_NUMBER: _ClassVar[int]
    METADATA_FIELD_NUMBER: _ClassVar[int]
    id: _util_pb2.GrpcGuid
    metadata: GrpcPipelineMetadata
    def __init__(self, id: _Optional[_Union[_util_pb2.GrpcGuid, _Mapping]] = ..., metadata: _Optional[_Union[GrpcPipelineMetadata, _Mapping]] = ...) -> None: ...
