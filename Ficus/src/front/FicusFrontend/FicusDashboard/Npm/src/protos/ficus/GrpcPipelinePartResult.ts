// Original file: ../../../../../protos/backend_service.proto

import type { GrpcContextValueWithKeyName_DONTUSE as _ficus_GrpcContextValueWithKeyName_DONTUSE, GrpcContextValueWithKeyName as _ficus_GrpcContextValueWithKeyName } from '../ficus/GrpcContextValueWithKeyName';
import type { GrpcGuid_DONTUSE as _ficus_GrpcGuid_DONTUSE, GrpcGuid as _ficus_GrpcGuid } from '../ficus/GrpcGuid';

export interface GrpcPipelinePartResult_DONTUSE {
  'contextValues'?: (_ficus_GrpcContextValueWithKeyName_DONTUSE)[];
  'guid'?: (_ficus_GrpcGuid_DONTUSE | null);
}

export interface GrpcPipelinePartResult {
  'contextValues': (_ficus_GrpcContextValueWithKeyName)[];
  'guid': (_ficus_GrpcGuid | null);
}
