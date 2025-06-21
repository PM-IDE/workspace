import {GrpcNodeAdditionalData} from "../protos/ficus/GrpcNodeAdditionalData";
import {GrpcGraph} from "../protos/ficus/GrpcGraph";
import {GrpcTimelineDiagramFragment} from "../protos/ficus/GrpcTimelineDiagramFragment";

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

export interface CountAndSum {
  count: number,
  sum: number
}

export interface MergedSoftwareData {
  histogram: Map<string, number>,
  timelineDiagramFragments: GrpcTimelineDiagramFragment[],
  allocations: Map<string, number>,

  inliningFailed: Map<string, number>,
  inliningSucceeded: Map<string, number>,
  inliningFailedReasons: Map<string, number>,

  methodsLoads: Map<string, number>,
  methodsUnloads: Map<string, number>,

  bufferAllocatedBytes: CountAndSum,
  bufferRentedBytes: CountAndSum,
  bufferReturnedBytes: CountAndSum,

  exceptions: Map<string, number>,

  createdThreads: Set<number>,
  terminatedThreads: Set<number>,

  httpRequests: Map<string, number>
}