// Original file: ../../../../../protos/pipelines_and_context.proto

import type { GrpcColorsLogRectangleAdjustment_DONTUSE as _ficus_GrpcColorsLogRectangleAdjustment_DONTUSE, GrpcColorsLogRectangleAdjustment as _ficus_GrpcColorsLogRectangleAdjustment } from '../ficus/GrpcColorsLogRectangleAdjustment';
import type { GrpcColorsLogXAxisAfterTraceAdjustment_DONTUSE as _ficus_GrpcColorsLogXAxisAfterTraceAdjustment_DONTUSE, GrpcColorsLogXAxisAfterTraceAdjustment as _ficus_GrpcColorsLogXAxisAfterTraceAdjustment } from '../ficus/GrpcColorsLogXAxisAfterTraceAdjustment';

export interface GrpcColorsLogAdjustment_DONTUSE {
  'rectangleAdjustment'?: (_ficus_GrpcColorsLogRectangleAdjustment_DONTUSE | null);
  'axisAfterTrace'?: (_ficus_GrpcColorsLogXAxisAfterTraceAdjustment_DONTUSE | null);
  'selection'?: "rectangleAdjustment"|"axisAfterTrace";
}

export interface GrpcColorsLogAdjustment {
  'rectangleAdjustment'?: (_ficus_GrpcColorsLogRectangleAdjustment | null);
  'axisAfterTrace'?: (_ficus_GrpcColorsLogXAxisAfterTraceAdjustment | null);
  'selection': "rectangleAdjustment"|"axisAfterTrace";
}
