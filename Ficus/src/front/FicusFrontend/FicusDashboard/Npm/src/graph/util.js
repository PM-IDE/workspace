import {generateRandomColor} from "../utils";

export function createDagreLayout() {
  return {
    name: 'dagre',
    rankDir: 'LR',
    nodeDimensionsIncludeLabels: true,
    ranker: 'tight-tree',
    spacingFactor: 2
  }
}

export function createPresetLayout() {
  return {
    name: 'preset'
  }
}

export function getSoftwareDataOrNull(node) {
  let mergedSoftwareData = {
    histogram: new Map(),
    timelineDiagramFragment: []
  };

  for (let data of node.additionalData) {
    if (data.softwareData != null) {
      for (let [name, count] of data.softwareData.histogram.map(entry => [entry.name, entry.count])) {
        if (mergedSoftwareData.histogram.has(name)) {
          mergedSoftwareData.histogram.set(name, mergedSoftwareData.histogram.get(name) + count);
        } else {
          mergedSoftwareData.histogram.set(name, count);
        }
      }

      mergedSoftwareData.timelineDiagramFragment.push(data.softwareData.timelineDiagramFragment);
    }
  }

  mergedSoftwareData.histogram = mergedSoftwareData.histogram.entries().map(e => {
      return {
        name: e[0],
        count: e[1]
      }
    }
  ).toArray();

  return mergedSoftwareData;
}

export function calculateOverallExecutionTime(node) {
  let timeData = getTimeData(node);
  let minTime = Math.min(...timeData.map(t => t.startTime));
  let maxTime = Math.max(...timeData.map(t => t.endTime));

  let overallExecutionTime = maxTime - minTime;
  if (!isFinite(overallExecutionTime) || isNaN(overallExecutionTime)) {
    return 0;
  }

  return overallExecutionTime;
}

export function getTimeData(node) {
  let result = [];
  for (let data of node.additionalData) {
    if (data.timeData != null) {
      result.push(data.timeData);
    }
  }
  
  return result;
}

export function belongsToRootSequence(node) {
  for (let data of node.additionalData.filter((d, _) => d.traceData != null)) {
    if (data.traceData.belongsToRootSequence === true) {
      return true;
    }
  }

  return false;
}

let colorsCache = {};

export function getOrCreateColor(name) {
  if (!(name in colorsCache)) {
    colorsCache[name] = generateRandomColor();
  }

  return colorsCache[name];
}
