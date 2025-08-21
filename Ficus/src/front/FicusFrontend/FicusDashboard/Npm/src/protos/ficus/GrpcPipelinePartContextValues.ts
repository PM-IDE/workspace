// Original file: ../../../../../protos/front_contract.proto

import type { GrpcPipelinePartInfo_DONTUSE as _ficus_GrpcPipelinePartInfo_DONTUSE, GrpcPipelinePartInfo as _ficus_GrpcPipelinePartInfo } from '../ficus/GrpcPipelinePartInfo';
import type { Timestamp_DONTUSE as _google_protobuf_Timestamp_DONTUSE, Timestamp as _google_protobuf_Timestamp } from '../google/protobuf/Timestamp';
import type { GrpcCasePipelinePartExecutionResult_DONTUSE as _ficus_GrpcCasePipelinePartExecutionResult_DONTUSE, GrpcCasePipelinePartExecutionResult as _ficus_GrpcCasePipelinePartExecutionResult } from '../ficus/GrpcCasePipelinePartExecutionResult';

export interface GrpcPipelinePartContextValues_DONTUSE {
  'pipelinePartInfo'?: (_ficus_GrpcPipelinePartInfo_DONTUSE | null);
  'stamp'?: (_google_protobuf_Timestamp_DONTUSE | null);
  'executionResults'?: (_ficus_GrpcCasePipelinePartExecutionResult_DONTUSE)[];
}

export interface GrpcPipelinePartContextValues {
  'pipelinePartInfo': (_ficus_GrpcPipelinePartInfo | null);
  'stamp': (_google_protobuf_Timestamp | null);
  'executionResults': (_ficus_GrpcCasePipelinePartExecutionResult)[];
}
