import {calculateGradient, createNextFrontendUniqueId} from "../utils";
import {darkTheme, graphColors} from "../colors";
import {
  calculateEdgeExecutionTime,
  calculateOverallExecutionTime, createEmptySoftwareData,
  getEdgeEnhancementDataOrNull,
  getNodeEnhancementDataOrNull,
  getPerformanceAnnotationColor, increment, preprocessForCSharpInterop, preprocessFromCSharpInterop,
} from "./util";
import {GrpcGraph} from "../protos/ficus/GrpcGraph";
import {GrpcAnnotation} from "../protos/ficus/GrpcAnnotation";
import {GrpcGraphNode} from "../protos/ficus/GrpcGraphNode";
import {GrpcGraphEdge} from "../protos/ficus/GrpcGraphEdge";
import cytoscape from "cytoscape";
import {
  AggregatedData, CytoscapeElementDef,
  GraphEdge, GraphNode,
  MergedSoftwareData,
  SoftwareEnhancementKind,
  ValueWithUnits
} from "./types";
import {createEdgeStandaloneEnhancements} from "./labels/edge_html_label";
import {createTimeSpanString} from "./labels/util";
import {GrpcDurationKind} from "../protos/ficus/GrpcDurationKind";
import {createNodeStandaloneEnhancements} from "./labels/node_html_label";

export function createGraphElements(
  graph: GrpcGraph,
  annotation: GrpcAnnotation,
  aggregatedData: AggregatedData,
  filter: RegExp | null
): cytoscape.ElementDefinition[] {
  let elements: cytoscape.ElementDefinition[] = [];

  let performanceEdgesMap = buildEdgesTimeAnnotationMap(annotation);

  elements.push(...createGraphNodesElements(graph.nodes, aggregatedData, filter));
  elements.push(...createGraphEdgesElements(graph.edges, performanceEdgesMap, aggregatedData, filter));

  return elements;
}

function createAggregatedDataInternal(graph: GrpcGraph, performanceMap: Record<number, any>, filter: RegExp | null) {
  let aggregatedData: AggregatedData = {
    totalExecutionTimeNs: 0,
    maxExecutionTime: Number.MIN_VALUE,

    globalSoftwareData: createEmptySoftwareData()
  };

  processNodesAggregatedData(graph.nodes, aggregatedData, filter);
  processEdgesAggregatedData(graph.edges, aggregatedData, performanceMap, filter);

  return aggregatedData;
}

export function createAggregatedData(graph: GrpcGraph, annotation: GrpcAnnotation, filter: string | null) {
  try {
    let performanceEdgesMap = buildEdgesTimeAnnotationMap(annotation);
    let regex = filter == null ? null : new RegExp(filter);

    return preprocessForCSharpInterop(createAggregatedDataInternal(graph, performanceEdgesMap, regex));
  } catch (e) {
    console.error(e);
    return null;
  }
}

function processNodesAggregatedData(nodes: GrpcGraphNode[], aggregatedData: AggregatedData, filter: RegExp | null) {
  for (let node of nodes) {
    let enhancementData = getNodeEnhancementDataOrNull(node, filter);
    updateAggregatedData(aggregatedData, enhancementData?.softwareData);

    let executionTime = calculateOverallExecutionTime(node);
    aggregatedData.totalExecutionTimeNs += executionTime;
    aggregatedData.maxExecutionTime = Math.max(aggregatedData.maxExecutionTime, executionTime);
  }
}

function processEdgesAggregatedData(edges: GrpcGraphEdge[], aggregatedData: AggregatedData, performanceMap: Record<number, any>, filter: RegExp | null) {
  for (let edge of edges) {
    let enhancementData = getEdgeEnhancementDataOrNull(edge, filter);
    updateAggregatedData(aggregatedData, enhancementData?.softwareData);

    let executionTime = performanceMap[edge.id] ?? calculateEdgeExecutionTime(edge);

    if (executionTime != null) {
      aggregatedData.totalExecutionTimeNs += executionTime;
      aggregatedData.maxExecutionTime = Math.max(executionTime, aggregatedData.maxExecutionTime);
    }
  }
}

function buildEdgesTimeAnnotationMap(annotation: GrpcAnnotation): Record<number, any> {
  let idsToTime: Record<number, any> = {};

  if (annotation?.timeAnnotation != null) {
    for (let timeAnnotation of annotation.timeAnnotation.annotations) {
      idsToTime[timeAnnotation.entityId] = timeAnnotation.interval.nanoseconds;
    }
  }

  return idsToTime;
}

export function createGraphNodesElements(
  nodes: GrpcGraphNode[],
  aggregatedData: AggregatedData,
  filter: RegExp | null
): CytoscapeElementDef<GraphNode>[] {
  let elements: CytoscapeElementDef<GraphNode>[] = [];

  for (let node of nodes) {
    elements.push({
      data: {
        frontendId: createNextFrontendUniqueId(),
        label: node.data,
        id: node.id.toString(),
        additionalData: node.additionalData,
        innerGraph: node.innerGraph,
        executionTimeNs: calculateOverallExecutionTime(node),
        enhancementData: getNodeEnhancementDataOrNull(node, filter),
        aggregatedData: aggregatedData
      }
    })
  }

  return elements;
}

