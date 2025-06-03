import {calculateGradient, createNextFrontendUniqueId} from "../utils";
import {darkTheme, graphColors} from "../colors";
import {
  calculateEdgeExecutionTime,
  calculateOverallExecutionTime,
  getEdgeSoftwareDataOrNull,
  getNodeSoftwareDataOrNull,
  getPerformanceAnnotationColor,
} from "./util";
import {GrpcGraph} from "../protos/ficus/GrpcGraph";
import {GrpcAnnotation} from "../protos/ficus/GrpcAnnotation";
import {GrpcGraphNode} from "../protos/ficus/GrpcGraphNode";
import {GrpcGraphEdge} from "../protos/ficus/GrpcGraphEdge";
import cytoscape from "cytoscape";
import {AggregatedData, MergedSoftwareData} from "./types";

const graphColor = graphColors(darkTheme);

export function createGraphElements(graph: GrpcGraph, annotation: GrpcAnnotation, filter: RegExp | null): cytoscape.ElementDefinition[] {
  let elements: cytoscape.ElementDefinition[] = [];

  let aggregatedData: AggregatedData = {
    totalAllocatedBytes: 0,
    totalExecutionTime: 0,
    maxExecutionTime: Number.MIN_VALUE,
    totalBufferReturnedBytes: 0,
    totalBufferAllocatedBytes: 0,
    totalBufferRentedBytes: 0
  };

  let performanceEdgesMap = buildEdgesTimeAnnotationMap(annotation);

  processNodesAggregatedData(graph.nodes, aggregatedData, filter);
  processEdgesAggregatedData(graph.edges, aggregatedData, performanceEdgesMap, filter);

  elements.push(...createGraphNodesElements(graph.nodes, filter));
  elements.push(...createGraphEdgesElements(graph.edges, performanceEdgesMap, aggregatedData, filter));

  for (let element of elements) {
    (<any>element).data.aggregatedData = aggregatedData;
  }

  return elements;
}

function processNodesAggregatedData(nodes: GrpcGraphNode[], aggregatedData: AggregatedData, filter: RegExp | null) {
  for (let node of nodes) {
    let softwareData = getNodeSoftwareDataOrNull(node, filter);
    updateAggregatedData(aggregatedData, softwareData);

    let executionTime = calculateOverallExecutionTime(node);
    aggregatedData.totalExecutionTime += executionTime;
    aggregatedData.maxExecutionTime = Math.max(aggregatedData.maxExecutionTime, executionTime);
  }
}

function processEdgesAggregatedData(edges: GrpcGraphEdge[], aggregatedData: AggregatedData, performanceMap: Record<number, any>, filter: RegExp | null) {
  for (let edge of edges) {
    let softwareData = getEdgeSoftwareDataOrNull(edge, filter);
    updateAggregatedData(aggregatedData, softwareData);

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
    let executionTime = calculateOverallExecutionTime(node);
    let softwareData = getNodeSoftwareDataOrNull(node, filter);

    elements.push({
      data: {
        frontendId: createNextFrontendUniqueId(),
        label: node.data,
        id: node.id.toString(),
        additionalData: node.additionalData,
        innerGraph: node.innerGraph,
        executionTime: executionTime,
        softwareData: softwareData,
      }
    })
  }

  return elements;
}

function updateAggregatedData(aggregatedData: AggregatedData, softwareData: MergedSoftwareData) {
  if (softwareData != null) {
    aggregatedData.totalAllocatedBytes += softwareData.allocations.values().reduce((a, b) => a + b, 0);

    aggregatedData.totalBufferAllocatedBytes += softwareData.bufferAllocatedBytes.sum;
    aggregatedData.totalBufferRentedBytes += softwareData.bufferRentedBytes.sum;
    aggregatedData.totalBufferReturnedBytes += softwareData.bufferReturnedBytes.sum;
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
    let softwareData = getEdgeSoftwareDataOrNull(edge, filter);

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
        softwareData: softwareData,
        executionTime: executionTime,
        weight: edge.weight,
        width: width,
        color: color
      }
    })
  }

  return elements;
}
