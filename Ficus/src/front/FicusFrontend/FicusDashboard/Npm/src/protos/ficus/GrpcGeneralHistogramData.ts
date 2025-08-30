// Original file: ../../../../../protos/pipelines_and_context.proto

import type { GrpcGenericEnhancementBase_DONTUSE as _ficus_GrpcGenericEnhancementBase_DONTUSE, GrpcGenericEnhancementBase as _ficus_GrpcGenericEnhancementBase } from '../ficus/GrpcGenericEnhancementBase';
import type { GrpcHistogramEntry_DONTUSE as _ficus_GrpcHistogramEntry_DONTUSE, GrpcHistogramEntry as _ficus_GrpcHistogramEntry } from '../ficus/GrpcHistogramEntry';

export interface GrpcGeneralHistogramData_DONTUSE {
  'base'?: (_ficus_GrpcGenericEnhancementBase_DONTUSE | null);
  'entries'?: (_ficus_GrpcHistogramEntry_DONTUSE)[];
}

export interface GrpcGeneralHistogramData {
  'base': (_ficus_GrpcGenericEnhancementBase | null);
  'entries': (_ficus_GrpcHistogramEntry)[];
}
