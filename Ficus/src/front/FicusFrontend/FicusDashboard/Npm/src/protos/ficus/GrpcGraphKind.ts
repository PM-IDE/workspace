// Original file: ../../../../../protos/pipelines_and_context.proto

export const GrpcGraphKind = {
  None: 0,
  DAG: 1,
  DagLCS: 2,
} as const;

export type GrpcGraphKind_DONTUSE =
  | 'None'
  | 0
  | 'DAG'
  | 1
  | 'DagLCS'
  | 2

export type GrpcGraphKind = typeof GrpcGraphKind[keyof typeof GrpcGraphKind]
