// Original file: ../../../../../protos/front_contract.proto

import type { GrpcGuid_DONTUSE as _ficus_GrpcGuid_DONTUSE, GrpcGuid as _ficus_GrpcGuid } from '../ficus/GrpcGuid';
import type { GrpcCaseName_DONTUSE as _ficus_GrpcCaseName_DONTUSE, GrpcCaseName as _ficus_GrpcCaseName } from '../ficus/GrpcCaseName';

export interface GrpcGetPipelineCaseContextValuesRequest_DONTUSE {
  'subscriptionId'?: (_ficus_GrpcGuid_DONTUSE | null);
  'pipelineId'?: (_ficus_GrpcGuid_DONTUSE | null);
  'processName'?: (string);
  'caseName'?: (_ficus_GrpcCaseName_DONTUSE | null);
}

export interface GrpcGetPipelineCaseContextValuesRequest {
  'subscriptionId': (_ficus_GrpcGuid | null);
  'pipelineId': (_ficus_GrpcGuid | null);
  'processName': (string);
  'caseName': (_ficus_GrpcCaseName | null);
}
