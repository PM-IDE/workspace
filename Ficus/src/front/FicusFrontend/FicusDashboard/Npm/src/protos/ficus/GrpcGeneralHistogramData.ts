// Original file: /Users/aero/work/workspace/Ficus/protos/pipelines_and_context.proto

import type { GrpcGeneralHistogramEntry_DONTUSE as _ficus_GrpcGeneralHistogramEntry_DONTUSE, GrpcGeneralHistogramEntry as _ficus_GrpcGeneralHistogramEntry } from '../ficus/GrpcGeneralHistogramEntry';

export interface GrpcGeneralHistogramData_DONTUSE {
  'name'?: (string);
  'entries'?: (_ficus_GrpcGeneralHistogramEntry_DONTUSE)[];
}

export interface GrpcGeneralHistogramData {
  'name': (string);
  'entries': (_ficus_GrpcGeneralHistogramEntry)[];
}
