// Original file: ../../../../../protos/pm_models.proto

import type { GrpcMatrix_DONTUSE as _ficus_GrpcMatrix_DONTUSE, GrpcMatrix as _ficus_GrpcMatrix } from '../ficus/GrpcMatrix';

export interface GrpcDataset_DONTUSE {
  'matrix'?: (_ficus_GrpcMatrix_DONTUSE | null);
  'columnsNames'?: (string)[];
  'rowNames'?: (string)[];
}

export interface GrpcDataset {
  'matrix': (_ficus_GrpcMatrix | null);
  'columnsNames': (string)[];
  'rowNames': (string)[];
}
