// Original file: /Users/aero/work/workspace/Ficus/protos/pipelines_and_context.proto

export const GrpcArrayPoolEventKind = {
  Allocated: 'Allocated',
  Rented: 'Rented',
  Returned: 'Returned',
  Trimmed: 'Trimmed',
} as const;

export type GrpcArrayPoolEventKind_DONTUSE =
  | 'Allocated'
  | 0
  | 'Rented'
  | 1
  | 'Returned'
  | 2
  | 'Trimmed'
  | 3

export type GrpcArrayPoolEventKind = typeof GrpcArrayPoolEventKind[keyof typeof GrpcArrayPoolEventKind]
