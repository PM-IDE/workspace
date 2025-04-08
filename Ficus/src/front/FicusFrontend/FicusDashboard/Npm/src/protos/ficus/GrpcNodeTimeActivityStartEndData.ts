// Original file: /Users/aero/work/workspace/Ficus/protos/pipelines_and_context.proto

import type { Long } from '@grpc/proto-loader';

export interface GrpcNodeTimeActivityStartEndData_DONTUSE {
  'startTime'?: (number | string | Long);
  'endTime'?: (number | string | Long);
}

export interface GrpcNodeTimeActivityStartEndData {
  'startTime': (number);
  'endTime': (number);
}
