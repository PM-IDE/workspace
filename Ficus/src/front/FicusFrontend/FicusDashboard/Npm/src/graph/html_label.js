import {
  belongsToRootSequence,
  findAllRelatedTraceIds,
  getOrCreateColor,
  getSoftwareDataOrNull,
  getTimeAnnotationColor
} from "./util";
import {darkTheme, graphColors} from "../colors";
import {nodeWidthPx, nodeHeightPx} from "./constants";

const graphColor = graphColors(darkTheme);

export function createHtmlLabel(node) {
  let softwareData = getSoftwareDataOrNull(node);
  if (softwareData == null) {
    return null;
  }

  let sortedHistogramEntries = softwareData.histogram.toSorted((f, s) => s.count - f.count);
  let nodeColor = belongsToRootSequence(node) ? graphColor.rootSequenceColor : graphColor.nodeBackground;
  let timeAnnotationColor = getTimeAnnotationColor(node.relativeExecutionTime);
  console.log(node);
  let allTraceIds = findAllRelatedTraceIds(node);

  return `
          <div style="background: ${nodeColor}; min-width: ${nodeWidthPx}px; min-height: ${nodeHeightPx}px">
              <div style="width: 100%; text-align: center; color: ${graphColor.labelColor}; background-color: ${timeAnnotationColor}">
                  ${node.label} [${node.executionTime}] ${[...allTraceIds.values().map(x => x.toString())].join(" ")}
              </div>
              <div style="width: 100%; display: flex; flex-direction: row;">
                  ${createHistogram(sortedHistogramEntries).join('\n')}
              </div>
              <div style="width: 100%; display: flex; flex-direction: column;">
                  ${createEventClassesDescription(sortedHistogramEntries).join('\n')}
              </div>
          </div>
        `;
}

function createHistogram(sortedHistogramEntries) {
  let summedCount = Math.max(...sortedHistogramEntries.map(entry => entry.count));

  return sortedHistogramEntries.map((entry) => {
    let divWidth = (entry.count / summedCount) * 100;
    return `<div style="width: ${divWidth}%; height: 10px; background-color: ${getOrCreateColor(entry.name)}"></div>`;
  });
}

function createEventClassesDescription(sortedHistogramEntries) {
  return sortedHistogramEntries.map((entry) => {
    return `
        <div style="display: flex; flex-direction: row; height: 20px; align-items: center">
            <div style="width: 15px; height: 15px; background-color: ${getOrCreateColor(entry.name)}"></div>
            <div>${entry.name}</div>
        </div>
      `;
  });
}