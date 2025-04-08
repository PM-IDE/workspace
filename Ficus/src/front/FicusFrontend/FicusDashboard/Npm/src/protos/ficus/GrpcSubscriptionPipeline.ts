// Original file: /Users/aero/work/workspace/Ficus/protos/kafka_service.proto

import type { GrpcGuid_DONTUSE as _ficus_GrpcGuid_DONTUSE, GrpcGuid as _ficus_GrpcGuid } from '../ficus/GrpcGuid';
import type { GrpcPipelineMetadata_DONTUSE as _ficus_GrpcPipelineMetadata_DONTUSE, GrpcPipelineMetadata as _ficus_GrpcPipelineMetadata } from '../ficus/GrpcPipelineMetadata';

export interface GrpcSubscriptionPipeline_DONTUSE {
  'id'?: (_ficus_GrpcGuid_DONTUSE | null);
  'metadata'?: (_ficus_GrpcPipelineMetadata_DONTUSE | null);
}

export interface GrpcSubscriptionPipeline {
  'id': (_ficus_GrpcGuid | null);
  'metadata': (_ficus_GrpcPipelineMetadata | null);
}
