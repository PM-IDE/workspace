// Original file: ../../../../../protos/backend_service.proto

import type { GrpcPipelinePartDescriptor_DONTUSE as _ficus_GrpcPipelinePartDescriptor_DONTUSE, GrpcPipelinePartDescriptor as _ficus_GrpcPipelinePartDescriptor } from '../ficus/GrpcPipelinePartDescriptor';

export interface GrpcFicusBackendInfo_DONTUSE {
  'name'?: (string);
  'pipelineParts'?: (_ficus_GrpcPipelinePartDescriptor_DONTUSE)[];
}

export interface GrpcFicusBackendInfo {
  'name': (string);
  'pipelineParts': (_ficus_GrpcPipelinePartDescriptor)[];
}
