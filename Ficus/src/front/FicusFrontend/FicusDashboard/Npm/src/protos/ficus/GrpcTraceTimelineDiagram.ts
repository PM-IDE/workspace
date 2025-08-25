// Original file: ../../../../../protos/pipelines_and_context.proto

import type { GrpcThread_DONTUSE as _ficus_GrpcThread_DONTUSE, GrpcThread as _ficus_GrpcThread } from '../ficus/GrpcThread';
import type { GrpcTimelineTraceEventsGroup_DONTUSE as _ficus_GrpcTimelineTraceEventsGroup_DONTUSE, GrpcTimelineTraceEventsGroup as _ficus_GrpcTimelineTraceEventsGroup } from '../ficus/GrpcTimelineTraceEventsGroup';

export interface GrpcTraceTimelineDiagram_DONTUSE {
  'threads'?: (_ficus_GrpcThread_DONTUSE)[];
  'eventsGroups'?: (_ficus_GrpcTimelineTraceEventsGroup_DONTUSE)[];
}

export interface GrpcTraceTimelineDiagram {
  'threads': (_ficus_GrpcThread)[];
  'eventsGroups': (_ficus_GrpcTimelineTraceEventsGroup)[];
}
