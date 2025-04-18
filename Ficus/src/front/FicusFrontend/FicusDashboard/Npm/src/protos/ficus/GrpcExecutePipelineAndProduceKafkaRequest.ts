// Original file: /Users/aero/work/workspace/Ficus/protos/kafka_service.proto

import type { GrpcProxyPipelineExecutionRequest_DONTUSE as _ficus_GrpcProxyPipelineExecutionRequest_DONTUSE, GrpcProxyPipelineExecutionRequest as _ficus_GrpcProxyPipelineExecutionRequest } from '../ficus/GrpcProxyPipelineExecutionRequest';
import type { GrpcKafkaConnectionMetadata_DONTUSE as _ficus_GrpcKafkaConnectionMetadata_DONTUSE, GrpcKafkaConnectionMetadata as _ficus_GrpcKafkaConnectionMetadata } from '../ficus/GrpcKafkaConnectionMetadata';
import type { GrpcProcessInfo_DONTUSE as _ficus_GrpcProcessInfo_DONTUSE, GrpcProcessInfo as _ficus_GrpcProcessInfo } from '../ficus/GrpcProcessInfo';
import type { GrpcGuid_DONTUSE as _ficus_GrpcGuid_DONTUSE, GrpcGuid as _ficus_GrpcGuid } from '../ficus/GrpcGuid';

export interface GrpcExecutePipelineAndProduceKafkaRequest_DONTUSE {
  'pipelineRequest'?: (_ficus_GrpcProxyPipelineExecutionRequest_DONTUSE | null);
  'producerMetadata'?: (_ficus_GrpcKafkaConnectionMetadata_DONTUSE | null);
  'caseInfo'?: (_ficus_GrpcProcessInfo_DONTUSE | null);
  'subscriptionId'?: (_ficus_GrpcGuid_DONTUSE | null);
  'pipelineId'?: (_ficus_GrpcGuid_DONTUSE | null);
  'pipelineName'?: (string);
  'subscriptionName'?: (string);
}

export interface GrpcExecutePipelineAndProduceKafkaRequest {
  'pipelineRequest': (_ficus_GrpcProxyPipelineExecutionRequest | null);
  'producerMetadata': (_ficus_GrpcKafkaConnectionMetadata | null);
  'caseInfo': (_ficus_GrpcProcessInfo | null);
  'subscriptionId': (_ficus_GrpcGuid | null);
  'pipelineId': (_ficus_GrpcGuid | null);
  'pipelineName': (string);
  'subscriptionName': (string);
}
