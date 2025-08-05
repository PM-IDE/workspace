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
  enhancementData: MergedEnhancementData,
  aggregatedData: AggregatedData,
}

export interface GraphEdge {
  frontendId: number,
  additionalData: GrpcNodeAdditionalData[]
  enhancementData: MergedEnhancementData,
  aggregatedData: AggregatedData,
  executionTime: number,
  weight: number
}

export interface AggregatedData {
  totalExecutionTime: number,
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

export interface MergedSoftwareData {
  histograms: Map<string, ValueWithUnits<Map<string, number>>>,
  counters: Map<string, ValueWithUnits<number>>,
  activitiesDurations: Map<string, ValueWithUnits<number>>
}

export interface MergedEnhancementData {
  eventClasses: Map<string, number>,
  timelineDiagramFragments: GrpcTimelineDiagramFragment[],
  softwareData: MergedSoftwareData
} 