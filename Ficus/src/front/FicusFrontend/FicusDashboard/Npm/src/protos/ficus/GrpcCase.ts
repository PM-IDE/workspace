// Original file: /Users/aero/work/workspace/Ficus/protos/front_contract.proto

import type { GrpcProcessCaseMetadata_DONTUSE as _ficus_GrpcProcessCaseMetadata_DONTUSE, GrpcProcessCaseMetadata as _ficus_GrpcProcessCaseMetadata } from '../ficus/GrpcProcessCaseMetadata';
import type { GrpcPipelinePartContextValues_DONTUSE as _ficus_GrpcPipelinePartContextValues_DONTUSE, GrpcPipelinePartContextValues as _ficus_GrpcPipelinePartContextValues } from '../ficus/GrpcPipelinePartContextValues';

export interface GrpcCase_DONTUSE {
  'processCaseMetadata'?: (_ficus_GrpcProcessCaseMetadata_DONTUSE | null);
  'contextValues'?: (_ficus_GrpcPipelinePartContextValues_DONTUSE)[];
}

export interface GrpcCase {
  'processCaseMetadata': (_ficus_GrpcProcessCaseMetadata | null);
  'contextValues': (_ficus_GrpcPipelinePartContextValues)[];
}
