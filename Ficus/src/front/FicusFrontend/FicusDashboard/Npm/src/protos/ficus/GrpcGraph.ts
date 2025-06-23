// Original file: /Users/aero/work/workspace/Ficus/protos/pipelines_and_context.proto

import type { GrpcGraphNode_DONTUSE as _ficus_GrpcGraphNode_DONTUSE, GrpcGraphNode as _ficus_GrpcGraphNode } from '../ficus/GrpcGraphNode';
import type { GrpcGraphEdge_DONTUSE as _ficus_GrpcGraphEdge_DONTUSE, GrpcGraphEdge as _ficus_GrpcGraphEdge } from '../ficus/GrpcGraphEdge';
import type { GrpcGraphKind_DONTUSE as _ficus_GrpcGraphKind_DONTUSE, GrpcGraphKind as _ficus_GrpcGraphKind } from '../ficus/GrpcGraphKind';

export interface GrpcGraph_DONTUSE {
  'nodes'?: (_ficus_GrpcGraphNode_DONTUSE)[];
  'edges'?: (_ficus_GrpcGraphEdge_DONTUSE)[];
  'kind'?: (_ficus_GrpcGraphKind_DONTUSE);
}

export interface GrpcGraph {
  'nodes': (_ficus_GrpcGraphNode)[];
  'edges': (_ficus_GrpcGraphEdge)[];
  'kind': (_ficus_GrpcGraphKind);
}
