// Original file: /Users/aero/work/workspace/Ficus/protos/pipelines_and_context.proto

import type { Long } from '@grpc/proto-loader';

export interface GrpcAllocationInfo_DONTUSE {
  'typeName'?: (string);
  'allocatedObjectsCount'?: (number | string | Long);
  'allocatedBytes'?: (number | string | Long);
}

export interface GrpcAllocationInfo {
  'typeName': (string);
  'allocatedObjectsCount': (number);
  'allocatedBytes': (number);
}
