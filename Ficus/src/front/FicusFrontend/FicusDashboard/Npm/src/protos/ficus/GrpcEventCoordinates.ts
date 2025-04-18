// Original file: /Users/aero/work/workspace/Ficus/protos/pipelines_and_context.proto

import type { Long } from '@grpc/proto-loader';

export interface GrpcEventCoordinates_DONTUSE {
  'traceId'?: (number | string | Long);
  'eventIndex'?: (number | string | Long);
}

export interface GrpcEventCoordinates {
  'traceId': (number);
  'eventIndex': (number);
}
