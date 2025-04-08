// Original file: /Users/aero/work/workspace/Ficus/protos/backend_service.proto

import type { GrpcContextValue_DONTUSE as _ficus_GrpcContextValue_DONTUSE, GrpcContextValue as _ficus_GrpcContextValue } from '../ficus/GrpcContextValue';

export interface GrpcGetContextValueResult_DONTUSE {
  'value'?: (_ficus_GrpcContextValue_DONTUSE | null);
  'error'?: (string);
  'contextValueResult'?: "value"|"error";
}

export interface GrpcGetContextValueResult {
  'value'?: (_ficus_GrpcContextValue | null);
  'error'?: (string);
  'contextValueResult': "value"|"error";
}
