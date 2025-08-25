// Original file: ../../../../../protos/front_contract.proto

import type { GrpcProcessCaseMetadata_DONTUSE as _ficus_GrpcProcessCaseMetadata_DONTUSE, GrpcProcessCaseMetadata as _ficus_GrpcProcessCaseMetadata } from '../ficus/GrpcProcessCaseMetadata';
import type { Long } from '@grpc/proto-loader';

export interface GrpcProcessCaseMetadataWithStamp_DONTUSE {
  'stamp'?: (number | string | Long);
  'metadata'?: (_ficus_GrpcProcessCaseMetadata_DONTUSE | null);
}

export interface GrpcProcessCaseMetadataWithStamp {
  'stamp': (number);
  'metadata': (_ficus_GrpcProcessCaseMetadata | null);
}
