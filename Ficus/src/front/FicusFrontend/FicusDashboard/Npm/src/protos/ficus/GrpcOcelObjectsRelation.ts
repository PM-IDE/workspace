// Original file: ../../../../../protos/pipelines_and_context.proto

import type { Long } from '@grpc/proto-loader';

export interface GrpcOcelObjectsRelation_DONTUSE {
  'fromElementId'?: (number | string | Long);
  'objectId'?: (string);
}

export interface GrpcOcelObjectsRelation {
  'fromElementId': (number);
  'objectId': (string);
}
