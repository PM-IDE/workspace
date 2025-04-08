// Original file: /Users/aero/work/workspace/Ficus/protos/kafka_service.proto

import type { Empty_DONTUSE as _google_protobuf_Empty_DONTUSE, Empty as _google_protobuf_Empty } from '../google/protobuf/Empty';
import type { GrpcT1StreamingConfiguration_DONTUSE as _ficus_GrpcT1StreamingConfiguration_DONTUSE, GrpcT1StreamingConfiguration as _ficus_GrpcT1StreamingConfiguration } from '../ficus/GrpcT1StreamingConfiguration';
import type { GrpcT2StreamingConfiguration_DONTUSE as _ficus_GrpcT2StreamingConfiguration_DONTUSE, GrpcT2StreamingConfiguration as _ficus_GrpcT2StreamingConfiguration } from '../ficus/GrpcT2StreamingConfiguration';

export interface GrpcPipelineStreamingConfiguration_DONTUSE {
  'notSpecified'?: (_google_protobuf_Empty_DONTUSE | null);
  't1Configuration'?: (_ficus_GrpcT1StreamingConfiguration_DONTUSE | null);
  't2Configuration'?: (_ficus_GrpcT2StreamingConfiguration_DONTUSE | null);
  'configuration'?: "notSpecified"|"t1Configuration"|"t2Configuration";
}

export interface GrpcPipelineStreamingConfiguration {
  'notSpecified'?: (_google_protobuf_Empty | null);
  't1Configuration'?: (_ficus_GrpcT1StreamingConfiguration | null);
  't2Configuration'?: (_ficus_GrpcT2StreamingConfiguration | null);
  'configuration': "notSpecified"|"t1Configuration"|"t2Configuration";
}
