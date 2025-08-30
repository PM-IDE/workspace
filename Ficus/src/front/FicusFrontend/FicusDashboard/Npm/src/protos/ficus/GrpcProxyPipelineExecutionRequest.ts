// Original file: ../../../../../protos/backend_service.proto

import type { GrpcPipeline_DONTUSE as _ficus_GrpcPipeline_DONTUSE, GrpcPipeline as _ficus_GrpcPipeline } from '../ficus/GrpcPipeline';
import type { GrpcGuid_DONTUSE as _ficus_GrpcGuid_DONTUSE, GrpcGuid as _ficus_GrpcGuid } from '../ficus/GrpcGuid';

export interface GrpcProxyPipelineExecutionRequest_DONTUSE {
  'pipeline'?: (_ficus_GrpcPipeline_DONTUSE | null);
  'contextValuesIds'?: (_ficus_GrpcGuid_DONTUSE)[];
}

export interface GrpcProxyPipelineExecutionRequest {
  'pipeline': (_ficus_GrpcPipeline | null);
  'contextValuesIds': (_ficus_GrpcGuid)[];
}
