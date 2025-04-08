// Original file: /Users/aero/work/workspace/Ficus/protos/front_contract.proto

import type { GrpcGuid_DONTUSE as _ficus_GrpcGuid_DONTUSE, GrpcGuid as _ficus_GrpcGuid } from '../ficus/GrpcGuid';

export interface GrpcPipelinePartInfo_DONTUSE {
  'name'?: (string);
  'id'?: (_ficus_GrpcGuid_DONTUSE | null);
  'executionId'?: (_ficus_GrpcGuid_DONTUSE | null);
}

export interface GrpcPipelinePartInfo {
  'name': (string);
  'id': (_ficus_GrpcGuid | null);
  'executionId': (_ficus_GrpcGuid | null);
}
