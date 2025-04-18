// Original file: /Users/aero/work/workspace/Ficus/protos/pipelines_and_context.proto

import type { GrpcArrayPoolEventKind_DONTUSE as _ficus_GrpcArrayPoolEventKind_DONTUSE, GrpcArrayPoolEventKind as _ficus_GrpcArrayPoolEventKind } from '../ficus/GrpcArrayPoolEventKind';
import type { Long } from '@grpc/proto-loader';

export interface GrpcArrayPoolEvent_DONTUSE {
  'bufferId'?: (number | string | Long);
  'eventKind'?: (_ficus_GrpcArrayPoolEventKind_DONTUSE);
}

export interface GrpcArrayPoolEvent {
  'bufferId': (number);
  'eventKind': (_ficus_GrpcArrayPoolEventKind);
}
