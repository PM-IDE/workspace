// Original file: /Users/aero/work/workspace/Ficus/protos/pipelines_and_context.proto

import type { GrpcGenericEnhancementBase_DONTUSE as _ficus_GrpcGenericEnhancementBase_DONTUSE, GrpcGenericEnhancementBase as _ficus_GrpcGenericEnhancementBase } from '../ficus/GrpcGenericEnhancementBase';
import type { GrpcDurationKind_DONTUSE as _ficus_GrpcDurationKind_DONTUSE, GrpcDurationKind as _ficus_GrpcDurationKind } from '../ficus/GrpcDurationKind';
import type { Long } from '@grpc/proto-loader';

export interface GrpcActivityDurationData_DONTUSE {
  'base'?: (_ficus_GrpcGenericEnhancementBase_DONTUSE | null);
  'duration'?: (number | string | Long);
  'kind'?: (_ficus_GrpcDurationKind_DONTUSE);
}

export interface GrpcActivityDurationData {
  'base': (_ficus_GrpcGenericEnhancementBase | null);
  'duration': (number);
  'kind': (_ficus_GrpcDurationKind);
}
