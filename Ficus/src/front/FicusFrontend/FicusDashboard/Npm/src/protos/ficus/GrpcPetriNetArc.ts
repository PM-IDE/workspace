// Original file: /Users/aero/work/workspace/Ficus/protos/pm_models.proto

import type { Long } from '@grpc/proto-loader';

export interface GrpcPetriNetArc_DONTUSE {
  'id'?: (number | string | Long);
  'placeId'?: (number | string | Long);
  'tokensCount'?: (number | string | Long);
}

export interface GrpcPetriNetArc {
  'id': (number);
  'placeId': (number);
  'tokensCount': (number);
}
