// Original file: ../../../../../protos/kafka_service.proto

import type { GrpcKafkaConnectionMetadata_DONTUSE as _ficus_GrpcKafkaConnectionMetadata_DONTUSE, GrpcKafkaConnectionMetadata as _ficus_GrpcKafkaConnectionMetadata } from '../ficus/GrpcKafkaConnectionMetadata';
import type { GrpcKafkaSubscriptionMetadata_DONTUSE as _ficus_GrpcKafkaSubscriptionMetadata_DONTUSE, GrpcKafkaSubscriptionMetadata as _ficus_GrpcKafkaSubscriptionMetadata } from '../ficus/GrpcKafkaSubscriptionMetadata';

export interface GrpcSubscribeToKafkaRequest_DONTUSE {
  'connectionMetadata'?: (_ficus_GrpcKafkaConnectionMetadata_DONTUSE | null);
  'subscriptionMetadata'?: (_ficus_GrpcKafkaSubscriptionMetadata_DONTUSE | null);
}

export interface GrpcSubscribeToKafkaRequest {
  'connectionMetadata': (_ficus_GrpcKafkaConnectionMetadata | null);
  'subscriptionMetadata': (_ficus_GrpcKafkaSubscriptionMetadata | null);
}
