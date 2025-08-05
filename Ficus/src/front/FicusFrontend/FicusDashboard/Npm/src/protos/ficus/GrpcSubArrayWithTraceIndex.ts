// Original file: /Users/aero/work/workspace/Ficus/protos/pipelines_and_context.proto

import type { GrpcTraceSubArray_DONTUSE as _ficus_GrpcTraceSubArray_DONTUSE, GrpcTraceSubArray as _ficus_GrpcTraceSubArray } from '../ficus/GrpcTraceSubArray';

export interface GrpcSubArrayWithTraceIndex_DONTUSE {
  'subArray'?: (_ficus_GrpcTraceSubArray_DONTUSE | null);
  'traceIndex'?: (number);
}

export interface GrpcSubArrayWithTraceIndex {
  'subArray': (_ficus_GrpcTraceSubArray | null);
  'traceIndex': (number);
}
