// Original file: ../../../../../protos/backend_service.proto

import type { GrpcGuid_DONTUSE as _ficus_GrpcGuid_DONTUSE, GrpcGuid as _ficus_GrpcGuid } from '../ficus/GrpcGuid';

export interface GrpcPipelineFinalResult_DONTUSE {
  'success'?: (_ficus_GrpcGuid_DONTUSE | null);
  'error'?: (string);
  'executionResult'?: "success"|"error";
}

export interface GrpcPipelineFinalResult {
  'success'?: (_ficus_GrpcGuid | null);
  'error'?: (string);
  'executionResult': "success"|"error";
}