function updateAggregatedData(aggregatedData: AggregatedData, softwareData: MergedSoftwareData) {
  if (softwareData != null) {
    for (let [name, histogram] of softwareData.histograms.entries()) {
      if (!aggregatedData.globalSoftwareData.histograms.has(name)) {
        aggregatedData.globalSoftwareData.histograms.set(name, {
          units: histogram.units,
          value: new Map<string, number>(),
          group: histogram.group,
        })
      }

      mergeMaps(aggregatedData.globalSoftwareData.histograms.get(name).value, histogram.value);
    }

    mergeSimpleMap(aggregatedData.globalSoftwareData.counters, softwareData.counters);

    for (let [name, duration] of softwareData.activitiesDurations.entries()) {
      if (!aggregatedData.globalSoftwareData.activitiesDurations.has(name)) {
        aggregatedData.globalSoftwareData.activitiesDurations.set(name, {
          units: duration.units,
          group: duration.group,
          value: {
            value: 0,
            kind: duration.value.kind
          }
        })
      }

      aggregatedData.globalSoftwareData.activitiesDurations.get(name).value.value += duration.value.value;
    }
  }
}

function mergeSimpleMap(to: Map<string, ValueWithUnits<number>>, from: Map<string, ValueWithUnits<number>>) {
  for (let [name, counter] of from.entries()) {
    if (!to.has(name)) {
      to.set(name, {
        units: counter.units,
        value: 0,
        group: counter.group,
      })
    }

    to.get(name).value += counter.value;
  }
}

function mergeMaps(first: Map<string, number>, second: Map<string, number>) {
  for (let [key, value] of second.entries()) {
    increment(first, key, value);
  }
}

export function createEnhancedEdges(
  graph: GrpcGraph,
  annotation: GrpcAnnotation,
  aggregatedData: AggregatedData,
  enhancements: SoftwareEnhancementKind[],
  filter: RegExp | null,
) {
  aggregatedData = preprocessFromCSharpInterop(aggregatedData);
  let performanceEdgesMap = buildEdgesTimeAnnotationMap(annotation);
  let elements = createGraphEdgesElements(graph.edges, performanceEdgesMap, aggregatedData, filter);

  return elements.map(e => {
    let enhancementHtml = "";
    if (e.data.enhancementData != null) {
      enhancementHtml = createEdgeStandaloneEnhancements(enhancements, e.data.enhancementData, aggregatedData);
    }

    return {
      id: Number.parseInt(e.data.id),
      html: enhancementHtml,
      color: e.data.color,
      executionTimeStringRepr: createTimeSpanString(e.data.executionTimeNs, GrpcDurationKind.Nanos),
      numberOfExecutions: e.data.weight
    }
  });
}

export function createEnhancedNodes(
  graph: GrpcGraph,
  aggregatedData: AggregatedData,
  enhancements: SoftwareEnhancementKind[],
  filter: RegExp | null,
) {
  aggregatedData = preprocessFromCSharpInterop(aggregatedData);
  let elements = createGraphNodesElements(graph.nodes, aggregatedData, filter);

  return elements.map(e => {
    let enhancementHtml = null;
    if (e.data.enhancementData != null) {
      enhancementHtml = createNodeStandaloneEnhancements(enhancements, e.data.enhancementData, aggregatedData);
    }

    return {
      id: Number.parseInt(e.data.id),
      html: enhancementHtml,
      color: getPerformanceAnnotationColor(e.data.executionTimeNs / aggregatedData.totalExecutionTimeNs),
      executionTimeStringRepr: createTimeSpanString(e.data.executionTimeNs, GrpcDurationKind.Nanos),
    }
  });
}

function createGraphEdgesElements(
  edges: GrpcGraphEdge[],
  performanceMap: Record<number, any>,
  aggregatedData: AggregatedData,
  filter: RegExp | null
): CytoscapeElementDef<GraphEdge>[] {
  let elements: CytoscapeElementDef<GraphEdge>[] = [];

  let maxWeight = Math.max(...edges.map(e => e.weight));
  const minWidth = 5;
  const maxWidth = 20;

  for (let edge of edges) {
    let weightRatio = edge.weight / maxWeight
    let width = minWidth + (maxWidth - minWidth) * weightRatio;

    if (isNaN(width)) {
      width = 1;
    }

    let executionTime = performanceMap[edge.id] ?? calculateEdgeExecutionTime(edge);
    let color = getPerformanceAnnotationColor(executionTime / aggregatedData.totalExecutionTimeNs);

    elements.push({
      data: {
        frontendId: createNextFrontendUniqueId(),
        label: edge.data,
        id: edge.id.toString(),
        source: edge.fromNode.toString(),
        target: edge.toNode.toString(),
        additionalData: edge.additionalData,
        enhancementData: getEdgeEnhancementDataOrNull(edge, filter),
        executionTimeNs: executionTime,
        weight: edge.weight,
        width: width,
        color: color,
        aggregatedData: aggregatedData
      }
    })
  }

  return elements;
}
