// Original file: ../../../../../protos/pipelines_and_context.proto

import type { GrpcNodeAdditionalData_DONTUSE as _ficus_GrpcNodeAdditionalData_DONTUSE, GrpcNodeAdditionalData as _ficus_GrpcNodeAdditionalData } from '../ficus/GrpcNodeAdditionalData';
import type { GrpcGraph_DONTUSE as _ficus_GrpcGraph_DONTUSE, GrpcGraph as _ficus_GrpcGraph } from '../ficus/GrpcGraph';
import type { Long } from '@grpc/proto-loader';

export interface GrpcGraphNode_DONTUSE {
  'id'?: (number | string | Long);
  'data'?: (string);
  'additionalData'?: (_ficus_GrpcNodeAdditionalData_DONTUSE)[];
  'innerGraph'?: (_ficus_GrpcGraph_DONTUSE | null);
}

export interface GrpcGraphNode {
  'id': (number);
  'data': (string);
  'additionalData': (_ficus_GrpcNodeAdditionalData)[];
  'innerGraph': (_ficus_GrpcGraph | null);
}
