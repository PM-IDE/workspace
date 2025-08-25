// Original file: ../../../../../protos/front_contract.proto

import type { GrpcPipelinePartContextValues_DONTUSE as _ficus_GrpcPipelinePartContextValues_DONTUSE, GrpcPipelinePartContextValues as _ficus_GrpcPipelinePartContextValues } from '../ficus/GrpcPipelinePartContextValues';
import type { Long } from '@grpc/proto-loader';

export interface GrpcCaseContextValues_DONTUSE {
  'contextValues'?: (_ficus_GrpcPipelinePartContextValues_DONTUSE)[];
  'stamp'?: (number | string | Long);
}

export interface GrpcCaseContextValues {
  'contextValues': (_ficus_GrpcPipelinePartContextValues)[];
  'stamp': (number);
}
