// Original file: ../../../../../protos/pm_models.proto

import type { Long } from '@grpc/proto-loader';

export interface GrpcEntityFrequencyAnnotation_DONTUSE {
  'entityId'?: (number | string | Long);
  'frequency'?: (number | string);
}

export interface GrpcEntityFrequencyAnnotation {
  'entityId': (number);
  'frequency': (number);
}
