// Original file: ../../../../../protos/pipelines_and_context.proto

import type { Long } from '@grpc/proto-loader';

export interface GrpcActivityStartEndData_DONTUSE {
  'startTime'?: (number | string | Long);
  'endTime'?: (number | string | Long);
}

export interface GrpcActivityStartEndData {
  'startTime': (number);
  'endTime': (number);
}
