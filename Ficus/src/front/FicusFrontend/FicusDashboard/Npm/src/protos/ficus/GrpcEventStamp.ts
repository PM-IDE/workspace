// Original file: ../../../../../protos/pm_models.proto

import type { Timestamp_DONTUSE as _google_protobuf_Timestamp_DONTUSE, Timestamp as _google_protobuf_Timestamp } from '../google/protobuf/Timestamp';
import type { Long } from '@grpc/proto-loader';

export interface GrpcEventStamp_DONTUSE {
  'date'?: (_google_protobuf_Timestamp_DONTUSE | null);
  'order'?: (number | string | Long);
  'stamp'?: "date"|"order";
}

export interface GrpcEventStamp {
  'date'?: (_google_protobuf_Timestamp | null);
  'order'?: (number);
  'stamp': "date"|"order";
}
