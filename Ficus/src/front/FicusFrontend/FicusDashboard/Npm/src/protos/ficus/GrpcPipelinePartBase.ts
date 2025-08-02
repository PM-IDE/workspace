// Original file: ../../../../../protos/pipelines_and_context.proto

import type { GrpcPipelinePart_DONTUSE as _ficus_GrpcPipelinePart_DONTUSE, GrpcPipelinePart as _ficus_GrpcPipelinePart } from '../ficus/GrpcPipelinePart';
import type { GrpcParallelPipelinePart_DONTUSE as _ficus_GrpcParallelPipelinePart_DONTUSE, GrpcParallelPipelinePart as _ficus_GrpcParallelPipelinePart } from '../ficus/GrpcParallelPipelinePart';
import type { GrpcSimpleContextRequestPipelinePart_DONTUSE as _ficus_GrpcSimpleContextRequestPipelinePart_DONTUSE, GrpcSimpleContextRequestPipelinePart as _ficus_GrpcSimpleContextRequestPipelinePart } from '../ficus/GrpcSimpleContextRequestPipelinePart';
import type { GrpcComplexContextRequestPipelinePart_DONTUSE as _ficus_GrpcComplexContextRequestPipelinePart_DONTUSE, GrpcComplexContextRequestPipelinePart as _ficus_GrpcComplexContextRequestPipelinePart } from '../ficus/GrpcComplexContextRequestPipelinePart';

export interface GrpcPipelinePartBase_DONTUSE {
  'defaultPart'?: (_ficus_GrpcPipelinePart_DONTUSE | null);
  'parallelPart'?: (_ficus_GrpcParallelPipelinePart_DONTUSE | null);
  'simpleContextRequestPart'?: (_ficus_GrpcSimpleContextRequestPipelinePart_DONTUSE | null);
  'complexContextRequestPart'?: (_ficus_GrpcComplexContextRequestPipelinePart_DONTUSE | null);
  'part'?: "defaultPart"|"parallelPart"|"simpleContextRequestPart"|"complexContextRequestPart";
}

export interface GrpcPipelinePartBase {
  'defaultPart'?: (_ficus_GrpcPipelinePart | null);
  'parallelPart'?: (_ficus_GrpcParallelPipelinePart | null);
  'simpleContextRequestPart'?: (_ficus_GrpcSimpleContextRequestPipelinePart | null);
  'complexContextRequestPart'?: (_ficus_GrpcComplexContextRequestPipelinePart | null);
  'part': "defaultPart"|"parallelPart"|"simpleContextRequestPart"|"complexContextRequestPart";
}
