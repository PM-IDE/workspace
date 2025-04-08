// Original file: /Users/aero/work/workspace/Ficus/protos/pipelines_and_context.proto

import type { Empty_DONTUSE as _google_protobuf_Empty_DONTUSE, Empty as _google_protobuf_Empty } from '../google/protobuf/Empty';
import type { GrpcSoftwareData_DONTUSE as _ficus_GrpcSoftwareData_DONTUSE, GrpcSoftwareData as _ficus_GrpcSoftwareData } from '../ficus/GrpcSoftwareData';
import type { GrpcUnderlyingPatternInfo_DONTUSE as _ficus_GrpcUnderlyingPatternInfo_DONTUSE, GrpcUnderlyingPatternInfo as _ficus_GrpcUnderlyingPatternInfo } from '../ficus/GrpcUnderlyingPatternInfo';
import type { GrpcNodeCorrespondingTraceData_DONTUSE as _ficus_GrpcNodeCorrespondingTraceData_DONTUSE, GrpcNodeCorrespondingTraceData as _ficus_GrpcNodeCorrespondingTraceData } from '../ficus/GrpcNodeCorrespondingTraceData';
import type { GrpcNodeTimeActivityStartEndData_DONTUSE as _ficus_GrpcNodeTimeActivityStartEndData_DONTUSE, GrpcNodeTimeActivityStartEndData as _ficus_GrpcNodeTimeActivityStartEndData } from '../ficus/GrpcNodeTimeActivityStartEndData';
import type { GrpcEventCoordinates_DONTUSE as _ficus_GrpcEventCoordinates_DONTUSE, GrpcEventCoordinates as _ficus_GrpcEventCoordinates } from '../ficus/GrpcEventCoordinates';

export interface GrpcNodeAdditionalData_DONTUSE {
  'none'?: (_google_protobuf_Empty_DONTUSE | null);
  'softwareData'?: (_ficus_GrpcSoftwareData_DONTUSE | null);
  'patternInfo'?: (_ficus_GrpcUnderlyingPatternInfo_DONTUSE | null);
  'traceData'?: (_ficus_GrpcNodeCorrespondingTraceData_DONTUSE | null);
  'timeData'?: (_ficus_GrpcNodeTimeActivityStartEndData_DONTUSE | null);
  'originalEventCoordinates'?: (_ficus_GrpcEventCoordinates_DONTUSE | null);
  'data'?: "none"|"softwareData"|"patternInfo"|"traceData"|"timeData";
}

export interface GrpcNodeAdditionalData {
  'none'?: (_google_protobuf_Empty | null);
  'softwareData'?: (_ficus_GrpcSoftwareData | null);
  'patternInfo'?: (_ficus_GrpcUnderlyingPatternInfo | null);
  'traceData'?: (_ficus_GrpcNodeCorrespondingTraceData | null);
  'timeData'?: (_ficus_GrpcNodeTimeActivityStartEndData | null);
  'originalEventCoordinates': (_ficus_GrpcEventCoordinates | null);
  'data': "none"|"softwareData"|"patternInfo"|"traceData"|"timeData";
}
