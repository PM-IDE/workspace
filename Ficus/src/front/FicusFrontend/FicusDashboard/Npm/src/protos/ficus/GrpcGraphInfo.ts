// Original file: ../../../../../protos/pm_models.proto

import type { Long } from '@grpc/proto-loader';

export interface GrpcGraphInfo_DONTUSE {
  'nodesCount'?: (number | string | Long);
  'edgesCount'?: (number | string | Long);
}

export interface GrpcGraphInfo {
  'nodesCount': (number);
  'edgesCount': (number);
}
