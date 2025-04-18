// Original file: /Users/aero/work/workspace/Ficus/protos/kafka_service.proto

import type { GrpcT2LossyCountConfiguration_DONTUSE as _ficus_GrpcT2LossyCountConfiguration_DONTUSE, GrpcT2LossyCountConfiguration as _ficus_GrpcT2LossyCountConfiguration } from '../ficus/GrpcT2LossyCountConfiguration';
import type { GrpcT2TimedSlidingWindowConfiguration_DONTUSE as _ficus_GrpcT2TimedSlidingWindowConfiguration_DONTUSE, GrpcT2TimedSlidingWindowConfiguration as _ficus_GrpcT2TimedSlidingWindowConfiguration } from '../ficus/GrpcT2TimedSlidingWindowConfiguration';
import type { GrpcPipeline_DONTUSE as _ficus_GrpcPipeline_DONTUSE, GrpcPipeline as _ficus_GrpcPipeline } from '../ficus/GrpcPipeline';

export interface GrpcT2StreamingConfiguration_DONTUSE {
  'lossyCount'?: (_ficus_GrpcT2LossyCountConfiguration_DONTUSE | null);
  'timedSlidingWindow'?: (_ficus_GrpcT2TimedSlidingWindowConfiguration_DONTUSE | null);
  'incomingTracesFilteringPipeline'?: (_ficus_GrpcPipeline_DONTUSE | null);
  'configuration'?: "lossyCount"|"timedSlidingWindow";
}

export interface GrpcT2StreamingConfiguration {
  'lossyCount'?: (_ficus_GrpcT2LossyCountConfiguration | null);
  'timedSlidingWindow'?: (_ficus_GrpcT2TimedSlidingWindowConfiguration | null);
  'incomingTracesFilteringPipeline': (_ficus_GrpcPipeline | null);
  'configuration': "lossyCount"|"timedSlidingWindow";
}
