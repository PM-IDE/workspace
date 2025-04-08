// Original file: /Users/aero/work/workspace/Ficus/protos/backend_service.proto

import type { GrpcContextValueWithKeyName_DONTUSE as _ficus_GrpcContextValueWithKeyName_DONTUSE, GrpcContextValueWithKeyName as _ficus_GrpcContextValueWithKeyName } from '../ficus/GrpcContextValueWithKeyName';
import type { GrpcUuid_DONTUSE as _ficus_GrpcUuid_DONTUSE, GrpcUuid as _ficus_GrpcUuid } from '../ficus/GrpcUuid';

export interface GrpcPipelinePartResult_DONTUSE {
  'contextValues'?: (_ficus_GrpcContextValueWithKeyName_DONTUSE)[];
  'uuid'?: (_ficus_GrpcUuid_DONTUSE | null);
}

export interface GrpcPipelinePartResult {
  'contextValues': (_ficus_GrpcContextValueWithKeyName)[];
  'uuid': (_ficus_GrpcUuid | null);
}
