// Original file: /Users/aero/work/workspace/Ficus/protos/pipelines_and_context.proto

import type { GrpcUnderlyingPatternKind_DONTUSE as _ficus_GrpcUnderlyingPatternKind_DONTUSE, GrpcUnderlyingPatternKind as _ficus_GrpcUnderlyingPatternKind } from '../ficus/GrpcUnderlyingPatternKind';
import type { GrpcGraph_DONTUSE as _ficus_GrpcGraph_DONTUSE, GrpcGraph as _ficus_GrpcGraph } from '../ficus/GrpcGraph';

export interface GrpcUnderlyingPatternInfo_DONTUSE {
  'patternKind'?: (_ficus_GrpcUnderlyingPatternKind_DONTUSE);
  'baseSequence'?: (string)[];
  'graph'?: (_ficus_GrpcGraph_DONTUSE | null);
}

export interface GrpcUnderlyingPatternInfo {
  'patternKind': (_ficus_GrpcUnderlyingPatternKind);
  'baseSequence': (string)[];
  'graph': (_ficus_GrpcGraph | null);
}
