// Original file: ../../../../../protos/pipelines_and_context.proto

import type { GrpcGraphEdgeAdditionalData_DONTUSE as _ficus_GrpcGraphEdgeAdditionalData_DONTUSE, GrpcGraphEdgeAdditionalData as _ficus_GrpcGraphEdgeAdditionalData } from '../ficus/GrpcGraphEdgeAdditionalData';
import type { Long } from '@grpc/proto-loader';

export interface GrpcGraphEdge_DONTUSE {
  'id'?: (number | string | Long);
  'fromNode'?: (number | string | Long);
  'toNode'?: (number | string | Long);
  'weight'?: (number | string);
  'data'?: (string);
  'additionalData'?: (_ficus_GrpcGraphEdgeAdditionalData_DONTUSE)[];
}

export interface GrpcGraphEdge {
  'id': (number);
  'fromNode': (number);
  'toNode': (number);
  'weight': (number);
  'data': (string);
  'additionalData': (_ficus_GrpcGraphEdgeAdditionalData)[];
}
