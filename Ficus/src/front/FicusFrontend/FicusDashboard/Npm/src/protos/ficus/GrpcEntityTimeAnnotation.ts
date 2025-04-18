// Original file: /Users/aero/work/workspace/Ficus/protos/pm_models.proto

import type { GrpcTimeSpan_DONTUSE as _ficus_GrpcTimeSpan_DONTUSE, GrpcTimeSpan as _ficus_GrpcTimeSpan } from '../ficus/GrpcTimeSpan';
import type { Long } from '@grpc/proto-loader';

export interface GrpcEntityTimeAnnotation_DONTUSE {
  'entityId'?: (number | string | Long);
  'interval'?: (_ficus_GrpcTimeSpan_DONTUSE | null);
}

export interface GrpcEntityTimeAnnotation {
  'entityId': (number);
  'interval': (_ficus_GrpcTimeSpan | null);
}
