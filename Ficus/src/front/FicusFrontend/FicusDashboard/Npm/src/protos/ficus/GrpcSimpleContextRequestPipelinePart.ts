// Original file: ../../../../../protos/pipelines_and_context.proto

import type { GrpcContextKey_DONTUSE as _ficus_GrpcContextKey_DONTUSE, GrpcContextKey as _ficus_GrpcContextKey } from '../ficus/GrpcContextKey';
import type { GrpcGuid_DONTUSE as _ficus_GrpcGuid_DONTUSE, GrpcGuid as _ficus_GrpcGuid } from '../ficus/GrpcGuid';

export interface GrpcSimpleContextRequestPipelinePart_DONTUSE {
  'key'?: (_ficus_GrpcContextKey_DONTUSE | null);
  'frontendPartUuid'?: (_ficus_GrpcGuid_DONTUSE | null);
  'frontendPipelinePartName'?: (string);
}

export interface GrpcSimpleContextRequestPipelinePart {
  'key': (_ficus_GrpcContextKey | null);
  'frontendPartUuid': (_ficus_GrpcGuid | null);
  'frontendPipelinePartName': (string);
}
