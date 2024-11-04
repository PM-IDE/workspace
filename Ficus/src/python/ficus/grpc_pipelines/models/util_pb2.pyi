from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from typing import ClassVar as _ClassVar, Optional as _Optional

DESCRIPTOR: _descriptor.FileDescriptor

class GrpcGuid(_message.Message):
    __slots__ = ["guid"]
    GUID_FIELD_NUMBER: _ClassVar[int]
    guid: str
    def __init__(self, guid: _Optional[str] = ...) -> None: ...

class GrpcColor(_message.Message):
    __slots__ = ["red", "green", "blue"]
    RED_FIELD_NUMBER: _ClassVar[int]
    GREEN_FIELD_NUMBER: _ClassVar[int]
    BLUE_FIELD_NUMBER: _ClassVar[int]
    red: int
    green: int
    blue: int
    def __init__(self, red: _Optional[int] = ..., green: _Optional[int] = ..., blue: _Optional[int] = ...) -> None: ...

class GrpcUuid(_message.Message):
    __slots__ = ["uuid"]
    UUID_FIELD_NUMBER: _ClassVar[int]
    uuid: str
    def __init__(self, uuid: _Optional[str] = ...) -> None: ...

class GrpcStringKeyValue(_message.Message):
    __slots__ = ["key", "value"]
    KEY_FIELD_NUMBER: _ClassVar[int]
    VALUE_FIELD_NUMBER: _ClassVar[int]
    key: str
    value: str
    def __init__(self, key: _Optional[str] = ..., value: _Optional[str] = ...) -> None: ...

class GrpcTimeSpan(_message.Message):
    __slots__ = ["nanoseconds"]
    NANOSECONDS_FIELD_NUMBER: _ClassVar[int]
    nanoseconds: int
    def __init__(self, nanoseconds: _Optional[int] = ...) -> None: ...

class GrpcDateTime(_message.Message):
    __slots__ = ["nanosSinceUnixEpoch"]
    NANOSSINCEUNIXEPOCH_FIELD_NUMBER: _ClassVar[int]
    nanosSinceUnixEpoch: int
    def __init__(self, nanosSinceUnixEpoch: _Optional[int] = ...) -> None: ...
