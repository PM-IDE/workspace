import {GrpcNodeAdditionalData} from "../protos/ficus/GrpcNodeAdditionalData";
import {GrpcGraph} from "../protos/ficus/GrpcGraph";
import {MergedSoftwareData} from "./util";

export interface GraphNode {
  frontendId: number,
  label: string,
  id: string,
  innerGraph?: GrpcGraph,
  executionTime: number,
  additionalData: GrpcNodeAdditionalData[],
  softwareData: MergedSoftwareData,
  aggregatedData: AggregatedData,
}

export interface GraphEdge {
  frontendId: number,
  additionalData: GrpcNodeAdditionalData[]
  softwareData: MergedSoftwareData,
  aggregatedData: AggregatedData,
  executionTime: number,
  weight: number
}

export interface AggregatedData {
  totalAllocatedBytes: number,
  totalExecutionTime: number,
  maxExecutionTime: number,

  totalBufferAllocatedBytes: number,
  totalBufferRentedBytes: number,
  totalBufferReturnedBytes: number,
}

export enum SoftwareEnhancementKind {
  Allocations,
  Exceptions,
  MethodsLoadUnload,
  MethodsInlinings,
  ArrayPools,
  Http,
  Sockets,
  Threads
}
