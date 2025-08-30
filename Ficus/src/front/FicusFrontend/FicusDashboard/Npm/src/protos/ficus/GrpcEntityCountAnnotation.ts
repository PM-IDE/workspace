// Original file: ../../../../../protos/pm_models.proto

import type { Long } from '@grpc/proto-loader';

export interface GrpcEntityCountAnnotation_DONTUSE {
  'entityId'?: (number | string | Long);
  'count'?: (number | string | Long);
}

export interface GrpcEntityCountAnnotation {
  'entityId': (number);
  'count': (number);
}
