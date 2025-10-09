// Original file: ../../../../../protos/pipelines_and_context.proto

import type { GrpcOcelState_DONTUSE as _ficus_GrpcOcelState_DONTUSE, GrpcOcelState as _ficus_GrpcOcelState } from '../ficus/GrpcOcelState';
import type { Long } from '@grpc/proto-loader';

export interface GrpcModelElementOcelAnnotation_DONTUSE {
  'elementId'?: (number | string | Long);
  'initialState'?: (_ficus_GrpcOcelState_DONTUSE | null);
  'finalState'?: (_ficus_GrpcOcelState_DONTUSE | null);
  '_initialState'?: "initialState";
}

export interface GrpcModelElementOcelAnnotation {
  'elementId': (number);
  'initialState'?: (_ficus_GrpcOcelState | null);
  'finalState': (_ficus_GrpcOcelState | null);
  '_initialState': "initialState";
}
