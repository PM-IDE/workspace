// Original file: /Users/aero/work/workspace/Ficus/protos/pipelines_and_context.proto

import type { GrpcLogPoint_DONTUSE as _ficus_GrpcLogPoint_DONTUSE, GrpcLogPoint as _ficus_GrpcLogPoint } from '../ficus/GrpcLogPoint';

export interface GrpcColorsLogRectangleAdjustment_DONTUSE {
  'upLeftPoint'?: (_ficus_GrpcLogPoint_DONTUSE | null);
  'downRightPoint'?: (_ficus_GrpcLogPoint_DONTUSE | null);
  'extendToNearestVerticalBorders'?: (boolean);
}

export interface GrpcColorsLogRectangleAdjustment {
  'upLeftPoint': (_ficus_GrpcLogPoint | null);
  'downRightPoint': (_ficus_GrpcLogPoint | null);
  'extendToNearestVerticalBorders': (boolean);
}
