// Original file: /Users/aero/work/workspace/Ficus/protos/pipelines_and_context.proto

import type { GrpcGraphNode_DONTUSE as _ficus_GrpcGraphNode_DONTUSE, GrpcGraphNode as _ficus_GrpcGraphNode } from '../ficus/GrpcGraphNode';
import type { GrpcGraphEdge_DONTUSE as _ficus_GrpcGraphEdge_DONTUSE, GrpcGraphEdge as _ficus_GrpcGraphEdge } from '../ficus/GrpcGraphEdge';

export interface GrpcGraph_DONTUSE {
  'nodes'?: (_ficus_GrpcGraphNode_DONTUSE)[];
  'edges'?: (_ficus_GrpcGraphEdge_DONTUSE)[];
}

export interface GrpcGraph {
  'nodes': (_ficus_GrpcGraphNode)[];
  'edges': (_ficus_GrpcGraphEdge)[];
}
