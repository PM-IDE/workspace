// Original file: ../../../../../protos/pm_models.proto

import type { GrpcGuid_DONTUSE as _ficus_GrpcGuid_DONTUSE, GrpcGuid as _ficus_GrpcGuid } from '../ficus/GrpcGuid';
import type { Empty_DONTUSE as _google_protobuf_Empty_DONTUSE, Empty as _google_protobuf_Empty } from '../google/protobuf/Empty';
import type { Timestamp_DONTUSE as _google_protobuf_Timestamp_DONTUSE, Timestamp as _google_protobuf_Timestamp } from '../google/protobuf/Timestamp';
import type { Long } from '@grpc/proto-loader';

export interface GrpcEventAttribute_DONTUSE {
  'key'?: (string);
  'int'?: (number | string | Long);
  'string'?: (string);
  'bool'?: (boolean);
  'double'?: (number | string);
  'guid'?: (_ficus_GrpcGuid_DONTUSE | null);
  'null'?: (_google_protobuf_Empty_DONTUSE | null);
  'stamp'?: (_google_protobuf_Timestamp_DONTUSE | null);
  'uint'?: (number | string | Long);
  'value'?: "int"|"string"|"bool"|"double"|"guid"|"null"|"stamp"|"uint";
}

export interface GrpcEventAttribute {
  'key': (string);
  'int'?: (number);
  'string'?: (string);
  'bool'?: (boolean);
  'double'?: (number);
  'guid'?: (_ficus_GrpcGuid | null);
  'null'?: (_google_protobuf_Empty | null);
  'stamp'?: (_google_protobuf_Timestamp | null);
  'uint'?: (number);
  'value': "int"|"string"|"bool"|"double"|"guid"|"null"|"stamp"|"uint";
}
