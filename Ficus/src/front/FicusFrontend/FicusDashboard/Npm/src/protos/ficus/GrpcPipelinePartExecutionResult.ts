// Original file: /Users/aero/work/workspace/Ficus/protos/backend_service.proto

import type { GrpcPipelineFinalResult_DONTUSE as _ficus_GrpcPipelineFinalResult_DONTUSE, GrpcPipelineFinalResult as _ficus_GrpcPipelineFinalResult } from '../ficus/GrpcPipelineFinalResult';
import type { GrpcPipelinePartResult_DONTUSE as _ficus_GrpcPipelinePartResult_DONTUSE, GrpcPipelinePartResult as _ficus_GrpcPipelinePartResult } from '../ficus/GrpcPipelinePartResult';
import type { GrpcPipelinePartLogMessage_DONTUSE as _ficus_GrpcPipelinePartLogMessage_DONTUSE, GrpcPipelinePartLogMessage as _ficus_GrpcPipelinePartLogMessage } from '../ficus/GrpcPipelinePartLogMessage';

export interface GrpcPipelinePartExecutionResult_DONTUSE {
  'finalResult'?: (_ficus_GrpcPipelineFinalResult_DONTUSE | null);
  'pipelinePartResult'?: (_ficus_GrpcPipelinePartResult_DONTUSE | null);
  'logMessage'?: (_ficus_GrpcPipelinePartLogMessage_DONTUSE | null);
  'result'?: "finalResult"|"pipelinePartResult"|"logMessage";
}

export interface GrpcPipelinePartExecutionResult {
  'finalResult'?: (_ficus_GrpcPipelineFinalResult | null);
  'pipelinePartResult'?: (_ficus_GrpcPipelinePartResult | null);
  'logMessage'?: (_ficus_GrpcPipelinePartLogMessage | null);
  'result': "finalResult"|"pipelinePartResult"|"logMessage";
}
