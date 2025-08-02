// Original file: ../../../../../protos/pipelines_and_context.proto

import type { Long } from '@grpc/proto-loader';

export interface GrpcContentionEvent_DONTUSE {
  'startTime'?: (number | string | Long);
  'endTime'?: (number | string | Long);
}

export interface GrpcContentionEvent {
  'startTime': (number);
  'endTime': (number);
}
