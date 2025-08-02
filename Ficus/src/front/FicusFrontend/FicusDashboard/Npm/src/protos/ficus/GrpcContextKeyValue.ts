// Original file: ../../../../../protos/pipelines_and_context.proto

import type { GrpcContextKey_DONTUSE as _ficus_GrpcContextKey_DONTUSE, GrpcContextKey as _ficus_GrpcContextKey } from '../ficus/GrpcContextKey';
import type { GrpcContextValue_DONTUSE as _ficus_GrpcContextValue_DONTUSE, GrpcContextValue as _ficus_GrpcContextValue } from '../ficus/GrpcContextValue';

export interface GrpcContextKeyValue_DONTUSE {
  'key'?: (_ficus_GrpcContextKey_DONTUSE | null);
  'value'?: (_ficus_GrpcContextValue_DONTUSE | null);
}

export interface GrpcContextKeyValue {
  'key': (_ficus_GrpcContextKey | null);
  'value': (_ficus_GrpcContextValue | null);
}
