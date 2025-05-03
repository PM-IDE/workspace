import {calculateGradient, createNextFrontendUniqueId} from "../utils";
import {darkTheme, graphColors} from "../colors";
import {
  calculateEdgeExecutionTime,
  calculateOverallExecutionTime,
  getEdgeSoftwareDataOrNull,
  getNodeSoftwareDataOrNull,
  getPerformanceAnnotationColor, MergedSoftwareData
} from "./util";
import {GrpcGraph} from "../protos/ficus/GrpcGraph";
import {GrpcAnnotation} from "../protos/ficus/GrpcAnnotation";
import {GrpcGraphNode} from "../protos/ficus/GrpcGraphNode";
import {GrpcGraphEdge} from "../protos/ficus/GrpcGraphEdge";
import {GrpcTimePerformanceAnnotation} from "../protos/ficus/GrpcTimePerformanceAnnotation";
import cytoscape from "cytoscape";
import {AggregatedData} from "./types";

const graphColor = graphColors(darkTheme);

export function createGraphElementForDagre(graph: GrpcGraph, annotation: GrpcAnnotation, filter: RegExp | null): cytoscape.ElementDefinition[] {
  let elements: cytoscape.ElementDefinition[] = [];

  let aggregatedData: AggregatedData = {
    totalAllocatedBytes: 0,
    totalExecutionTime: 0,
    maxExecutionTime: Number.MIN_VALUE,
    totalBufferReturnedBytes: 0,
    totalBufferAllocatedBytes: 0,
    totalBufferRentedBytes: 0
  };

  elements.push(...createGraphNodesElements(graph.nodes, aggregatedData, filter));
  elements.push(...createGraphEdgesElements(graph.edges, annotation, aggregatedData, filter));

  for (let element of elements) {
    (<any>element).data.aggregatedData = aggregatedData;
  }

  return elements;
}

function createGraphNodesElements(nodes: GrpcGraphNode[], aggregatedData: AggregatedData, filter: RegExp | null): cytoscape.ElementDefinition[] {
  let elements = [];

  for (let node of nodes) {
    let softwareData = getNodeSoftwareDataOrNull(node, filter);
    updateAggregatedData(aggregatedData, softwareData);

    let executionTime = calculateOverallExecutionTime(node);
    aggregatedData.totalExecutionTime += executionTime;
    aggregatedData.maxExecutionTime = Math.max(aggregatedData.maxExecutionTime, executionTime);

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
  annotation: GrpcAnnotation,
  aggregatedData: AggregatedData,
  filter: RegExp | null
): cytoscape.ElementDefinition[] {
  let elements: cytoscape.ElementDefinition[] = [];
  for (let edge of edges) {
    let softwareData = getEdgeSoftwareDataOrNull(edge, filter);
    updateAggregatedData(aggregatedData, softwareData);

    let executionTime = calculateEdgeExecutionTime(edge);
    aggregatedData.totalExecutionTime += executionTime;
    aggregatedData.maxExecutionTime = Math.max(executionTime, aggregatedData.maxExecutionTime);

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
        weight: edge.weight
      }
    })
  }

  let edgesMap: Record<number, any> = {};

  for (let edge of edges) {
    edgesMap[edge.id] = {};
  }

  processEdgesWeights(edges, edgesMap);
  if (annotation !== null && annotation.timeAnnotation !== null) {
    processTimeAnnotation(annotation.timeAnnotation, edges, edgesMap, aggregatedData);
  }

  for (let element of elements) {
    element.data.color = edgesMap[parseInt(element.data.id)].color;
    element.data.width = edgesMap[parseInt(element.data.id)].width;
  }

  return elements;
}

function processEdgesWeights(edges: GrpcGraphEdge[], edgesMap: Record<number, any>) {
  const minWidth = 1;
  const maxWidth = 15;
  let maxWeight = Math.max(...edges.map(e => e.weight));

  for (let edge of edges) {
    let weightRatio = edge.weight / maxWeight
    let width = minWidth + (maxWidth - minWidth) * weightRatio;

    if (isNaN(width)) {
      width = 1;
    }

    edgesMap[edge.id].width = width;

    let blueMin = graphColor.blueMin;
    let blueMax = graphColor.blueMax;

    let greenMin = graphColor.greenMin;
    let greenMax = graphColor.greenMax;

    let redMin = graphColor.redMin;
    let redMax = graphColor.redMax;

    edgesMap[edge.id].color = calculateGradient(redMin, redMax, greenMin, greenMax, blueMin, blueMax, weightRatio);
  }
}

function processTimeAnnotation(
  annotation: GrpcTimePerformanceAnnotation,
  edges: GrpcGraphEdge[],
  edgesMap: Record<number, any>,
  aggregatedData: AggregatedData
) {
  let idsToTime: Record<number, any> = {};

  for (let timeAnnotation of annotation.annotations) {
    idsToTime[timeAnnotation.entityId] = timeAnnotation.interval.nanoseconds;
  }

  for (let edge of edges) {
    let timeAnnotation = idsToTime[edge.id] / aggregatedData.totalExecutionTime;

    edgesMap[edge.id].timeAnnotation = timeAnnotation;
    edgesMap[edge.id].color = getPerformanceAnnotationColor(timeAnnotation);
  }
}