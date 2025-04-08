// Original file: /Users/aero/work/workspace/Ficus/protos/front_contract.proto

import type { GrpcProcessCaseMetadata_DONTUSE as _ficus_GrpcProcessCaseMetadata_DONTUSE, GrpcProcessCaseMetadata as _ficus_GrpcProcessCaseMetadata } from '../ficus/GrpcProcessCaseMetadata';
import type { GrpcPipelinePartInfo_DONTUSE as _ficus_GrpcPipelinePartInfo_DONTUSE, GrpcPipelinePartInfo as _ficus_GrpcPipelinePartInfo } from '../ficus/GrpcPipelinePartInfo';
import type { GrpcContextValueWithKeyName_DONTUSE as _ficus_GrpcContextValueWithKeyName_DONTUSE, GrpcContextValueWithKeyName as _ficus_GrpcContextValueWithKeyName } from '../ficus/GrpcContextValueWithKeyName';

export interface GrpcKafkaUpdate_DONTUSE {
  'processCaseMetadata'?: (_ficus_GrpcProcessCaseMetadata_DONTUSE | null);
  'pipelinePartInfo'?: (_ficus_GrpcPipelinePartInfo_DONTUSE | null);
  'contextValues'?: (_ficus_GrpcContextValueWithKeyName_DONTUSE)[];
}

export interface GrpcKafkaUpdate {
  'processCaseMetadata': (_ficus_GrpcProcessCaseMetadata | null);
  'pipelinePartInfo': (_ficus_GrpcPipelinePartInfo | null);
  'contextValues': (_ficus_GrpcContextValueWithKeyName)[];
}
