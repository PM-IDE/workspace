// Original file: /Users/aero/work/workspace/Ficus/protos/pipelines_and_context.proto

import type { GrpcSocketConnectStart_DONTUSE as _ficus_GrpcSocketConnectStart_DONTUSE, GrpcSocketConnectStart as _ficus_GrpcSocketConnectStart } from '../ficus/GrpcSocketConnectStart';
import type { GrpcSocketAcceptStart_DONTUSE as _ficus_GrpcSocketAcceptStart_DONTUSE, GrpcSocketAcceptStart as _ficus_GrpcSocketAcceptStart } from '../ficus/GrpcSocketAcceptStart';
import type { GrpcSocketConnectStop_DONTUSE as _ficus_GrpcSocketConnectStop_DONTUSE, GrpcSocketConnectStop as _ficus_GrpcSocketConnectStop } from '../ficus/GrpcSocketConnectStop';
import type { GrpcSocketAcceptStop_DONTUSE as _ficus_GrpcSocketAcceptStop_DONTUSE, GrpcSocketAcceptStop as _ficus_GrpcSocketAcceptStop } from '../ficus/GrpcSocketAcceptStop';
import type { GrpcSocketConnectFailed_DONTUSE as _ficus_GrpcSocketConnectFailed_DONTUSE, GrpcSocketConnectFailed as _ficus_GrpcSocketConnectFailed } from '../ficus/GrpcSocketConnectFailed';
import type { GrpcSocketAcceptFailed_DONTUSE as _ficus_GrpcSocketAcceptFailed_DONTUSE, GrpcSocketAcceptFailed as _ficus_GrpcSocketAcceptFailed } from '../ficus/GrpcSocketAcceptFailed';

export interface GrpcSocketEvent_DONTUSE {
  'connectStart'?: (_ficus_GrpcSocketConnectStart_DONTUSE | null);
  'acceptStart'?: (_ficus_GrpcSocketAcceptStart_DONTUSE | null);
  'connectStop'?: (_ficus_GrpcSocketConnectStop_DONTUSE | null);
  'acceptStop'?: (_ficus_GrpcSocketAcceptStop_DONTUSE | null);
  'connectFailed'?: (_ficus_GrpcSocketConnectFailed_DONTUSE | null);
  'acceptFailed'?: (_ficus_GrpcSocketAcceptFailed_DONTUSE | null);
  'event'?: "connectStart"|"acceptStart"|"connectStop"|"acceptStop"|"connectFailed"|"acceptFailed";
}

export interface GrpcSocketEvent {
  'connectStart'?: (_ficus_GrpcSocketConnectStart | null);
  'acceptStart'?: (_ficus_GrpcSocketAcceptStart | null);
  'connectStop'?: (_ficus_GrpcSocketConnectStop | null);
  'acceptStop'?: (_ficus_GrpcSocketAcceptStop | null);
  'connectFailed'?: (_ficus_GrpcSocketConnectFailed | null);
  'acceptFailed'?: (_ficus_GrpcSocketAcceptFailed | null);
  'event': "connectStart"|"acceptStart"|"connectStop"|"acceptStop"|"connectFailed"|"acceptFailed";
}
