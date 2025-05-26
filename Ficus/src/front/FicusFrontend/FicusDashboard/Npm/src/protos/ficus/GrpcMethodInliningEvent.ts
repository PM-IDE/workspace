// Original file: /Users/aero/work/workspace/Ficus/protos/pipelines_and_context.proto

import type { GrpcMethodInliningInfo_DONTUSE as _ficus_GrpcMethodInliningInfo_DONTUSE, GrpcMethodInliningInfo as _ficus_GrpcMethodInliningInfo } from '../ficus/GrpcMethodInliningInfo';
import type { Empty_DONTUSE as _google_protobuf_Empty_DONTUSE, Empty as _google_protobuf_Empty } from '../google/protobuf/Empty';
import type { GrpcMethodInliningFailedEvent_DONTUSE as _ficus_GrpcMethodInliningFailedEvent_DONTUSE, GrpcMethodInliningFailedEvent as _ficus_GrpcMethodInliningFailedEvent } from '../ficus/GrpcMethodInliningFailedEvent';

export interface GrpcMethodInliningEvent_DONTUSE {
  'inliningInfo'?: (_ficus_GrpcMethodInliningInfo_DONTUSE | null);
  'succeeded'?: (_google_protobuf_Empty_DONTUSE | null);
  'failed'?: (_ficus_GrpcMethodInliningFailedEvent_DONTUSE | null);
  'event'?: "succeeded"|"failed";
}

export interface GrpcMethodInliningEvent {
  'inliningInfo': (_ficus_GrpcMethodInliningInfo | null);
  'succeeded'?: (_google_protobuf_Empty | null);
  'failed'?: (_ficus_GrpcMethodInliningFailedEvent | null);
  'event': "succeeded"|"failed";
}
