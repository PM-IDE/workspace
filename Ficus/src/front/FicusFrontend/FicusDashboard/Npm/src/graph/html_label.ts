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
          <div>
            <div style="width: 100%; font-size: 22px; background-color: transparent; color: ${graphColor.labelColor}; text-align: left;">
                ${createNodeDisplayName(node, sortedHistogramEntries)}
            </div>
            <div style="background: ${nodeColor}; width: ${nodeWidthPx}px; height: ${nodeHeightPx}px; border-width: 5px; 
                        border-style: solid; border-color: ${timeAnnotationColor}; 
                        position: relative;">
                <div style="width: 100%; height: 25px; text-align: center; color: ${graphColor.labelColor}; background-color: ${timeAnnotationColor}">
                    ${node.executionTime}
                </div>
                ${createNodeBody(node, sortedHistogramEntries)}
                <div style="display: flex; flex-direction: row; position: absolute; bottom: 0;">
                    ${createTracesDescription(allTraceIds).join("\n")}
                </div>
            </div>
          </div>
         `;
}

function createNodeBody(node: GraphNode, sortedHistogramEntries: [string, number][]): string {
  if (isPatternNode(node)) {
    return createPatternInformation(node);
  }

  return createDefaultNodeBody(sortedHistogramEntries);
}

function createDefaultNodeBody(sortedHistogramEntries: [string, number][]): string {
  return `
    <div style="display: flex; flex-direction: row;">
       <div style='width: 65px; height: 65px; margin-left: 10px; margin-top: 10px;'
            class="graph-node-histogram"
            data-histogram-tooltip='${JSON.stringify(sortedHistogramEntries)}'>
          <svg-pie-chart style="pointer-events: none">
            ${createHistogram(sortedHistogramEntries).join('\n')}
          </svg-pie-chart>
       </div>
    </div>
  `
}

addEventListener("mouseover", event => {
  let element = event.target;

  if (element instanceof HTMLElement) {
    let rawData = element.dataset.histogramTooltip;

    if (rawData != null) {
      let histogramEntries: [string, number][] = JSON.parse(rawData);

      tippy(element, {
        appendTo: document.fullscreenElement ? document.fullscreenElement : undefined,
        content: `
                <div style="padding: 10px; background: black; color: white; border-radius: 5px;">
                    ${createEventClassesDescription(histogramEntries).join('\n')}
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

function createNodeDisplayName(node: GraphNode, sortedHistogramEntries: [string, number][]) {
  let nodeNameParts: string[] = [];
  for (let i = 0; i < Math.min(3, sortedHistogramEntries.length); ++i) {
    nodeNameParts.push(`
      <div style="max-width: ${nodeWidthPx}px; text-overflow: ellipsis;">
        ${sortedHistogramEntries[i][0]}
      </div>
    `);
  }

  if (nodeNameParts.length == 0) {
    nodeNameParts.push(`<div>${node.label}</div>`)
  }

  return nodeNameParts.join("\n");
}

function createHistogram(sortedHistogramEntries: [string, number][]) {
  let summedCount = sortedHistogramEntries.map(entry => entry[1]).reduce((a, b) => a + b, 0);

  return sortedHistogramEntries.map((entry) => {
    let divWidth = (entry[1] / summedCount) * 100;
    return `
        <segment percent="${divWidth}" stroke="${getOrCreateColor(entry[0])}" />
      `;
  });
}

function createEventClassesDescription(sortedHistogramEntries: [string, number][]) {
  return sortedHistogramEntries.map((entry) => {
    return `
        <div style="display: flex; flex-direction: row; height: 20px; align-items: center">
            <div style="width: 15px; height: 15px; background-color: ${getOrCreateColor(entry[0])}"></div>
            <div style="margin-left: 5px;">${entry[0]}</div>
            <div style="margin-left: 5px;">${entry[1]}</div>
        </div>
      `;
  });
}

function createTracesDescription(tracesIds: number[]): string[] {
  let result = [];
  let index = 0;
  let groupStartIndex = 0;

  while (index < tracesIds.length) {
    groupStartIndex = index;
    while (index < tracesIds.length - 1 && tracesIds[index] + 1 === tracesIds[index + 1]) {
      index += 1;
    }

    if (groupStartIndex === index) {
      result.push(`<div class="graph-node-trace-id-container">${tracesIds[groupStartIndex]}</div>`)
    } else {
      result.push(`<div class="graph-node-trace-id-container">${tracesIds[groupStartIndex]}..${tracesIds[index]}</div>`)
    }

    index += 1;
  }

  return result;
}

function isPatternNode(node: GraphNode): boolean {
  return node.additionalData.find(d => d.patternInfo != null) != null;
}

function createPatternInformation(node: GraphNode): string {
  let patterns: string[] = [];

  for (let data of node.additionalData) {
    if (data.patternInfo != null) {
      let baseSequence = data.patternInfo.baseSequence.map((c, index) => `
        <div style="width: 20px; height: 20px; background-color: ${getOrCreateColor(c)}; margin-left: ${index == 0 ? 5 : 0}px;"></div>
      `);
      
      patterns.push(`
        <div style="display: flex; flex-direction: row;">
            ${baseSequence.join("\n")}
        </div>
      `);
    }
  }

  return `
    <div style="margin-top: 5px;">
      ${patterns.join("\n")}
    </div>
  `
}