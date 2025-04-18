// Original file: /Users/aero/work/workspace/Ficus/protos/front_contract.proto

import type { GrpcCaseName_DONTUSE as _ficus_GrpcCaseName_DONTUSE, GrpcCaseName as _ficus_GrpcCaseName } from '../ficus/GrpcCaseName';
import type { GrpcGuid_DONTUSE as _ficus_GrpcGuid_DONTUSE, GrpcGuid as _ficus_GrpcGuid } from '../ficus/GrpcGuid';
import type { GrpcStringKeyValue_DONTUSE as _ficus_GrpcStringKeyValue_DONTUSE, GrpcStringKeyValue as _ficus_GrpcStringKeyValue } from '../ficus/GrpcStringKeyValue';

export interface GrpcProcessCaseMetadata_DONTUSE {
  'processName'?: (string);
  'caseName'?: (_ficus_GrpcCaseName_DONTUSE | null);
  'subscriptionId'?: (_ficus_GrpcGuid_DONTUSE | null);
  'subscriptionName'?: (string);
  'pipelineId'?: (_ficus_GrpcGuid_DONTUSE | null);
  'pipelineName'?: (string);
  'metadata'?: (_ficus_GrpcStringKeyValue_DONTUSE)[];
}

export interface GrpcProcessCaseMetadata {
  'processName': (string);
  'caseName': (_ficus_GrpcCaseName | null);
  'subscriptionId': (_ficus_GrpcGuid | null);
  'subscriptionName': (string);
  'pipelineId': (_ficus_GrpcGuid | null);
  'pipelineName': (string);
  'metadata': (_ficus_GrpcStringKeyValue)[];
}
