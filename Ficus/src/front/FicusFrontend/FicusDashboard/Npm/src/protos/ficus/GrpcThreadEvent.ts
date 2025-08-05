// Original file: /Users/aero/work/workspace/Ficus/protos/pipelines_and_context.proto

import type { Long } from '@grpc/proto-loader';

export interface GrpcThreadEvent_DONTUSE {
  'name'?: (string);
  'stamp'?: (number | string | Long);
}

export interface GrpcThreadEvent {
  'name': (string);
  'stamp': (number);
}
