// Original file: /Users/aero/work/workspace/Ficus/protos/pipelines_and_context.proto

import type { Long } from '@grpc/proto-loader';

export interface GrpcExecutionSuspensionInfo_DONTUSE {
  'reason'?: (string);
  'startTime'?: (number | string | Long);
  'endTime'?: (number | string | Long);
}

export interface GrpcExecutionSuspensionInfo {
  'reason': (string);
  'startTime': (number);
  'endTime': (number);
}
