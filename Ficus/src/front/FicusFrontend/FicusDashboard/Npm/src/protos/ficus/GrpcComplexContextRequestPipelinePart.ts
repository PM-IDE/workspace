// Original file: ../../../../../protos/pipelines_and_context.proto

import type { GrpcContextKey_DONTUSE as _ficus_GrpcContextKey_DONTUSE, GrpcContextKey as _ficus_GrpcContextKey } from '../ficus/GrpcContextKey';
import type { GrpcPipelinePart_DONTUSE as _ficus_GrpcPipelinePart_DONTUSE, GrpcPipelinePart as _ficus_GrpcPipelinePart } from '../ficus/GrpcPipelinePart';
import type { GrpcUuid_DONTUSE as _ficus_GrpcUuid_DONTUSE, GrpcUuid as _ficus_GrpcUuid } from '../ficus/GrpcUuid';

export interface GrpcComplexContextRequestPipelinePart_DONTUSE {
  'keys'?: (_ficus_GrpcContextKey_DONTUSE)[];
  'beforePipelinePart'?: (_ficus_GrpcPipelinePart_DONTUSE | null);
  'frontendPartUuid'?: (_ficus_GrpcUuid_DONTUSE | null);
  'frontendPipelinePartName'?: (string);
}

export interface GrpcComplexContextRequestPipelinePart {
  'keys': (_ficus_GrpcContextKey)[];
  'beforePipelinePart': (_ficus_GrpcPipelinePart | null);
  'frontendPartUuid': (_ficus_GrpcUuid | null);
  'frontendPipelinePartName': (string);
}
