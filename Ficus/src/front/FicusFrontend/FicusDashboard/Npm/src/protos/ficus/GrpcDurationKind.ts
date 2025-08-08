// Original file: /Users/aero/work/workspace/Ficus/protos/pipelines_and_context.proto

export const GrpcDurationKind = {
  Unspecified: 0,
  Nanos: 1,
  Micros: 2,
  Millis: 3,
  Seconds: 4,
  Minutes: 5,
  Hours: 6,
  Days: 7,
} as const;

export type GrpcDurationKind_DONTUSE =
  | 'Unspecified'
  | 0
  | 'Nanos'
  | 1
  | 'Micros'
  | 2
  | 'Millis'
  | 3
  | 'Seconds'
  | 4
  | 'Minutes'
  | 5
  | 'Hours'
  | 6
  | 'Days'
  | 7

export type GrpcDurationKind = typeof GrpcDurationKind[keyof typeof GrpcDurationKind]
