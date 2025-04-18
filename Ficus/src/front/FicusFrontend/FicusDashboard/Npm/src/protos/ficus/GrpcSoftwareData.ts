// Original file: /Users/aero/work/workspace/Ficus/protos/pipelines_and_context.proto

import type { GrpcHistogramEntry_DONTUSE as _ficus_GrpcHistogramEntry_DONTUSE, GrpcHistogramEntry as _ficus_GrpcHistogramEntry } from '../ficus/GrpcHistogramEntry';
import type { GrpcTimelineDiagramFragment_DONTUSE as _ficus_GrpcTimelineDiagramFragment_DONTUSE, GrpcTimelineDiagramFragment as _ficus_GrpcTimelineDiagramFragment } from '../ficus/GrpcTimelineDiagramFragment';
import type { GrpcAllocationInfo_DONTUSE as _ficus_GrpcAllocationInfo_DONTUSE, GrpcAllocationInfo as _ficus_GrpcAllocationInfo } from '../ficus/GrpcAllocationInfo';
import type { GrpcExecutionSuspensionInfo_DONTUSE as _ficus_GrpcExecutionSuspensionInfo_DONTUSE, GrpcExecutionSuspensionInfo as _ficus_GrpcExecutionSuspensionInfo } from '../ficus/GrpcExecutionSuspensionInfo';
import type { GrpcThreadEventInfo_DONTUSE as _ficus_GrpcThreadEventInfo_DONTUSE, GrpcThreadEventInfo as _ficus_GrpcThreadEventInfo } from '../ficus/GrpcThreadEventInfo';
import type { GrpcMethodInliningEvent_DONTUSE as _ficus_GrpcMethodInliningEvent_DONTUSE, GrpcMethodInliningEvent as _ficus_GrpcMethodInliningEvent } from '../ficus/GrpcMethodInliningEvent';
import type { GrpcArrayPoolEvent_DONTUSE as _ficus_GrpcArrayPoolEvent_DONTUSE, GrpcArrayPoolEvent as _ficus_GrpcArrayPoolEvent } from '../ficus/GrpcArrayPoolEvent';
import type { GrpcExceptionEvent_DONTUSE as _ficus_GrpcExceptionEvent_DONTUSE, GrpcExceptionEvent as _ficus_GrpcExceptionEvent } from '../ficus/GrpcExceptionEvent';
import type { GrpcHTTPEvent_DONTUSE as _ficus_GrpcHTTPEvent_DONTUSE, GrpcHTTPEvent as _ficus_GrpcHTTPEvent } from '../ficus/GrpcHTTPEvent';
import type { GrpcContentionEvent_DONTUSE as _ficus_GrpcContentionEvent_DONTUSE, GrpcContentionEvent as _ficus_GrpcContentionEvent } from '../ficus/GrpcContentionEvent';
import type { GrpcSocketEvent_DONTUSE as _ficus_GrpcSocketEvent_DONTUSE, GrpcSocketEvent as _ficus_GrpcSocketEvent } from '../ficus/GrpcSocketEvent';

export interface GrpcSoftwareData_DONTUSE {
  'histogram'?: (_ficus_GrpcHistogramEntry_DONTUSE)[];
  'timelineDiagramFragment'?: (_ficus_GrpcTimelineDiagramFragment_DONTUSE | null);
  'allocationsInfo'?: (_ficus_GrpcAllocationInfo_DONTUSE)[];
  'executionSuspensionInfo'?: (_ficus_GrpcExecutionSuspensionInfo_DONTUSE)[];
  'threadEvents'?: (_ficus_GrpcThreadEventInfo_DONTUSE)[];
  'methodsInliningEvents'?: (_ficus_GrpcMethodInliningEvent_DONTUSE)[];
  'arrayPoolEvents'?: (_ficus_GrpcArrayPoolEvent_DONTUSE)[];
  'exceptionEvents'?: (_ficus_GrpcExceptionEvent_DONTUSE)[];
  'httpEvents'?: (_ficus_GrpcHTTPEvent_DONTUSE)[];
  'contentionEvents'?: (_ficus_GrpcContentionEvent_DONTUSE)[];
  'socketEvent'?: (_ficus_GrpcSocketEvent_DONTUSE)[];
}

export interface GrpcSoftwareData {
  'histogram': (_ficus_GrpcHistogramEntry)[];
  'timelineDiagramFragment': (_ficus_GrpcTimelineDiagramFragment | null);
  'allocationsInfo': (_ficus_GrpcAllocationInfo)[];
  'executionSuspensionInfo': (_ficus_GrpcExecutionSuspensionInfo)[];
  'threadEvents': (_ficus_GrpcThreadEventInfo)[];
  'methodsInliningEvents': (_ficus_GrpcMethodInliningEvent)[];
  'arrayPoolEvents': (_ficus_GrpcArrayPoolEvent)[];
  'exceptionEvents': (_ficus_GrpcExceptionEvent)[];
  'httpEvents': (_ficus_GrpcHTTPEvent)[];
  'contentionEvents': (_ficus_GrpcContentionEvent)[];
  'socketEvent': (_ficus_GrpcSocketEvent)[];
}
