import {darkTheme, performanceColors} from "../colors";
import {GrpcNodeAdditionalData} from "../protos/ficus/GrpcNodeAdditionalData";
import {GraphEdge, GraphNode} from "./types";
import {GrpcTimelineDiagramFragment} from "../protos/ficus/GrpcTimelineDiagramFragment";
import {GrpcGraphNode} from "../protos/ficus/GrpcGraphNode";
import {GrpcSoftwareData} from "../protos/ficus/GrpcSoftwareData";
import {GrpcUnderlyingPatternInfo} from "../protos/ficus/GrpcUnderlyingPatternInfo";
import {GrpcGraphEdgeAdditionalData} from "../protos/ficus/GrpcGraphEdgeAdditionalData";
import {GrpcGraphEdge} from "../protos/ficus/GrpcGraphEdge";
import {GrpcMethodNameParts} from "../protos/ficus/GrpcMethodNameParts";

export function createDagreLayout() {
  return {
    name: 'dagre',
    rankDir: 'LR',
    nodeDimensionsIncludeLabels: true,
    ranker: 'tight-tree',
    spacingFactor: 1.5
  }
}

export function createPresetLayout() {
  return {
    name: 'preset'
  }
}


export function findAllRelatedTraceIds(node: GraphNode): Set<number> {
  let traceIds = new Set<number>();
  for (let data of node.additionalData) {
    traceIds.add(getTraceId(data));
  }

  return traceIds;
}

export function getTraceId(additionalData: GrpcNodeAdditionalData): number {
  return additionalData.originalEventCoordinates.traceId;
}

export interface MergedSoftwareData {
  histogram: Map<string, number>,
  timelineDiagramFragments: GrpcTimelineDiagramFragment[],
  allocations: Map<string, number>,
  inliningFailed: Map<string, number>,
  inliningSucceeded: Map<string, number>,
  inliningFailedReasons: Map<string, number>,
}

export function getEdgeSoftwareDataOrNull(edge: GraphEdge | GrpcGraphEdge): MergedSoftwareData {
  let softwareData = edge.additionalData.filter(e => e.softwareData != null).map(e => e.softwareData);
  return createMergedSoftwareData(softwareData);
}

export function getNodeSoftwareDataOrNull(node: GraphNode | GrpcGraphNode): MergedSoftwareData {
  return createMergedSoftwareData(extractAllSoftwareData(node));
}

function createMergedSoftwareData(originalSoftwareData: GrpcSoftwareData[]): MergedSoftwareData {
  if (originalSoftwareData.length == 0) {
    return null;
  }

  let mergedSoftwareData: MergedSoftwareData = {
    histogram: new Map(),
    timelineDiagramFragments: [],
    allocations: new Map(),
    inliningFailed: new Map(),
    inliningSucceeded: new Map(),
    inliningFailedReasons: new Map(),
  };

  for (let softwareData of originalSoftwareData) {
    for (let entry of softwareData.histogram) {
      let [name, count] = [entry.name, entry.count];
      increment(mergedSoftwareData.histogram, name, count);
    }

    mergedSoftwareData.timelineDiagramFragments.push(softwareData.timelineDiagramFragment);

    for (let alloc of softwareData.allocationsInfo) {
      let allocBytes = alloc.allocatedBytes * alloc.allocatedObjectsCount;
      increment(mergedSoftwareData.allocations, alloc.typeName, allocBytes);
    }

    for (let inliningEvent of softwareData.methodsInliningEvents) {
      if (inliningEvent.failed != null) {
        increment(mergedSoftwareData.inliningFailed, restoreFqn(inliningEvent.inliningInfo.inlineeInfo), 1);
        increment(mergedSoftwareData.inliningFailedReasons, inliningEvent.failed.reason, 1);
      } else if (inliningEvent.succeeded != null) {
        increment(mergedSoftwareData.inliningSucceeded, restoreFqn(inliningEvent.inliningInfo.inlineeInfo), 1);
      }
    }
  }

  return mergedSoftwareData;
}

function restoreFqn(data: GrpcMethodNameParts) {
  return data.namespace + "." + data.name + "[" + data.signature + "]";
}

function increment(map: Map<string, number>, key: string, value: number) {
  if (!map.has(key)) {
    map.set(key, value);
  } else {
    map.set(key, map.get(key) + value);
  }
}

export function executeWithAdditionalData(node : GraphNode | GrpcGraphNode, handler: Function) {
  let result: GrpcSoftwareData[] = [];

  if (node.innerGraph != null) {
    for (let innerNode of node.innerGraph.nodes) {
      result.push(...executeWithAdditionalData(innerNode, handler));
    }

    for (let edge of node.innerGraph.edges) {
      for (let data of edge.additionalData) {
        handler(data);
      }
    }

    return result;
  }

  let patterns: GrpcUnderlyingPatternInfo[] = [];
  for (let data of node.additionalData) {
    if (data.patternInfo != null) {
      patterns.push(data.patternInfo);
    }
  }

  if (patterns.length > 0) {
    for (let pattern of patterns) {
      for (let patternNode of pattern.graph.nodes) {
        result.push(...executeWithAdditionalData(patternNode, handler));
      }

      for (let edge of pattern.graph.edges) {
        for (let data of edge.additionalData) {
          handler(data);
        }
      }
    }

    return result;
  }

  for (let data of node.additionalData) {
    handler(data);
  }

  return result;
}

export function extractAllSoftwareData(node : GraphNode | GrpcGraphNode): GrpcSoftwareData[] {
  let result: GrpcSoftwareData[] = [];

  executeWithAdditionalData(node, (data: GrpcNodeAdditionalData | GrpcGraphEdgeAdditionalData) => {
    if (data.softwareData != null) {
      result.push(data.softwareData); 
    }
  });
  
  return result;
}

export function calculateOverallExecutionTime(node: GrpcGraphNode) {
  let overallExecutionTime = 0;

  executeWithAdditionalData(node, (data: GrpcGraphEdgeAdditionalData | GrpcNodeAdditionalData) => {
    if (data.timeData != null) {
      overallExecutionTime += data.timeData.endTime - data.timeData.startTime;
    }
  });

  return overallExecutionTime;
}

export function belongsToRootSequence(node: GraphNode) {
  for (let data of node.additionalData.filter((d, _) => d.traceData != null)) {
    if (data.traceData.belongsToRootSequence === true) {
      return true;
    }
  }

  return false;
}

const performanceColor = performanceColors(darkTheme);

export function getPerformanceAnnotationColor(relativeExecutionTime: number) {
  let colorName = `color${(Math.floor(relativeExecutionTime * 10) % 100).toString()}`;
  return performanceColor[colorName];
}