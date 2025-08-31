// Original file: ../../../../../protos/pm_models.proto

import type { Timestamp_DONTUSE as _google_protobuf_Timestamp_DONTUSE, Timestamp as _google_protobuf_Timestamp } from '../google/protobuf/Timestamp';
import type { GrpcEventAttribute_DONTUSE as _ficus_GrpcEventAttribute_DONTUSE, GrpcEventAttribute as _ficus_GrpcEventAttribute } from '../ficus/GrpcEventAttribute';

export interface GrpcEvent_DONTUSE {
  'name'?: (string);
  'stamp'?: (_google_protobuf_Timestamp_DONTUSE | null);
  'attributes'?: (_ficus_GrpcEventAttribute_DONTUSE)[];
}

export interface GrpcEvent {
  'name': (string);
  'stamp': (_google_protobuf_Timestamp | null);
  'attributes': (_ficus_GrpcEventAttribute)[];
}
