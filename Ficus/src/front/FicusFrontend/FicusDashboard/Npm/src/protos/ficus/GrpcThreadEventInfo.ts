// Original file: /Users/aero/work/workspace/Ficus/protos/pipelines_and_context.proto

import type { GrpcThreadEventKind_DONTUSE as _ficus_GrpcThreadEventKind_DONTUSE, GrpcThreadEventKind as _ficus_GrpcThreadEventKind } from '../ficus/GrpcThreadEventKind';
import type { Long } from '@grpc/proto-loader';

export interface GrpcThreadEventInfo_DONTUSE {
  'threadId'?: (number | string | Long);
  'eventKind'?: (_ficus_GrpcThreadEventKind_DONTUSE);
}

export interface GrpcThreadEventInfo {
  'threadId': (number);
  'eventKind': (_ficus_GrpcThreadEventKind);
}
