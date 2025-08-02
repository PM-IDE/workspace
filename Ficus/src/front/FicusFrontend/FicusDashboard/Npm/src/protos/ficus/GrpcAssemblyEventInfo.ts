// Original file: ../../../../../protos/pipelines_and_context.proto

import type { GrpcAssemblyEventKind_DONTUSE as _ficus_GrpcAssemblyEventKind_DONTUSE, GrpcAssemblyEventKind as _ficus_GrpcAssemblyEventKind } from '../ficus/GrpcAssemblyEventKind';

export interface GrpcAssemblyEventInfo_DONTUSE {
  'assemblyName'?: (string);
  'eventKind'?: (_ficus_GrpcAssemblyEventKind_DONTUSE);
}

export interface GrpcAssemblyEventInfo {
  'assemblyName': (string);
  'eventKind': (_ficus_GrpcAssemblyEventKind);
}
