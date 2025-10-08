// Original file: ../../../../../protos/pipelines_and_context.proto

import type { Empty_DONTUSE as _google_protobuf_Empty_DONTUSE, Empty as _google_protobuf_Empty } from '../google/protobuf/Empty';
import type { GrpcMergedObjectAllocation_DONTUSE as _ficus_GrpcMergedObjectAllocation_DONTUSE, GrpcMergedObjectAllocation as _ficus_GrpcMergedObjectAllocation } from '../ficus/GrpcMergedObjectAllocation';
import type { GrpcProduceObjectConsumption_DONTUSE as _ficus_GrpcProduceObjectConsumption_DONTUSE, GrpcProduceObjectConsumption as _ficus_GrpcProduceObjectConsumption } from '../ficus/GrpcProduceObjectConsumption';

export interface GrpcOcelData_DONTUSE {
  'objectType'?: (string);
  'objectId'?: (string);
  'allocate'?: (_google_protobuf_Empty_DONTUSE | null);
  'consume'?: (_google_protobuf_Empty_DONTUSE | null);
  'mergedObjectAllocation'?: (_ficus_GrpcMergedObjectAllocation_DONTUSE | null);
  'produceObjectConsumption'?: (_ficus_GrpcProduceObjectConsumption_DONTUSE | null);
  'action'?: "allocate"|"consume"|"mergedObjectAllocation"|"produceObjectConsumption";
}

export interface GrpcOcelData {
  'objectType': (string);
  'objectId': (string);
  'allocate'?: (_google_protobuf_Empty | null);
  'consume'?: (_google_protobuf_Empty | null);
  'mergedObjectAllocation'?: (_ficus_GrpcMergedObjectAllocation | null);
  'produceObjectConsumption'?: (_ficus_GrpcProduceObjectConsumption | null);
  'action': "allocate"|"consume"|"mergedObjectAllocation"|"produceObjectConsumption";
}
