import {darkTheme, performanceColors} from "../colors";
import {GrpcNodeAdditionalData} from "../protos/ficus/GrpcNodeAdditionalData";
import {CountAndSum, GraphEdge, GraphNode, MergedEnhancementData, MergedSoftwareData} from "./types";
import {GrpcGraphNode} from "../protos/ficus/GrpcGraphNode";
import {GrpcSoftwareData} from "../protos/ficus/GrpcSoftwareData";
import {GrpcUnderlyingPatternInfo} from "../protos/ficus/GrpcUnderlyingPatternInfo";
import {GrpcGraphEdgeAdditionalData} from "../protos/ficus/GrpcGraphEdgeAdditionalData";
import {GrpcGraphEdge} from "../protos/ficus/GrpcGraphEdge";
import {GrpcMethodNameParts} from "../protos/ficus/GrpcMethodNameParts";
import {GrpcGraphKind} from "../protos/ficus/GrpcGraphKind";
import cytoscape from "cytoscape";
import dagre from 'cytoscape-dagre';

let elk = require('cytoscape-elk');
cytoscape.use(elk);

cytoscape.use(dagre);

export function createLayout(kind: GrpcGraphKind, spacingFactor: number = 1, useLROrientation: boolean = true) {
  switch (kind) {
    case GrpcGraphKind.None:
      return createGridLayout(spacingFactor, useLROrientation);
    case GrpcGraphKind.DAG:
      return createDagreLayout(spacingFactor, useLROrientation);
  }
}

function createDagreLayout(spacingFactor: number = 1, useLROrientation: boolean = true) {
  return {
    name: 'dagre',
    rankDir: useLROrientation ? 'LR' : 'TB',
    nodeDimensionsIncludeLabels: true,
    ranker: 'tight-tree',
    spacingFactor: spacingFactor
  }
}

