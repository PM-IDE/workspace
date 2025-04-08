// Original file: /Users/aero/work/workspace/Ficus/protos/pipelines_and_context.proto

import type { GrpcHistogramEntry_DONTUSE as _ficus_GrpcHistogramEntry_DONTUSE, GrpcHistogramEntry as _ficus_GrpcHistogramEntry } from '../ficus/GrpcHistogramEntry';
import type { GrpcTimelineDiagramFragment_DONTUSE as _ficus_GrpcTimelineDiagramFragment_DONTUSE, GrpcTimelineDiagramFragment as _ficus_GrpcTimelineDiagramFragment } from '../ficus/GrpcTimelineDiagramFragment';
import type { GrpcAllocationsInfo_DONTUSE as _ficus_GrpcAllocationsInfo_DONTUSE, GrpcAllocationsInfo as _ficus_GrpcAllocationsInfo } from '../ficus/GrpcAllocationsInfo';

export interface GrpcSoftwareData_DONTUSE {
  'histogram'?: (_ficus_GrpcHistogramEntry_DONTUSE)[];
  'timelineDiagramFragment'?: (_ficus_GrpcTimelineDiagramFragment_DONTUSE | null);
  'allocationsInfo'?: (_ficus_GrpcAllocationsInfo_DONTUSE | null);
}

export interface GrpcSoftwareData {
  'histogram': (_ficus_GrpcHistogramEntry)[];
  'timelineDiagramFragment': (_ficus_GrpcTimelineDiagramFragment | null);
  'allocationsInfo': (_ficus_GrpcAllocationsInfo | null);
}
