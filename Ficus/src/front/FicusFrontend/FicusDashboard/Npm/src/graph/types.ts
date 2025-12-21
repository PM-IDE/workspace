import {GrpcNodeAdditionalData} from "../protos/ficus/GrpcNodeAdditionalData";
import {GrpcGraph} from "../protos/ficus/GrpcGraph";
import {GrpcTimelineDiagramFragment} from "../protos/ficus/GrpcTimelineDiagramFragment";
import {GrpcDurationKind} from "../protos/ficus/GrpcDurationKind";
import {GrpcGraphEdgeAdditionalData} from "../protos/ficus/GrpcGraphEdgeAdditionalData";

export interface CytoscapeElementDef<T> extends cytoscape.ElementDefinition {
  data: T
}

export interface GraphEntity {
  label: string,
  frontendId: number,
  id: string,
  enhancementData: MergedEnhancementData,
  aggregatedData: AggregatedData,
  executionTimeNs: number,
}

export interface GraphNode extends GraphEntity {
  innerGraph?: GrpcGraph,
  additionalData: GrpcNodeAdditionalData[],
}

export interface GraphEdge extends GraphEntity {
  source: string,
  target: string,
  weight: number,
  color: string,
  width: number,
  additionalData: GrpcGraphEdgeAdditionalData[],
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