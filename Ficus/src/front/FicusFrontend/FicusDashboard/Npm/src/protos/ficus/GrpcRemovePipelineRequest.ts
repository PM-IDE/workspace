// Original file: ../../../../../protos/kafka_service.proto

import type { GrpcGuid_DONTUSE as _ficus_GrpcGuid_DONTUSE, GrpcGuid as _ficus_GrpcGuid } from '../ficus/GrpcGuid';

export interface GrpcRemovePipelineRequest_DONTUSE {
  'subscriptionId'?: (_ficus_GrpcGuid_DONTUSE | null);
  'pipelineId'?: (_ficus_GrpcGuid_DONTUSE | null);
}

export interface GrpcRemovePipelineRequest {
  'subscriptionId': (_ficus_GrpcGuid | null);
  'pipelineId': (_ficus_GrpcGuid | null);
}
