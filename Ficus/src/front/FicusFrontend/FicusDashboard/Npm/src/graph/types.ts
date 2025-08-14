import {GrpcNodeAdditionalData} from "../protos/ficus/GrpcNodeAdditionalData";
import {GrpcGraph} from "../protos/ficus/GrpcGraph";
import {GrpcTimelineDiagramFragment} from "../protos/ficus/GrpcTimelineDiagramFragment";
import {GrpcDurationKind} from "../protos/ficus/GrpcDurationKind";

export interface GraphNode {
  frontendId: number,
  label: string,
  id: string,
  innerGraph?: GrpcGraph,
  executionTimeNs: number,
  additionalData: GrpcNodeAdditionalData[],
  enhancementData: MergedEnhancementData,
  aggregatedData: AggregatedData,
}

export interface GraphEdge {
  frontendId: number,
  additionalData: GrpcNodeAdditionalData[]
  enhancementData: MergedEnhancementData,
  aggregatedData: AggregatedData,
  executionTimeNs: number,
  weight: number
}

export interface AggregatedData {
  totalExecutionTimeNs: number,
  maxExecutionTime: number,

  globalSoftwareData: MergedSoftwareData
}

export type SoftwareEnhancementKind = string

export interface CountAndSum {
  count: number,
  sum: number
}

export interface ValueWithUnits<T> {
  value: T
  units: string,
  group: string | null
}

export interface Duration {
  value: number,
  kind: GrpcDurationKind,
}

export interface MergedSoftwareData {
  histograms: Map<string, ValueWithUnits<Map<string, number>>>,
  counters: Map<string, ValueWithUnits<number>>,
  activitiesDurations: Map<string, ValueWithUnits<Duration>>
}

export interface MergedEnhancementData {
  eventClasses: Map<string, number>,
  timelineDiagramFragments: GrpcTimelineDiagramFragment[],
  softwareData: MergedSoftwareData
} 