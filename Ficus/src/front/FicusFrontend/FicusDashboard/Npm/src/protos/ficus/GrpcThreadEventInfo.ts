// Original file: /Users/aero/work/workspace/Ficus/protos/pipelines_and_context.proto

import type { Empty_DONTUSE as _google_protobuf_Empty_DONTUSE, Empty as _google_protobuf_Empty } from '../google/protobuf/Empty';
import type { Long } from '@grpc/proto-loader';

export interface GrpcThreadEventInfo_DONTUSE {
  'threadId'?: (number | string | Long);
  'created'?: (_google_protobuf_Empty_DONTUSE | null);
  'terminated'?: (_google_protobuf_Empty_DONTUSE | null);
  'event'?: "created"|"terminated";
}

export interface GrpcThreadEventInfo {
  'threadId': (number);
  'created'?: (_google_protobuf_Empty | null);
  'terminated'?: (_google_protobuf_Empty | null);
  'event': "created"|"terminated";
}
