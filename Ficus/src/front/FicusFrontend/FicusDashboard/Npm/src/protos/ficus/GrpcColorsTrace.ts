// Original file: /Users/aero/work/workspace/Ficus/protos/pipelines_and_context.proto

import type { GrpcColoredRectangle_DONTUSE as _ficus_GrpcColoredRectangle_DONTUSE, GrpcColoredRectangle as _ficus_GrpcColoredRectangle } from '../ficus/GrpcColoredRectangle';

export interface GrpcColorsTrace_DONTUSE {
  'eventColors'?: (_ficus_GrpcColoredRectangle_DONTUSE)[];
  'constantWidth'?: (boolean);
}

export interface GrpcColorsTrace {
  'eventColors': (_ficus_GrpcColoredRectangle)[];
  'constantWidth': (boolean);
}
