// Original file: ../../../../../protos/util.proto

import type { Long } from '@grpc/proto-loader';

export interface GrpcDateTime_DONTUSE {
  'nanosSinceUnixEpoch'?: (number | string | Long);
}

export interface GrpcDateTime {
  'nanosSinceUnixEpoch': (number);
}
