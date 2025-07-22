// Original file: /Users/aero/work/workspace/Ficus/protos/pipelines_and_context.proto

export const GrpcThreadEventKind = {
  Created: 'Created',
  Terminated: 'Terminated',
} as const;

export type GrpcThreadEventKind_DONTUSE =
  | 'Created'
  | 0
  | 'Terminated'
  | 1

export type GrpcThreadEventKind = typeof GrpcThreadEventKind[keyof typeof GrpcThreadEventKind]
