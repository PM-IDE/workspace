// Original file: ../../../../../protos/pm_models.proto

import type { GrpcEventStamp_DONTUSE as _ficus_GrpcEventStamp_DONTUSE, GrpcEventStamp as _ficus_GrpcEventStamp } from '../ficus/GrpcEventStamp';

export interface GrpcEvent_DONTUSE {
  'name'?: (string);
  'stamp'?: (_ficus_GrpcEventStamp_DONTUSE | null);
}

export interface GrpcEvent {
  'name': (string);
  'stamp': (_ficus_GrpcEventStamp | null);
}
