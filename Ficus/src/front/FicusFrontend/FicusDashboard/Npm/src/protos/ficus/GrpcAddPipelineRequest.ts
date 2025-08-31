// Original file: ../../../../../protos/kafka_service.proto

import type { GrpcKafkaPipelineExecutionRequest_DONTUSE as _ficus_GrpcKafkaPipelineExecutionRequest_DONTUSE, GrpcKafkaPipelineExecutionRequest as _ficus_GrpcKafkaPipelineExecutionRequest } from '../ficus/GrpcKafkaPipelineExecutionRequest';
import type { GrpcKafkaConnectionMetadata_DONTUSE as _ficus_GrpcKafkaConnectionMetadata_DONTUSE, GrpcKafkaConnectionMetadata as _ficus_GrpcKafkaConnectionMetadata } from '../ficus/GrpcKafkaConnectionMetadata';

export interface GrpcAddPipelineRequest_DONTUSE {
  'pipelineRequest'?: (_ficus_GrpcKafkaPipelineExecutionRequest_DONTUSE | null);
  'producerKafkaMetadata'?: (_ficus_GrpcKafkaConnectionMetadata_DONTUSE | null);
}

export interface GrpcAddPipelineRequest {
  'pipelineRequest': (_ficus_GrpcKafkaPipelineExecutionRequest | null);
  'producerKafkaMetadata': (_ficus_GrpcKafkaConnectionMetadata | null);
}
