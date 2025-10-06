// Original file: ../../../../../protos/pipelines_and_context.proto

import type { Empty_DONTUSE as _google_protobuf_Empty_DONTUSE, Empty as _google_protobuf_Empty } from '../google/protobuf/Empty';

export interface GrpcOcelData_DONTUSE {
  'objectType'?: (string);
  'objectId'?: (string);
  'Allocate'?: (_google_protobuf_Empty_DONTUSE | null);
  'Consume'?: (_google_protobuf_Empty_DONTUSE | null);
  'action'?: "Allocate"|"Consume";
}

export interface GrpcOcelData {
  'objectType': (string);
  'objectId': (string);
  'Allocate'?: (_google_protobuf_Empty | null);
  'Consume'?: (_google_protobuf_Empty | null);
  'action': "Allocate"|"Consume";
}
