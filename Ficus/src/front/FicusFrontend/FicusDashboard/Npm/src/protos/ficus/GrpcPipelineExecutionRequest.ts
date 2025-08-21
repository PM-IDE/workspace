// Original file: ../../../../../protos/backend_service.proto

import type { GrpcPipeline_DONTUSE as _ficus_GrpcPipeline_DONTUSE, GrpcPipeline as _ficus_GrpcPipeline } from '../ficus/GrpcPipeline';
import type { GrpcContextKeyValue_DONTUSE as _ficus_GrpcContextKeyValue_DONTUSE, GrpcContextKeyValue as _ficus_GrpcContextKeyValue } from '../ficus/GrpcContextKeyValue';

export interface GrpcPipelineExecutionRequest_DONTUSE {
  'pipeline'?: (_ficus_GrpcPipeline_DONTUSE | null);
  'initialContext'?: (_ficus_GrpcContextKeyValue_DONTUSE)[];
}

export interface GrpcPipelineExecutionRequest {
  'pipeline': (_ficus_GrpcPipeline | null);
  'initialContext': (_ficus_GrpcContextKeyValue)[];
}
