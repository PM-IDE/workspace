import {calculateGradient} from "../utils";
import {darkTheme, graphColors} from "../colors";
import {calculateOverallExecutionTime, getTimeAnnotationColor} from "./util";
import {GrpcGraph} from "../protos/ficus/GrpcGraph";
import {GrpcAnnotation} from "../protos/ficus/GrpcAnnotation";
import {GrpcGraphNode} from "../protos/ficus/GrpcGraphNode";
import {GrpcGraphEdge} from "../protos/ficus/GrpcGraphEdge";
import {GrpcTimePerformanceAnnotation} from "../protos/ficus/GrpcTimePerformanceAnnotation";
import cytoscape from "cytoscape";

const graphColor = graphColors(darkTheme);

export function createGraphElementForDagre(graph: GrpcGraph, annotation: GrpcAnnotation): cytoscape.ElementDefinition[] {
  let elements: cytoscape.ElementDefinition[] = [];

  let nodesMap = processNodes(graph.nodes);

  for (let node of graph.nodes) {
    elements.push({
      data: {
        label: node.data,
        id: node.id.toString(),
        additionalData: node.additionalData,
        innerGraph: node.innerGraph,
        executionTime: nodesMap[node.id].executionTime,
        relativeExecutionTime: nodesMap[node.id].relativeExecutionTime,
      }
    })
  }

  elements.push(...createGraphEdgesElements(graph.edges, annotation));

  return elements;
}

function processNodes(nodes: GrpcGraphNode[]): Record<number, any> {
  let nodesMap: Record<number, any> = {};
  for (let node of nodes) {
    nodesMap[node.id] = {};
  }

  let executionTimes = nodes.map(n => calculateOverallExecutionTime(n));
  let maxTime = Math.max(...executionTimes);
  let minTime = Math.min(...executionTimes);

  for (let i = 0; i < nodes.length; ++i) {
    nodesMap[nodes[i].id].executionTime = executionTimes[i];
    nodesMap[nodes[i].id].relativeExecutionTime = (executionTimes[i] - minTime) / (maxTime - minTime);
  }
  
  return nodesMap;
}

export function createGraphEdgesElements(edges: GrpcGraphEdge[], annotation: GrpcAnnotation): cytoscape.ElementDefinition[] {
  let edgesMap: Record<number, any> = {};

  for (let edge of edges) {
    edgesMap[edge.id] = {};
  }

  processEdgesWeights(edges, edgesMap);

  if (annotation !== null && annotation.timeAnnotation !== null) {
    processTimeAnnotation(annotation.timeAnnotation, edges, edgesMap);
  }

  let elements: cytoscape.ElementDefinition[] = [];
  for (let edge of edges) {
    elements.push({
      data: {
        color: edgesMap[edge.id].color,
        width: edgesMap[edge.id].width,
        label: edge.data,
        id: edge.id.toString(),
        source: edge.fromNode.toString(),
        target: edge.toNode.toString(),
        additionalData: edge.additionalData
      }
    })
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

function processTimeAnnotation(annotation: GrpcTimePerformanceAnnotation, edges: GrpcGraphEdge[], edgesMap: Record<number, any>) {
  let minTime = null;
  let maxTime = null;
  let idsToTime: Record<number, any> = {};

  for (let timeAnnotation of annotation.annotations) {
    let time = timeAnnotation.interval.nanoseconds;
    if (minTime === null || time < minTime) {
      minTime = time;
    }

    if (maxTime == null || time > maxTime) {
      maxTime = time;
    }

    idsToTime[timeAnnotation.entityId] = time;
  }

  for (let edge of edges) {
    let timeAnnotation = (idsToTime[edge.id] - minTime) / (maxTime - minTime);

    edgesMap[edge.id].timeAnnotation = timeAnnotation;
    edgesMap[edge.id].color = getTimeAnnotationColor(timeAnnotation);
  }
}