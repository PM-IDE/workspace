// Original file: ../../../../../protos/pipelines_and_context.proto

import type { GrpcHistogramEntry_DONTUSE as _ficus_GrpcHistogramEntry_DONTUSE, GrpcHistogramEntry as _ficus_GrpcHistogramEntry } from '../ficus/GrpcHistogramEntry';
import type { GrpcTimelineDiagramFragment_DONTUSE as _ficus_GrpcTimelineDiagramFragment_DONTUSE, GrpcTimelineDiagramFragment as _ficus_GrpcTimelineDiagramFragment } from '../ficus/GrpcTimelineDiagramFragment';
import type { GrpcGeneralHistogramData_DONTUSE as _ficus_GrpcGeneralHistogramData_DONTUSE, GrpcGeneralHistogramData as _ficus_GrpcGeneralHistogramData } from '../ficus/GrpcGeneralHistogramData';
import type { GrpcSimpleCounterData_DONTUSE as _ficus_GrpcSimpleCounterData_DONTUSE, GrpcSimpleCounterData as _ficus_GrpcSimpleCounterData } from '../ficus/GrpcSimpleCounterData';
import type { GrpcActivityDurationData_DONTUSE as _ficus_GrpcActivityDurationData_DONTUSE, GrpcActivityDurationData as _ficus_GrpcActivityDurationData } from '../ficus/GrpcActivityDurationData';
import type { GrpcOcelData_DONTUSE as _ficus_GrpcOcelData_DONTUSE, GrpcOcelData as _ficus_GrpcOcelData } from '../ficus/GrpcOcelData';

export interface GrpcSoftwareData_DONTUSE {
  'histogram'?: (_ficus_GrpcHistogramEntry_DONTUSE)[];
  'timelineDiagramFragment'?: (_ficus_GrpcTimelineDiagramFragment_DONTUSE | null);
  'histogramData'?: (_ficus_GrpcGeneralHistogramData_DONTUSE)[];
  'simpleCounterData'?: (_ficus_GrpcSimpleCounterData_DONTUSE)[];
  'activitiesDurationsData'?: (_ficus_GrpcActivityDurationData_DONTUSE)[];
  'ocelData'?: (_ficus_GrpcOcelData_DONTUSE)[];
}

export interface GrpcSoftwareData {
  'histogram': (_ficus_GrpcHistogramEntry)[];
  'timelineDiagramFragment': (_ficus_GrpcTimelineDiagramFragment | null);
  'histogramData': (_ficus_GrpcGeneralHistogramData)[];
  'simpleCounterData': (_ficus_GrpcSimpleCounterData)[];
  'activitiesDurationsData': (_ficus_GrpcActivityDurationData)[];
  'ocelData': (_ficus_GrpcOcelData)[];
}
