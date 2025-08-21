// Original file: ../../../../../protos/kafka_service.proto

import type { GrpcGuid_DONTUSE as _ficus_GrpcGuid_DONTUSE, GrpcGuid as _ficus_GrpcGuid } from '../ficus/GrpcGuid';
import type { GrpcKafkaSubscriptionMetadata_DONTUSE as _ficus_GrpcKafkaSubscriptionMetadata_DONTUSE, GrpcKafkaSubscriptionMetadata as _ficus_GrpcKafkaSubscriptionMetadata } from '../ficus/GrpcKafkaSubscriptionMetadata';
import type { GrpcSubscriptionPipeline_DONTUSE as _ficus_GrpcSubscriptionPipeline_DONTUSE, GrpcSubscriptionPipeline as _ficus_GrpcSubscriptionPipeline } from '../ficus/GrpcSubscriptionPipeline';

export interface GrpcKafkaSubscription_DONTUSE {
  'id'?: (_ficus_GrpcGuid_DONTUSE | null);
  'metadata'?: (_ficus_GrpcKafkaSubscriptionMetadata_DONTUSE | null);
  'pipelines'?: (_ficus_GrpcSubscriptionPipeline_DONTUSE)[];
}

export interface GrpcKafkaSubscription {
  'id': (_ficus_GrpcGuid | null);
  'metadata': (_ficus_GrpcKafkaSubscriptionMetadata | null);
  'pipelines': (_ficus_GrpcSubscriptionPipeline)[];
}
