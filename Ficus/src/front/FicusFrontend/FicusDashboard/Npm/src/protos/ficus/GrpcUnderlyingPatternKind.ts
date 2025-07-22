// Original file: /Users/aero/work/workspace/Ficus/protos/pipelines_and_context.proto

export const GrpcUnderlyingPatternKind = {
  StrictLoop: 'StrictLoop',
  PrimitiveTandemArray: 'PrimitiveTandemArray',
  MaximalTandemArray: 'MaximalTandemArray',
  MaximalRepeat: 'MaximalRepeat',
  SuperMaximalRepeat: 'SuperMaximalRepeat',
  NearSuperMaximalRepeat: 'NearSuperMaximalRepeat',
  Unknown: 'Unknown',
} as const;

export type GrpcUnderlyingPatternKind_DONTUSE =
  | 'StrictLoop'
  | 0
  | 'PrimitiveTandemArray'
  | 1
  | 'MaximalTandemArray'
  | 2
  | 'MaximalRepeat'
  | 3
  | 'SuperMaximalRepeat'
  | 4
  | 'NearSuperMaximalRepeat'
  | 5
  | 'Unknown'
  | 6

export type GrpcUnderlyingPatternKind = typeof GrpcUnderlyingPatternKind[keyof typeof GrpcUnderlyingPatternKind]
