// Original file: ../../../../../protos/kafka_service.proto

import type { GrpcKafkaMetadata_DONTUSE as _ficus_GrpcKafkaMetadata_DONTUSE, GrpcKafkaMetadata as _ficus_GrpcKafkaMetadata } from '../ficus/GrpcKafkaMetadata';

export interface GrpcKafkaConnectionMetadata_DONTUSE {
  'topicName'?: (string);
  'metadata'?: (_ficus_GrpcKafkaMetadata_DONTUSE)[];
}

export interface GrpcKafkaConnectionMetadata {
  'topicName': (string);
  'metadata': (_ficus_GrpcKafkaMetadata)[];
}
