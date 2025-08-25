// Original file: ../../../../../protos/pm_models.proto

import type { GrpcSimpleTrace_DONTUSE as _ficus_GrpcSimpleTrace_DONTUSE, GrpcSimpleTrace as _ficus_GrpcSimpleTrace } from '../ficus/GrpcSimpleTrace';

export interface GrpcSimpleEventLog_DONTUSE {
  'traces'?: (_ficus_GrpcSimpleTrace_DONTUSE)[];
}

export interface GrpcSimpleEventLog {
  'traces': (_ficus_GrpcSimpleTrace)[];
}
