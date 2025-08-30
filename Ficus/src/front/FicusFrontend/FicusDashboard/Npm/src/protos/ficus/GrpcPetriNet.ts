// Original file: ../../../../../protos/pm_models.proto

import type { GrpcPetriNetPlace_DONTUSE as _ficus_GrpcPetriNetPlace_DONTUSE, GrpcPetriNetPlace as _ficus_GrpcPetriNetPlace } from '../ficus/GrpcPetriNetPlace';
import type { GrpcPetriNetTransition_DONTUSE as _ficus_GrpcPetriNetTransition_DONTUSE, GrpcPetriNetTransition as _ficus_GrpcPetriNetTransition } from '../ficus/GrpcPetriNetTransition';
import type { GrpcPetriNetMarking_DONTUSE as _ficus_GrpcPetriNetMarking_DONTUSE, GrpcPetriNetMarking as _ficus_GrpcPetriNetMarking } from '../ficus/GrpcPetriNetMarking';

export interface GrpcPetriNet_DONTUSE {
  'places'?: (_ficus_GrpcPetriNetPlace_DONTUSE)[];
  'transitions'?: (_ficus_GrpcPetriNetTransition_DONTUSE)[];
  'initialMarking'?: (_ficus_GrpcPetriNetMarking_DONTUSE | null);
  'finalMarking'?: (_ficus_GrpcPetriNetMarking_DONTUSE | null);
}

export interface GrpcPetriNet {
  'places': (_ficus_GrpcPetriNetPlace)[];
  'transitions': (_ficus_GrpcPetriNetTransition)[];
  'initialMarking': (_ficus_GrpcPetriNetMarking | null);
  'finalMarking': (_ficus_GrpcPetriNetMarking | null);
}
