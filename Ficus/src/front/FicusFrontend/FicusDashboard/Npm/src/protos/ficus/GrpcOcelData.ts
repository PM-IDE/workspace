// Original file: ../../../../../protos/pipelines_and_context.proto

import type { GrpcOcelObjectTypeData_DONTUSE as _ficus_GrpcOcelObjectTypeData_DONTUSE, GrpcOcelObjectTypeData as _ficus_GrpcOcelObjectTypeData } from '../ficus/GrpcOcelObjectTypeData';
import type { GrpcOcelAllocateMerge_DONTUSE as _ficus_GrpcOcelAllocateMerge_DONTUSE, GrpcOcelAllocateMerge as _ficus_GrpcOcelAllocateMerge } from '../ficus/GrpcOcelAllocateMerge';
import type { GrpcOcelConsumeProduce_DONTUSE as _ficus_GrpcOcelConsumeProduce_DONTUSE, GrpcOcelConsumeProduce as _ficus_GrpcOcelConsumeProduce } from '../ficus/GrpcOcelConsumeProduce';

export interface GrpcOcelData_DONTUSE {
  'objectId'?: (string);
  'allocate'?: (_ficus_GrpcOcelObjectTypeData_DONTUSE | null);
  'consume'?: (_ficus_GrpcOcelObjectTypeData_DONTUSE | null);
  'mergedObjectAllocation'?: (_ficus_GrpcOcelAllocateMerge_DONTUSE | null);
  'produceObjectConsumption'?: (_ficus_GrpcOcelConsumeProduce_DONTUSE | null);
  'action'?: "allocate"|"consume"|"mergedObjectAllocation"|"produceObjectConsumption";
}

export interface GrpcOcelData {
  'objectId': (string);
  'allocate'?: (_ficus_GrpcOcelObjectTypeData | null);
  'consume'?: (_ficus_GrpcOcelObjectTypeData | null);
  'mergedObjectAllocation'?: (_ficus_GrpcOcelAllocateMerge | null);
  'produceObjectConsumption'?: (_ficus_GrpcOcelConsumeProduce | null);
  'action': "allocate"|"consume"|"mergedObjectAllocation"|"produceObjectConsumption";
}
