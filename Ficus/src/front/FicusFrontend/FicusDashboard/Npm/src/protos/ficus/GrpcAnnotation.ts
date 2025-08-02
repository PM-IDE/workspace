// Original file: ../../../../../protos/pm_models.proto

import type { GrpcCountAnnotation_DONTUSE as _ficus_GrpcCountAnnotation_DONTUSE, GrpcCountAnnotation as _ficus_GrpcCountAnnotation } from '../ficus/GrpcCountAnnotation';
import type { GrpcFrequenciesAnnotation_DONTUSE as _ficus_GrpcFrequenciesAnnotation_DONTUSE, GrpcFrequenciesAnnotation as _ficus_GrpcFrequenciesAnnotation } from '../ficus/GrpcFrequenciesAnnotation';
import type { GrpcTimePerformanceAnnotation_DONTUSE as _ficus_GrpcTimePerformanceAnnotation_DONTUSE, GrpcTimePerformanceAnnotation as _ficus_GrpcTimePerformanceAnnotation } from '../ficus/GrpcTimePerformanceAnnotation';

export interface GrpcAnnotation_DONTUSE {
  'countAnnotation'?: (_ficus_GrpcCountAnnotation_DONTUSE | null);
  'frequencyAnnotation'?: (_ficus_GrpcFrequenciesAnnotation_DONTUSE | null);
  'timeAnnotation'?: (_ficus_GrpcTimePerformanceAnnotation_DONTUSE | null);
  'annotation'?: "countAnnotation"|"frequencyAnnotation"|"timeAnnotation";
}

export interface GrpcAnnotation {
  'countAnnotation'?: (_ficus_GrpcCountAnnotation | null);
  'frequencyAnnotation'?: (_ficus_GrpcFrequenciesAnnotation | null);
  'timeAnnotation'?: (_ficus_GrpcTimePerformanceAnnotation | null);
  'annotation': "countAnnotation"|"frequencyAnnotation"|"timeAnnotation";
}
