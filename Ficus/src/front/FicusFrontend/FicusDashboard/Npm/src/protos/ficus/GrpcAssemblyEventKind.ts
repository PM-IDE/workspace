// Original file: /Users/aero/work/workspace/Ficus/protos/pipelines_and_context.proto

export const GrpcAssemblyEventKind = {
  Loaded: 'Loaded',
  Unloaded: 'Unloaded',
} as const;

export type GrpcAssemblyEventKind_DONTUSE =
  | 'Loaded'
  | 0
  | 'Unloaded'
  | 1

export type GrpcAssemblyEventKind = typeof GrpcAssemblyEventKind[keyof typeof GrpcAssemblyEventKind]
