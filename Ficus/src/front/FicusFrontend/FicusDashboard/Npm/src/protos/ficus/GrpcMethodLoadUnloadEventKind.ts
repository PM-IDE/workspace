// Original file: /Users/aero/work/workspace/Ficus/protos/pipelines_and_context.proto

export const GrpcMethodLoadUnloadEventKind = {
  Load: 'Load',
  Unload: 'Unload',
} as const;

export type GrpcMethodLoadUnloadEventKind_DONTUSE =
  | 'Load'
  | 0
  | 'Unload'
  | 1

export type GrpcMethodLoadUnloadEventKind = typeof GrpcMethodLoadUnloadEventKind[keyof typeof GrpcMethodLoadUnloadEventKind]