function createGridLayout(spacingFactor: number = 1, useLROrientation: boolean = true) {
  return {
    name: 'elk',
    spacingFactor: spacingFactor,
    nodeDimensionsIncludeLabels: true,
    rankDir: useLROrientation ? 'LR' : 'TB',
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

export function getEdgeEnhancementDataOrNull(edge: GraphEdge | GrpcGraphEdge, filter: RegExp | null): MergedEnhancementData {
  return createMergedEnhancementData((action: (data: GrpcSoftwareData) => void) => {
    for (let data of edge.additionalData.filter(e => e.softwareData != null).map(e => e.softwareData)) {
      action(data);
    }
  }, filter);
}

export function getNodeEnhancementDataOrNull(node: GraphNode | GrpcGraphNode, filter: RegExp | null): MergedEnhancementData {
  return createMergedEnhancementData((action: (data: GrpcSoftwareData) => void) => {
    executeWithNodeAdditionalData(node, (data) => {
      if (data.softwareData != null) {
        action(data.softwareData)
      }
    });
  }, filter);
}

export function createEmptySoftwareData(): MergedSoftwareData {
  return {
    allocations: new Map(),

    inliningFailed: new Map(),
    inliningSucceeded: new Map(),
    inliningFailedReasons: new Map(),

    methodsUnloads: new Map(),
    methodsLoads: new Map(),

    bufferAllocatedBytes: {count: 0, sum: 0},
    bufferRentedBytes: {count: 0, sum: 0},
    bufferReturnedBytes: {count: 0, sum: 0},

    exceptions: new Map(),

    createdThreads: new Set(),
    terminatedThreads: new Set(),

    httpRequests: new Map(),

    histograms: new Map(),
    counters: new Map(),
    activitiesDurations: new Map()
  };
}

function createMergedEnhancementData(
  softwareDataEnumerator: (data: ((softwareData: GrpcSoftwareData) => void)) => void,
  filter: RegExp | null
): MergedEnhancementData {
  let enhancementData: MergedEnhancementData = {
    eventClasses: new Map(),
    timelineDiagramFragments: [],
    softwareData: createEmptySoftwareData()
  };

  let matchesFilter = (value: string) => {
    if (filter != null) {
      return filter.test(value);
    }

    return true;
  }

  softwareDataEnumerator((softwareData: GrpcSoftwareData) => {
    for (let entry of softwareData.histogram) {
      let [name, count] = [entry.name, entry.count];

      if (matchesFilter(name)) {
        increment(enhancementData.eventClasses, name, count);
      }
    }

    enhancementData.timelineDiagramFragments.push(softwareData.timelineDiagramFragment);

    for (let alloc of softwareData.allocationsInfo) {
      let allocBytes = alloc.allocatedBytes * alloc.allocatedObjectsCount;

      if (matchesFilter(alloc.typeName)) {
        increment(enhancementData.softwareData.allocations, alloc.typeName, allocBytes);
      }
    }

    for (let inliningEvent of softwareData.methodsInliningEvents) {
      let fqn = restoreFqn(inliningEvent.inliningInfo.inlineeInfo);
      if (!matchesFilter(fqn)) {
        continue;
      }

      if (inliningEvent.failed != null) {
        increment(enhancementData.softwareData.inliningFailed, fqn, 1);
        increment(enhancementData.softwareData.inliningFailedReasons, inliningEvent.failed.reason, 1);
      } else if (inliningEvent.succeeded != null) {
        increment(enhancementData.softwareData.inliningSucceeded, fqn, 1);
      }
    }

    for (let loadUnloadEvent of softwareData.methodsLoadUnloadEvents) {
      let fqn = restoreFqn(loadUnloadEvent.methodNameParts);
      if (!matchesFilter(fqn)) {
        continue;
      }

      if (loadUnloadEvent.load != null) {
        increment(enhancementData.softwareData.methodsLoads, fqn, 1);
      } else if (loadUnloadEvent.unload != null) {
        increment(enhancementData.softwareData.methodsUnloads, fqn, 1);
      }
    }

    for (let arrayPoolEvent of softwareData.arrayPoolEvents) {
      if (!matchesFilter(arrayPoolEvent.bufferId.toString())) {
        continue;
      }

      if (arrayPoolEvent.bufferAllocated != null) {
        incrementCountAndSum(enhancementData.softwareData.bufferAllocatedBytes, arrayPoolEvent.bufferSizeBytes);
      } else if (arrayPoolEvent.bufferReturned != null) {
        incrementCountAndSum(enhancementData.softwareData.bufferReturnedBytes, arrayPoolEvent.bufferSizeBytes);
      } else if (arrayPoolEvent.bufferRented != null) {
        incrementCountAndSum(enhancementData.softwareData.bufferRentedBytes, arrayPoolEvent.bufferSizeBytes);
      }
    }

    for (let exception of softwareData.exceptionEvents) {
      if (matchesFilter(exception.exceptionType)) {
        increment(enhancementData.softwareData.exceptions, exception.exceptionType, 1);
      }
    }

    for (let threadEvent of softwareData.threadEvents) {
      if (!matchesFilter(threadEvent.threadId.toString())) {
        continue;
      }

      if (threadEvent.created != null) {
        enhancementData.softwareData.createdThreads.add(threadEvent.threadId);
      } else if (threadEvent.terminated != null) {
        enhancementData.softwareData.terminatedThreads.add(threadEvent.threadId);
      }
    }

    for (let httpEvent of softwareData.httpEvents) {
      let requestUrl = httpEvent.scheme + "://" + httpEvent.host + ":" + httpEvent.port + httpEvent.pathAndQuery;
      if (matchesFilter(requestUrl)) {
        increment(enhancementData.softwareData.httpRequests, requestUrl, 1);
      }
    }

    for (let histogram of softwareData.histogramData) {
      let histogramMap;
      if (enhancementData.softwareData.histograms.has(histogram.base.name)) {
        histogramMap = enhancementData.softwareData.histograms.get(histogram.base.name).value;
      } else {
        histogramMap = new Map();
        enhancementData.softwareData.histograms.set(histogram.base.name, {
          value: histogramMap,
          units: histogram.base.units,
          group: histogram.base.group,
        });
      }

      for (let data of histogram.entries) {
        increment(histogramMap, data.name, data.count);
      }
    }

    for (let counter of softwareData.simpleCounterData) {
      if (!enhancementData.softwareData.counters.has(counter.base.name)) {
        enhancementData.softwareData.counters.set(counter.base.name, {
          value: 0,
          units: counter.base.units,
          group: counter.base.group,
        });
      }

      enhancementData.softwareData.counters.get(counter.base.name).value += counter.count;
    }

    for (let activityDuration of softwareData.activitiesDurationsData) {
      if (!enhancementData.softwareData.activitiesDurations.has(activityDuration.base.name)) {
        enhancementData.softwareData.activitiesDurations.set(activityDuration.base.name, {
          value: 0,
          units: activityDuration.base.units,
          group: activityDuration.base.group,
        });
      }

      enhancementData.softwareData.activitiesDurations.get(activityDuration.base.name).value += activityDuration.duration;
    }
  });

  return enhancementData;
}

function restoreFqn(data: GrpcMethodNameParts) {
  return data.namespace + "." + data.name + "[" + data.signature + "]";
}

function incrementCountAndSum(countAndSum: CountAndSum, value: number) {
  countAndSum.sum += value;
  countAndSum.count += 1;
}

export function increment(map: Map<string, number>, key: string, value: number) {
  if (!map.has(key)) {
    map.set(key, value);
  } else {
    map.set(key, map.get(key) + value);
  }
}

export function calculateEdgeExecutionTime(edge: GraphEdge | GrpcGraphEdge): number | null {
  let executionTime = 0;

  for (let data of edge.additionalData) {
    if (data.timeData != null) {
      executionTime += data.timeData.endTime - data.timeData.startTime;
    }
  }

  return executionTime == 0 ? null : executionTime;
}

export function executeWithNodeAdditionalData(
  node: GraphNode | GrpcGraphNode,
  handler: (data: GrpcNodeAdditionalData | GrpcGraphEdgeAdditionalData) => void
) {
  if (node.innerGraph != null) {
    for (let innerNode of node.innerGraph.nodes) {
      executeWithNodeAdditionalData(innerNode, handler);
    }

    for (let edge of node.innerGraph.edges) {
      for (let data of edge.additionalData) {
        handler(data);
      }
    }

    return;
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
        executeWithNodeAdditionalData(patternNode, handler);
      }

      for (let edge of pattern.graph.edges) {
        for (let data of edge.additionalData) {
          handler(data);
        }
      }
    }

    return;
  }

  for (let data of node.additionalData) {
    handler(data);
  }
}


export function calculateOverallExecutionTime(node: GrpcGraphNode) {
  let overallExecutionTime = 0;

  executeWithNodeAdditionalData(node, (data: GrpcGraphEdgeAdditionalData | GrpcNodeAdditionalData) => {
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