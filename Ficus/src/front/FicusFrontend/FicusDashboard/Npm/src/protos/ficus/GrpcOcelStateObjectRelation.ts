// Original file: ../../../../../protos/pipelines_and_context.proto

import type { Long } from '@grpc/proto-loader';

export interface GrpcOcelStateObjectRelation_DONTUSE {
  'objectId'?: (string);
  'elementId'?: (number | string | Long);
  'relatedObjectsIds'?: (string)[];
}

export interface GrpcOcelStateObjectRelation {
  'objectId': (string);
  'elementId': (number);
  'relatedObjectsIds': (string)[];
}
