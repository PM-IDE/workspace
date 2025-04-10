import {darkTheme, performanceColors} from "../colors";
import {GrpcNodeAdditionalData} from "../protos/ficus/GrpcNodeAdditionalData";
import {GraphNode} from "./types";
import {GrpcTimelineDiagramFragment} from "../protos/ficus/GrpcTimelineDiagramFragment";
import {GrpcGraphNode} from "../protos/ficus/GrpcGraphNode";

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

interface MergedSoftwareData {
  histogram: Map<string, number>,
  timelineDiagramFragments: GrpcTimelineDiagramFragment[]
}

export function getSoftwareDataOrNull(node: GraphNode): MergedSoftwareData {
  let mergedSoftwareData: MergedSoftwareData = {
    histogram: new Map(),
    timelineDiagramFragments: []
  };

  for (let data of node.additionalData) {
    if (data.softwareData != null) {
      for (let entry of data.softwareData.histogram) {
        let [name, count] = [entry.name, entry.count];

        if (mergedSoftwareData.histogram.has(name)) {
          mergedSoftwareData.histogram.set(name, mergedSoftwareData.histogram.get(name) + count);
        } else {
          mergedSoftwareData.histogram.set(name, count);
        }
      }

      mergedSoftwareData.timelineDiagramFragments.push(data.softwareData.timelineDiagramFragment);
    }
  }

  return mergedSoftwareData;
}

export function calculateOverallExecutionTime(node: GrpcGraphNode) {
  let timeData = getTimeData(node);
  let minTime = Math.min(...timeData.map(t => t.startTime));
  let maxTime = Math.max(...timeData.map(t => t.endTime));

  let overallExecutionTime = maxTime - minTime;
  if (!isFinite(overallExecutionTime) || isNaN(overallExecutionTime)) {
    return 0;
  }

  return overallExecutionTime;
}

export function getTimeData(node: GrpcGraphNode) {
  let result = [];
  for (let data of node.additionalData) {
    if (data.timeData != null) {
      result.push(data.timeData);
    }
  }
  
  return result;
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

export function getTimeAnnotationColor(relativeExecutionTime: number) {
  let colorName = `color${(Math.floor(relativeExecutionTime * 10) % 100).toString()}`;
  return performanceColor[colorName];
}