// Original file: /Users/aero/work/workspace/Ficus/protos/pipelines_and_context.proto

import type { GrpcHistogramEntry_DONTUSE as _ficus_GrpcHistogramEntry_DONTUSE, GrpcHistogramEntry as _ficus_GrpcHistogramEntry } from '../ficus/GrpcHistogramEntry';

export interface GrpcGeneralHistogramData_DONTUSE {
  'name'?: (string);
  'entries'?: (_ficus_GrpcHistogramEntry_DONTUSE)[];
}

export interface GrpcGeneralHistogramData {
  'name': (string);
  'entries': (_ficus_GrpcHistogramEntry)[];
}
