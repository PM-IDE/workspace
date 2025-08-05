// Original file: /Users/aero/work/workspace/Ficus/protos/pipelines_and_context.proto

import type { GrpcColorsEventLogMapping_DONTUSE as _ficus_GrpcColorsEventLogMapping_DONTUSE, GrpcColorsEventLogMapping as _ficus_GrpcColorsEventLogMapping } from '../ficus/GrpcColorsEventLogMapping';
import type { GrpcColorsTrace_DONTUSE as _ficus_GrpcColorsTrace_DONTUSE, GrpcColorsTrace as _ficus_GrpcColorsTrace } from '../ficus/GrpcColorsTrace';
import type { GrpcColorsLogAdjustment_DONTUSE as _ficus_GrpcColorsLogAdjustment_DONTUSE, GrpcColorsLogAdjustment as _ficus_GrpcColorsLogAdjustment } from '../ficus/GrpcColorsLogAdjustment';

export interface GrpcColorsEventLog_DONTUSE {
  'mapping'?: (_ficus_GrpcColorsEventLogMapping_DONTUSE)[];
  'traces'?: (_ficus_GrpcColorsTrace_DONTUSE)[];
  'adjustments'?: (_ficus_GrpcColorsLogAdjustment_DONTUSE)[];
}

export interface GrpcColorsEventLog {
  'mapping': (_ficus_GrpcColorsEventLogMapping)[];
  'traces': (_ficus_GrpcColorsTrace)[];
  'adjustments': (_ficus_GrpcColorsLogAdjustment)[];
}
