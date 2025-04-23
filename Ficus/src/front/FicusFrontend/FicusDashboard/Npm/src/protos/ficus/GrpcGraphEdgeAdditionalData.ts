// Original file: /Users/aero/work/workspace/Ficus/protos/pipelines_and_context.proto

import type { GrpcSoftwareData_DONTUSE as _ficus_GrpcSoftwareData_DONTUSE, GrpcSoftwareData as _ficus_GrpcSoftwareData } from '../ficus/GrpcSoftwareData';
import type { GrpcEdgeExecutionInfo_DONTUSE as _ficus_GrpcEdgeExecutionInfo_DONTUSE, GrpcEdgeExecutionInfo as _ficus_GrpcEdgeExecutionInfo } from '../ficus/GrpcEdgeExecutionInfo';
import type { GrpcActivityStartEndData_DONTUSE as _ficus_GrpcActivityStartEndData_DONTUSE, GrpcActivityStartEndData as _ficus_GrpcActivityStartEndData } from '../ficus/GrpcActivityStartEndData';

export interface GrpcGraphEdgeAdditionalData_DONTUSE {
  'softwareData'?: (_ficus_GrpcSoftwareData_DONTUSE | null);
  'executionInfo'?: (_ficus_GrpcEdgeExecutionInfo_DONTUSE | null);
  'timeData'?: (_ficus_GrpcActivityStartEndData_DONTUSE | null);
  'data'?: "softwareData"|"executionInfo"|"timeData";
}

export interface GrpcGraphEdgeAdditionalData {
  'softwareData'?: (_ficus_GrpcSoftwareData | null);
  'executionInfo'?: (_ficus_GrpcEdgeExecutionInfo | null);
  'timeData'?: (_ficus_GrpcActivityStartEndData | null);
  'data': "softwareData"|"executionInfo"|"timeData";
}
