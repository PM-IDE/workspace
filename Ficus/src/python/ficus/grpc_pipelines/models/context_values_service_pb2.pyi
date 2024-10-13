import pipelines_and_context_pb2 as _pipelines_and_context_pb2
import util_pb2 as _util_pb2
from google.protobuf import empty_pb2 as _empty_pb2
from google.protobuf.internal import containers as _containers
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from typing import ClassVar as _ClassVar, Iterable as _Iterable, Mapping as _Mapping, Optional as _Optional, Union as _Union

DESCRIPTOR: _descriptor.FileDescriptor

class GrpcContextValuePart(_message.Message):
    __slots__ = ["key", "bytes"]
    KEY_FIELD_NUMBER: _ClassVar[int]
    BYTES_FIELD_NUMBER: _ClassVar[int]
    key: str
    bytes: bytes
    def __init__(self, key: _Optional[str] = ..., bytes: _Optional[bytes] = ...) -> None: ...

class GrpcDropContextValuesRequest(_message.Message):
    __slots__ = ["ids"]
    IDS_FIELD_NUMBER: _ClassVar[int]
    ids: _containers.RepeatedCompositeFieldContainer[_util_pb2.GrpcGuid]
    def __init__(self, ids: _Optional[_Iterable[_Union[_util_pb2.GrpcGuid, _Mapping]]] = ...) -> None: ...
