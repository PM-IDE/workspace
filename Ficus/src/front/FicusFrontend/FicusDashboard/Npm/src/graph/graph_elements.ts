import {calculateGradient, createNextFrontendUniqueId} from "../utils";
import {darkTheme, graphColors} from "../colors";
import {
  calculateEdgeExecutionTime,
  calculateOverallExecutionTime, createEmptySoftwareData,
  getEdgeEnhancementDataOrNull,
  getNodeEnhancementDataOrNull,
  getPerformanceAnnotationColor, increment,
} from "./util";
import {GrpcGraph} from "../protos/ficus/GrpcGraph";
import {GrpcAnnotation} from "../protos/ficus/GrpcAnnotation";
import {GrpcGraphNode} from "../protos/ficus/GrpcGraphNode";
import {GrpcGraphEdge} from "../protos/ficus/GrpcGraphEdge";
import cytoscape from "cytoscape";
import {AggregatedData, CountAndSum, MergedSoftwareData, ValueWithUnits} from "./types";

const graphColor = graphColors(darkTheme);

export function createGraphElements(
  graph: GrpcGraph,
  annotation: GrpcAnnotation,
  aggregatedData: AggregatedData,
  filter: RegExp | null
): cytoscape.ElementDefinition[] {
  let elements: cytoscape.ElementDefinition[] = [];

  let performanceEdgesMap = buildEdgesTimeAnnotationMap(annotation);

  elements.push(...createGraphNodesElements(graph.nodes, filter));
  elements.push(...createGraphEdgesElements(graph.edges, performanceEdgesMap, aggregatedData, filter));

  for (let element of elements) {
    (<any>element).data.aggregatedData = aggregatedData;
  }

  return elements;
}

function createAggregatedDataInternal(graph: GrpcGraph, performanceMap: Record<number, any>, filter: RegExp | null) {
  let aggregatedData: AggregatedData = {
    totalExecutionTime: 0,
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

function preprocessForCSharpInterop(data: AggregatedData): AggregatedData {
  //JS Map can not be converted to C# Dictionary
  data.globalSoftwareData.counters = toObjectCsharpInterop(data.globalSoftwareData.counters);
  data.globalSoftwareData.activitiesDurations = toObjectCsharpInterop(data.globalSoftwareData.activitiesDurations);

  for (let [key, map] of data.globalSoftwareData.histograms) {
    data.globalSoftwareData.histograms.set(key, {
      units: map.units,
      value: toObjectCsharpInterop(map.value),
      group: map.group
    });
  }

  data.globalSoftwareData.histograms = toObjectCsharpInterop(data.globalSoftwareData.histograms);

  return data;
}

function toObjectCsharpInterop<TKey, TValue>(map: Map<TKey, TValue>): Map<TKey, TValue> {
  // @ts-ignore
  return Object.fromEntries(map);
}

function processNodesAggregatedData(nodes: GrpcGraphNode[], aggregatedData: AggregatedData, filter: RegExp | null) {
  for (let node of nodes) {
    let enhancementData = getNodeEnhancementDataOrNull(node, filter);
    updateAggregatedData(aggregatedData, enhancementData?.softwareData);

    let executionTime = calculateOverallExecutionTime(node);
    aggregatedData.totalExecutionTime += executionTime;
    aggregatedData.maxExecutionTime = Math.max(aggregatedData.maxExecutionTime, executionTime);
  }
}

function processEdgesAggregatedData(edges: GrpcGraphEdge[], aggregatedData: AggregatedData, performanceMap: Record<number, any>, filter: RegExp | null) {
  for (let edge of edges) {
    let enhancementData = getEdgeEnhancementDataOrNull(edge, filter);
    updateAggregatedData(aggregatedData, enhancementData?.softwareData);

    let executionTime = performanceMap[edge.id] ?? calculateEdgeExecutionTime(edge);

    if (executionTime != null) {
      aggregatedData.totalExecutionTime += executionTime;
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

function createGraphNodesElements(nodes: GrpcGraphNode[], filter: RegExp | null): cytoscape.ElementDefinition[] {
  let elements = [];

  for (let node of nodes) {
    elements.push({
      data: {
        frontendId: createNextFrontendUniqueId(),
        label: node.data,
        id: node.id.toString(),
        additionalData: node.additionalData,
        innerGraph: node.innerGraph,
        executionTime: calculateOverallExecutionTime(node),
        enhancementData: getNodeEnhancementDataOrNull(node, filter),
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

export function createGraphEdgesElements(
  edges: GrpcGraphEdge[],
  performanceMap: Record<number, any>,
  aggregatedData: AggregatedData,
  filter: RegExp | null
): cytoscape.ElementDefinition[] {
  let elements: cytoscape.ElementDefinition[] = [];

  let maxWeight = Math.max(...edges.map(e => e.weight));
  const minWidth = 5;
  const maxWidth = 20;

  for (let edge of edges) {
    let weightRatio = edge.weight / maxWeight
    let width = minWidth + (maxWidth - minWidth) * weightRatio;

    if (isNaN(width)) {
      width = 1;
    }

    let blueMin = graphColor.blueMin;
    let blueMax = graphColor.blueMax;

    let greenMin = graphColor.greenMin;
    let greenMax = graphColor.greenMax;

    let redMin = graphColor.redMin;
    let redMax = graphColor.redMax;

    let executionTime = performanceMap[edge.id] ?? calculateEdgeExecutionTime(edge);

    let color = executionTime == null ?
      calculateGradient(redMin, redMax, greenMin, greenMax, blueMin, blueMax, weightRatio) :
      getPerformanceAnnotationColor(executionTime / aggregatedData.totalExecutionTime);

    elements.push({
      data: {
        frontendId: createNextFrontendUniqueId(),
        label: edge.data,
        id: edge.id.toString(),
        source: edge.fromNode.toString(),
        target: edge.toNode.toString(),
        additionalData: edge.additionalData,
        enhancementData: getEdgeEnhancementDataOrNull(edge, filter),
        executionTime: executionTime,
        weight: edge.weight,
        width: width,
        color: color
      }
    })
  }

  return elements;
}
