// Original file: /Users/aero/work/workspace/Ficus/protos/backend_service.proto

import type { GrpcContextKey_DONTUSE as _ficus_GrpcContextKey_DONTUSE, GrpcContextKey as _ficus_GrpcContextKey } from '../ficus/GrpcContextKey';
import type { GrpcGuid_DONTUSE as _ficus_GrpcGuid_DONTUSE, GrpcGuid as _ficus_GrpcGuid } from '../ficus/GrpcGuid';

export interface GrpcGetContextValueRequest_DONTUSE {
  'key'?: (_ficus_GrpcContextKey_DONTUSE | null);
  'executionId'?: (_ficus_GrpcGuid_DONTUSE | null);
}

export interface GrpcGetContextValueRequest {
  'key': (_ficus_GrpcContextKey | null);
  'executionId': (_ficus_GrpcGuid | null);
}
