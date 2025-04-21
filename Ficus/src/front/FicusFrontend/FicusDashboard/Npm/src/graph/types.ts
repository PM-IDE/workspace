import {GrpcNodeAdditionalData} from "../protos/ficus/GrpcNodeAdditionalData";
import {GrpcGraph} from "../protos/ficus/GrpcGraph";

export interface GraphNode {
  label: string,
  id: string,
  additionalData: GrpcNodeAdditionalData[],
  innerGraph?: GrpcGraph,
  executionTime: number,
  relativeExecutionTime: number,
}

export interface GraphEdge {
  additionalData: GrpcNodeAdditionalData[]
}