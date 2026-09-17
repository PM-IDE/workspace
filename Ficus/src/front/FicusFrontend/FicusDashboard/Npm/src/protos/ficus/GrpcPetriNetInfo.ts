// Original file: ../../../../../protos/pm_models.proto

import type { Long } from '@grpc/proto-loader';

export interface GrpcPetriNetInfo_DONTUSE {
  'placesCount'?: (number | string | Long);
  'transitionsCount'?: (number | string | Long);
  'arcsCount'?: (number | string | Long);
}

export interface GrpcPetriNetInfo {
  'placesCount': (number);
  'transitionsCount': (number);
  'arcsCount': (number);
}
