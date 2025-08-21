// Original file: ../../../../../protos/kafka_service.proto

import type { GrpcT1EventsTimeBasedCaching_DONTUSE as _ficus_GrpcT1EventsTimeBasedCaching_DONTUSE, GrpcT1EventsTimeBasedCaching as _ficus_GrpcT1EventsTimeBasedCaching } from '../ficus/GrpcT1EventsTimeBasedCaching';
import type { GrpcT1TraceTimeBasedCaching_DONTUSE as _ficus_GrpcT1TraceTimeBasedCaching_DONTUSE, GrpcT1TraceTimeBasedCaching as _ficus_GrpcT1TraceTimeBasedCaching } from '../ficus/GrpcT1TraceTimeBasedCaching';
import type { GrpcT1TracesQueueConfiguration_DONTUSE as _ficus_GrpcT1TracesQueueConfiguration_DONTUSE, GrpcT1TracesQueueConfiguration as _ficus_GrpcT1TracesQueueConfiguration } from '../ficus/GrpcT1TracesQueueConfiguration';

export interface GrpcT1StreamingConfiguration_DONTUSE {
  'eventsTimeout'?: (_ficus_GrpcT1EventsTimeBasedCaching_DONTUSE | null);
  'tracesTimeout'?: (_ficus_GrpcT1TraceTimeBasedCaching_DONTUSE | null);
  'tracesQueueConfiguration'?: (_ficus_GrpcT1TracesQueueConfiguration_DONTUSE | null);
  'configuration'?: "eventsTimeout"|"tracesTimeout"|"tracesQueueConfiguration";
}

export interface GrpcT1StreamingConfiguration {
  'eventsTimeout'?: (_ficus_GrpcT1EventsTimeBasedCaching | null);
  'tracesTimeout'?: (_ficus_GrpcT1TraceTimeBasedCaching | null);
  'tracesQueueConfiguration'?: (_ficus_GrpcT1TracesQueueConfiguration | null);
  'configuration': "eventsTimeout"|"tracesTimeout"|"tracesQueueConfiguration";
}
