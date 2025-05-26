// Original file: /Users/aero/work/workspace/Ficus/protos/pipelines_and_context.proto

import type { Empty_DONTUSE as _google_protobuf_Empty_DONTUSE, Empty as _google_protobuf_Empty } from '../google/protobuf/Empty';
import type { Long } from '@grpc/proto-loader';

export interface GrpcArrayPoolEvent_DONTUSE {
  'bufferId'?: (number | string | Long);
  'bufferSizeBytes'?: (number | string | Long);
  'bufferAllocated'?: (_google_protobuf_Empty_DONTUSE | null);
  'bufferRented'?: (_google_protobuf_Empty_DONTUSE | null);
  'bufferReturned'?: (_google_protobuf_Empty_DONTUSE | null);
  'bufferTrimmed'?: (_google_protobuf_Empty_DONTUSE | null);
  'event'?: "bufferAllocated"|"bufferRented"|"bufferReturned"|"bufferTrimmed";
}

export interface GrpcArrayPoolEvent {
  'bufferId': (number);
  'bufferSizeBytes': (number);
  'bufferAllocated'?: (_google_protobuf_Empty | null);
  'bufferRented'?: (_google_protobuf_Empty | null);
  'bufferReturned'?: (_google_protobuf_Empty | null);
  'bufferTrimmed'?: (_google_protobuf_Empty | null);
  'event': "bufferAllocated"|"bufferRented"|"bufferReturned"|"bufferTrimmed";
}
