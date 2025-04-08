// Original file: /Users/aero/work/workspace/Ficus/protos/pipelines_and_context.proto

import type { Long } from '@grpc/proto-loader';

export interface GrpcGraphEdge_DONTUSE {
  'id'?: (number | string | Long);
  'fromNode'?: (number | string | Long);
  'toNode'?: (number | string | Long);
  'weight'?: (number | string);
  'data'?: (string);
}

export interface GrpcGraphEdge {
  'id': (number);
  'fromNode': (number);
  'toNode': (number);
  'weight': (number);
  'data': (string);
}
