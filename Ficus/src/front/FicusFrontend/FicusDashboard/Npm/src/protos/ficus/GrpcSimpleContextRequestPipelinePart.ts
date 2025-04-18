// Original file: /Users/aero/work/workspace/Ficus/protos/pipelines_and_context.proto

import type { GrpcContextKey_DONTUSE as _ficus_GrpcContextKey_DONTUSE, GrpcContextKey as _ficus_GrpcContextKey } from '../ficus/GrpcContextKey';
import type { GrpcUuid_DONTUSE as _ficus_GrpcUuid_DONTUSE, GrpcUuid as _ficus_GrpcUuid } from '../ficus/GrpcUuid';

export interface GrpcSimpleContextRequestPipelinePart_DONTUSE {
  'key'?: (_ficus_GrpcContextKey_DONTUSE | null);
  'frontendPartUuid'?: (_ficus_GrpcUuid_DONTUSE | null);
  'frontendPipelinePartName'?: (string);
}

export interface GrpcSimpleContextRequestPipelinePart {
  'key': (_ficus_GrpcContextKey | null);
  'frontendPartUuid': (_ficus_GrpcUuid | null);
  'frontendPipelinePartName': (string);
}
