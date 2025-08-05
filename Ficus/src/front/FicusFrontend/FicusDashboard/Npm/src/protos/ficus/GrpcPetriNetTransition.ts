// Original file: /Users/aero/work/workspace/Ficus/protos/pm_models.proto

import type { GrpcPetriNetArc_DONTUSE as _ficus_GrpcPetriNetArc_DONTUSE, GrpcPetriNetArc as _ficus_GrpcPetriNetArc } from '../ficus/GrpcPetriNetArc';
import type { Long } from '@grpc/proto-loader';

export interface GrpcPetriNetTransition_DONTUSE {
  'id'?: (number | string | Long);
  'incomingArcs'?: (_ficus_GrpcPetriNetArc_DONTUSE)[];
  'outgoingArcs'?: (_ficus_GrpcPetriNetArc_DONTUSE)[];
  'data'?: (string);
}

export interface GrpcPetriNetTransition {
  'id': (number);
  'incomingArcs': (_ficus_GrpcPetriNetArc)[];
  'outgoingArcs': (_ficus_GrpcPetriNetArc)[];
  'data': (string);
}
