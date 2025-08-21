// Original file: ../../../../../protos/pipelines_and_context.proto

import type { GrpcLogPoint_DONTUSE as _ficus_GrpcLogPoint_DONTUSE, GrpcLogPoint as _ficus_GrpcLogPoint } from '../ficus/GrpcLogPoint';

export interface GrpcTimelineTraceEventsGroup_DONTUSE {
  'startPoint'?: (_ficus_GrpcLogPoint_DONTUSE | null);
  'endPoint'?: (_ficus_GrpcLogPoint_DONTUSE | null);
}

export interface GrpcTimelineTraceEventsGroup {
  'startPoint': (_ficus_GrpcLogPoint | null);
  'endPoint': (_ficus_GrpcLogPoint | null);
}
