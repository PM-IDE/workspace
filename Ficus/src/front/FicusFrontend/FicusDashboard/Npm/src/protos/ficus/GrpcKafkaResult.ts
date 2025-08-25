// Original file: ../../../../../protos/kafka_service.proto

import type { GrpcKafkaSuccessResult_DONTUSE as _ficus_GrpcKafkaSuccessResult_DONTUSE, GrpcKafkaSuccessResult as _ficus_GrpcKafkaSuccessResult } from '../ficus/GrpcKafkaSuccessResult';
import type { GrpcKafkaFailedResult_DONTUSE as _ficus_GrpcKafkaFailedResult_DONTUSE, GrpcKafkaFailedResult as _ficus_GrpcKafkaFailedResult } from '../ficus/GrpcKafkaFailedResult';

export interface GrpcKafkaResult_DONTUSE {
  'success'?: (_ficus_GrpcKafkaSuccessResult_DONTUSE | null);
  'failure'?: (_ficus_GrpcKafkaFailedResult_DONTUSE | null);
  'result'?: "success"|"failure";
}

export interface GrpcKafkaResult {
  'success'?: (_ficus_GrpcKafkaSuccessResult | null);
  'failure'?: (_ficus_GrpcKafkaFailedResult | null);
  'result': "success"|"failure";
}
