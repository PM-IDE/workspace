// Original file: /Users/aero/work/workspace/Ficus/protos/pm_models.proto

import type { GrpcHashesLogTrace_DONTUSE as _ficus_GrpcHashesLogTrace_DONTUSE, GrpcHashesLogTrace as _ficus_GrpcHashesLogTrace } from '../ficus/GrpcHashesLogTrace';

export interface GrpcHashesEventLog_DONTUSE {
  'traces'?: (_ficus_GrpcHashesLogTrace_DONTUSE)[];
}

export interface GrpcHashesEventLog {
  'traces': (_ficus_GrpcHashesLogTrace)[];
}
