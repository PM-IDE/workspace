// Original file: ../../../../../protos/kafka_service.proto

import type { GrpcGuid_DONTUSE as _ficus_GrpcGuid_DONTUSE, GrpcGuid as _ficus_GrpcGuid } from '../ficus/GrpcGuid';

export interface GrpcGetCurrentContextValuesRequest_DONTUSE {
  'subscriptionId'?: (_ficus_GrpcGuid_DONTUSE | null);
  'pipelineId'?: (_ficus_GrpcGuid_DONTUSE | null);
  'caseName'?: (string);
}

export interface GrpcGetCurrentContextValuesRequest {
  'subscriptionId': (_ficus_GrpcGuid | null);
  'pipelineId': (_ficus_GrpcGuid | null);
  'caseName': (string);
}
