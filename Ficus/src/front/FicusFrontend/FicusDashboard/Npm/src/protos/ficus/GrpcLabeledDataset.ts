// Original file: ../../../../../protos/pm_models.proto

import type { GrpcDataset_DONTUSE as _ficus_GrpcDataset_DONTUSE, GrpcDataset as _ficus_GrpcDataset } from '../ficus/GrpcDataset';
import type { GrpcColor_DONTUSE as _ficus_GrpcColor_DONTUSE, GrpcColor as _ficus_GrpcColor } from '../ficus/GrpcColor';

export interface GrpcLabeledDataset_DONTUSE {
  'dataset'?: (_ficus_GrpcDataset_DONTUSE | null);
  'labels'?: (number)[];
  'labelsColors'?: (_ficus_GrpcColor_DONTUSE)[];
}

export interface GrpcLabeledDataset {
  'dataset': (_ficus_GrpcDataset | null);
  'labels': (number)[];
  'labelsColors': (_ficus_GrpcColor)[];
}
