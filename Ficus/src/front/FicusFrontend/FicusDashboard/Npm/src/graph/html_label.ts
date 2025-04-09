import {
  belongsToRootSequence,
  findAllRelatedTraceIds,
  getSoftwareDataOrNull,
  getTimeAnnotationColor
} from "./util";
import {darkTheme, graphColors} from "../colors";
import {nodeWidthPx, nodeHeightPx} from "./constants";
import tippy from "tippy.js";
import {getOrCreateColor} from "../utils";
import {GraphNode} from "./types";

const graphColor = graphColors(darkTheme);

export function createHtmlLabel(node: GraphNode) {
  let softwareData = getSoftwareDataOrNull(node);
  if (softwareData == null) {
    return null;
  }

  let sortedHistogramEntries = [...softwareData.histogram.entries()].toSorted((f: [string, number], s: [string, number]) => s[1] - f[1]);
  let nodeColor = belongsToRootSequence(node) ? graphColor.rootSequenceColor : graphColor.nodeBackground;
  let timeAnnotationColor = getTimeAnnotationColor(node.relativeExecutionTime);
  let allTraceIds = [...findAllRelatedTraceIds(node).values()];
  allTraceIds.sort((f, s) => f - s);

  return `
          <div style="background: ${nodeColor}; min-width: ${nodeWidthPx}px; min-height: ${nodeHeightPx}px">
              <div style="width: 100%; text-align: center; color: ${graphColor.labelColor}; background-color: ${timeAnnotationColor}">
                  ${node.label} [${node.executionTime}] ${createTracesDescription(allTraceIds)}
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

addEventListener("mouseover", event => {
  let element = event.target;

  if (element instanceof HTMLElement) {
    let rawData = element.dataset.histogramTooltip;

    if (rawData != null) {
      let data = JSON.parse(rawData);

      tippy(element, {
        content: `
                <div style="padding: 10px; background: black; color: white; border-radius: 5px;">
                    ${data.name}
                </div>
               `,
        allowHTML: true,
        zIndex: Number.MAX_VALUE,
        duration: 0,
        arrow: true,
      });
    } 
  }
});

function createHistogram(sortedHistogramEntries: [string, number][]) {
  let summedCount = Math.max(...sortedHistogramEntries.map(entry => entry[1]));

  return sortedHistogramEntries.map((entry) => {
    let divWidth = (entry[1] / summedCount) * 100;
    return `
        <div class="graph-histogram-entry"
             style="width: ${divWidth}%; height: 10px; background-color: ${getOrCreateColor(entry[0])}" 
             data-histogram-tooltip='${JSON.stringify(entry)}'>
        </div>
      `;
  });
}

function createEventClassesDescription(sortedHistogramEntries: [string, number][]) {
  return sortedHistogramEntries.map((entry) => {
    return `
        <div style="display: flex; flex-direction: row; height: 20px; align-items: center">
            <div style="width: 15px; height: 15px; background-color: ${getOrCreateColor(entry[0])}"></div>
            <div>${entry[0]}</div>
        </div>
      `;
  });
}

function createTracesDescription(tracesIds: number[]) {
  let result = "";
  let index = 0;
  let groupStartIndex = 0;

  while (index < tracesIds.length) {
    groupStartIndex = index;
    while (index < tracesIds.length - 1 && tracesIds[index] + 1 === tracesIds[index + 1]) {
      index += 1;
    }

    if (groupStartIndex === index) {
      result += `${tracesIds[groupStartIndex]}, `;
    } else {
      result += `${tracesIds[groupStartIndex]}..${tracesIds[index]}, `;
    }

    index += 1;
  }
  
  if (result.length < 2) {
    return "No traces";
  }

  return result.substring(0, result.length - 2);
}