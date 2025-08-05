// Original file: /Users/aero/work/workspace/Ficus/protos/pipelines_and_context.proto

export const GrpcGraphKind = {
  None: 0,
  DAG: 1,
} as const;

export type GrpcGraphKind_DONTUSE =
  | 'None'
  | 0
  | 'DAG'
  | 1

export type GrpcGraphKind = typeof GrpcGraphKind[keyof typeof GrpcGraphKind]
