// Original file: /Users/aero/work/workspace/Ficus/protos/pipelines_and_context.proto

import type { GrpcMethodNameParts_DONTUSE as _ficus_GrpcMethodNameParts_DONTUSE, GrpcMethodNameParts as _ficus_GrpcMethodNameParts } from '../ficus/GrpcMethodNameParts';
import type { Empty_DONTUSE as _google_protobuf_Empty_DONTUSE, Empty as _google_protobuf_Empty } from '../google/protobuf/Empty';

export interface GrpcMethodLoadUnloadEvent_DONTUSE {
  'methodNameParts'?: (_ficus_GrpcMethodNameParts_DONTUSE | null);
  'load'?: (_google_protobuf_Empty_DONTUSE | null);
  'unload'?: (_google_protobuf_Empty_DONTUSE | null);
  'event'?: "load"|"unload";
}

export interface GrpcMethodLoadUnloadEvent {
  'methodNameParts': (_ficus_GrpcMethodNameParts | null);
  'load'?: (_google_protobuf_Empty | null);
  'unload'?: (_google_protobuf_Empty | null);
  'event': "load"|"unload";
}
