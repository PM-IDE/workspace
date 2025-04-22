import {GrpcNodeAdditionalData} from "../protos/ficus/GrpcNodeAdditionalData";
import {GrpcGraph} from "../protos/ficus/GrpcGraph";
import {MergedSoftwareData} from "./util";

export interface GraphNode {
  label: string,
  id: string,
  innerGraph?: GrpcGraph,
  executionTime: number,
  additionalData: GrpcNodeAdditionalData[],
  softwareData: MergedSoftwareData,
  aggregatedData: AggregatedData,
}

export interface GraphEdge {
  additionalData: GrpcNodeAdditionalData[]
  softwareData: MergedSoftwareData,
  aggregatedData: AggregatedData
}

export interface AggregatedData {
  totalAllocatedBytes: number,
  maxNodeExecutionTime: number
}
