// Original file: /Users/aero/work/workspace/Ficus/protos/pipelines_and_context.proto

import type { Long } from '@grpc/proto-loader';

export interface GrpcHistogramEntry_DONTUSE {
  'name'?: (string);
  'count'?: (number | string | Long);
}

export interface GrpcHistogramEntry {
  'name': (string);
  'count': (number);
}
