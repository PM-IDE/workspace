// Original file: /Users/aero/work/workspace/Ficus/protos/kafka_service.proto

import type { GrpcGuid_DONTUSE as _ficus_GrpcGuid_DONTUSE, GrpcGuid as _ficus_GrpcGuid } from '../ficus/GrpcGuid';
import type { GrpcPipelineExecutionRequest_DONTUSE as _ficus_GrpcPipelineExecutionRequest_DONTUSE, GrpcPipelineExecutionRequest as _ficus_GrpcPipelineExecutionRequest } from '../ficus/GrpcPipelineExecutionRequest';
import type { GrpcPipelineMetadata_DONTUSE as _ficus_GrpcPipelineMetadata_DONTUSE, GrpcPipelineMetadata as _ficus_GrpcPipelineMetadata } from '../ficus/GrpcPipelineMetadata';
import type { GrpcPipelineStreamingConfiguration_DONTUSE as _ficus_GrpcPipelineStreamingConfiguration_DONTUSE, GrpcPipelineStreamingConfiguration as _ficus_GrpcPipelineStreamingConfiguration } from '../ficus/GrpcPipelineStreamingConfiguration';

export interface GrpcKafkaPipelineExecutionRequest_DONTUSE {
  'subscriptionId'?: (_ficus_GrpcGuid_DONTUSE | null);
  'pipelineRequest'?: (_ficus_GrpcPipelineExecutionRequest_DONTUSE | null);
  'pipelineMetadata'?: (_ficus_GrpcPipelineMetadata_DONTUSE | null);
  'streamingConfiguration'?: (_ficus_GrpcPipelineStreamingConfiguration_DONTUSE | null);
}

export interface GrpcKafkaPipelineExecutionRequest {
  'subscriptionId': (_ficus_GrpcGuid | null);
  'pipelineRequest': (_ficus_GrpcPipelineExecutionRequest | null);
  'pipelineMetadata': (_ficus_GrpcPipelineMetadata | null);
  'streamingConfiguration': (_ficus_GrpcPipelineStreamingConfiguration | null);
}
