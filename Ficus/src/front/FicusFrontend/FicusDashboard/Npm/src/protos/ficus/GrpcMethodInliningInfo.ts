// Original file: /Users/aero/work/workspace/Ficus/protos/pipelines_and_context.proto

import type { GrpcMethodNameParts_DONTUSE as _ficus_GrpcMethodNameParts_DONTUSE, GrpcMethodNameParts as _ficus_GrpcMethodNameParts } from '../ficus/GrpcMethodNameParts';

export interface GrpcMethodInliningInfo_DONTUSE {
  'inlineeInfo'?: (_ficus_GrpcMethodNameParts_DONTUSE | null);
  'inlinerInfo'?: (_ficus_GrpcMethodNameParts_DONTUSE | null);
}

export interface GrpcMethodInliningInfo {
  'inlineeInfo': (_ficus_GrpcMethodNameParts | null);
  'inlinerInfo': (_ficus_GrpcMethodNameParts | null);
}
