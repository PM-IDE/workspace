// Original file: /Users/aero/work/workspace/Ficus/protos/pipelines_and_context.proto

import type { GrpcPipelinePartConfiguration_DONTUSE as _ficus_GrpcPipelinePartConfiguration_DONTUSE, GrpcPipelinePartConfiguration as _ficus_GrpcPipelinePartConfiguration } from '../ficus/GrpcPipelinePartConfiguration';

export interface GrpcPipelinePart_DONTUSE {
  'name'?: (string);
  'configuration'?: (_ficus_GrpcPipelinePartConfiguration_DONTUSE | null);
}

export interface GrpcPipelinePart {
  'name': (string);
  'configuration': (_ficus_GrpcPipelinePartConfiguration | null);
}
