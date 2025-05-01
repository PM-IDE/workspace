// Original file: /Users/aero/work/workspace/Ficus/protos/pipelines_and_context.proto

import type { GrpcMethodNameParts_DONTUSE as _ficus_GrpcMethodNameParts_DONTUSE, GrpcMethodNameParts as _ficus_GrpcMethodNameParts } from '../ficus/GrpcMethodNameParts';
import type { GrpcMethodLoadUnloadEventKind_DONTUSE as _ficus_GrpcMethodLoadUnloadEventKind_DONTUSE, GrpcMethodLoadUnloadEventKind as _ficus_GrpcMethodLoadUnloadEventKind } from '../ficus/GrpcMethodLoadUnloadEventKind';

export interface GrpcMethodLoadUnloadEvent_DONTUSE {
  'methodNameParts'?: (_ficus_GrpcMethodNameParts_DONTUSE | null);
  'eventKind'?: (_ficus_GrpcMethodLoadUnloadEventKind_DONTUSE);
}

export interface GrpcMethodLoadUnloadEvent {
  'methodNameParts': (_ficus_GrpcMethodNameParts | null);
  'eventKind': (_ficus_GrpcMethodLoadUnloadEventKind);
}
