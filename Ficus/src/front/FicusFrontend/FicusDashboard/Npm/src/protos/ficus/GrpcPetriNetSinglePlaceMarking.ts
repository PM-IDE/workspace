// Original file: /Users/aero/work/workspace/Ficus/protos/pm_models.proto

import type { Long } from '@grpc/proto-loader';

export interface GrpcPetriNetSinglePlaceMarking_DONTUSE {
  'placeId'?: (number | string | Long);
  'tokensCount'?: (number | string | Long);
}

export interface GrpcPetriNetSinglePlaceMarking {
  'placeId': (number);
  'tokensCount': (number);
}
