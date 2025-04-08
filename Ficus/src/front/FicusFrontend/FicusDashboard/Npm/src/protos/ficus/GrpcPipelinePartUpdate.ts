// Original file: /Users/aero/work/workspace/Ficus/protos/front_contract.proto

import type { GrpcCurrentCasesResponse_DONTUSE as _ficus_GrpcCurrentCasesResponse_DONTUSE, GrpcCurrentCasesResponse as _ficus_GrpcCurrentCasesResponse } from '../ficus/GrpcCurrentCasesResponse';
import type { GrpcKafkaUpdate_DONTUSE as _ficus_GrpcKafkaUpdate_DONTUSE, GrpcKafkaUpdate as _ficus_GrpcKafkaUpdate } from '../ficus/GrpcKafkaUpdate';

export interface GrpcPipelinePartUpdate_DONTUSE {
  'currentCases'?: (_ficus_GrpcCurrentCasesResponse_DONTUSE | null);
  'delta'?: (_ficus_GrpcKafkaUpdate_DONTUSE | null);
  'update'?: "currentCases"|"delta";
}

export interface GrpcPipelinePartUpdate {
  'currentCases'?: (_ficus_GrpcCurrentCasesResponse | null);
  'delta'?: (_ficus_GrpcKafkaUpdate | null);
  'update': "currentCases"|"delta";
}
