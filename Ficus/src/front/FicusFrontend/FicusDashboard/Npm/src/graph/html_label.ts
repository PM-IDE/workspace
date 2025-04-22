import {
  belongsToRootSequence,
  findAllRelatedTraceIds,
  getPerformanceAnnotationColor, 
  MergedSoftwareData
} from "./util";
import {darkTheme, graphColors} from "../colors";
import {nodeWidthPx, nodeHeightPx} from "./constants";
import tippy from "tippy.js";
import {getOrCreateColor} from "../utils";
import {AggregatedData, GraphEdge, GraphNode} from "./types";

const graphColor = graphColors(darkTheme);

export function createEdgeHtmlLabel(edge: GraphEdge) {
  let softwareData = edge.softwareData;
  if (softwareData == null) {
    return "";
  }

  return `
      <div>
        ${createRectangleHistogram(toSortedArray(softwareData.allocations), edge.aggregatedData)}
      </div>
    `;
}

export function createNodeHtmlLabel(node: GraphNode) {
  let softwareData = node.softwareData;
  if (softwareData == null) {
    return `
        <div style='width: ${nodeWidthPx}px; height: ${nodeHeightPx}px; 
                    background-color: ${graphColor.rootSequenceColor}'>
            ${createNodeDisplayName(node, node.label)}
        </div>
    `;
  }

  let sortedHistogramEntries = toSortedArray(softwareData.histogram);
  let nodeColor = belongsToRootSequence(node) ? graphColor.rootSequenceColor : graphColor.nodeBackground;
  let timeAnnotationColor = getPerformanceAnnotationColor(node.executionTime / node.aggregatedData.maxNodeExecutionTime);
  let allTraceIds = [...findAllRelatedTraceIds(node).values()];
  allTraceIds.sort((f, s) => f - s);

  return `
          <div>
            ${createNodeDisplayName(node, createNodeDisplayNameString(node, sortedHistogramEntries))}
            <div style="background: ${nodeColor}; min-width: ${nodeWidthPx}px; border-width: 5px; 
                        border-style: solid; border-color: ${timeAnnotationColor};">
                <div style="width: 100%; height: 25px; text-align: center; color: ${graphColor.labelColor}; background-color: ${timeAnnotationColor}">
                    ${node.executionTime}
                </div>

                <div style="display: flex; flex-direction: row; margin-top: 10px;">
                    <div>
                        ${createPieChart(sortedHistogramEntries)}
                    </div>
                    <div style="margin-left: 10px;">
                        ${createAllocationsHistogram(softwareData)}
                    </div>
                </div>

                ${isPatternNode(node) ? createPatternInformation(node) : ""}

                <div style="display: flex; flex-direction: row;">
                    ${createTracesDescription(allTraceIds).join("\n")}
                </div>
            </div>
          </div>
         `;
}

function createNodeDisplayName(node: GraphNode, name: string): string {
  return `
      <div style="width: 100%; font-size: 22px; background-color: transparent; color: ${graphColor.labelColor}; text-align: left;">
          ${name}
      </div>
    `; 
}

function createAllocationsHistogram(softwareData: MergedSoftwareData): string {
  if (softwareData.allocations.size > 0) {
    return createPieChart(toSortedArray(softwareData.allocations));
  }
  
  return "";
}

function toSortedArray(map: Map<string, number>): [string, number][] {
  return [...map.entries()].toSorted((f: [string, number], s: [string, number]) => s[1] - f[1]);
}

function createPieChart(sortedHistogramEntries: [string, number][]): string {
  return `
    <div style="display: flex; flex-direction: row;">
       <div style='width: 65px; height: 65px;'
            class="graph-node-histogram"
            data-histogram-tooltip='${JSON.stringify(sortedHistogramEntries)}'>
          <svg-pie-chart style="pointer-events: none">
            ${createPieChartEntries(sortedHistogramEntries).join('\n')}
          </svg-pie-chart>
       </div>
    </div>
  `
}

function createRectangleHistogram(sortedHistogramEntries: [string, number][], aggregatedData: AggregatedData): string {
  let valuesSum: number = sortedHistogramEntries.map(x => x[1]).reduce((a, b) => a + b, 0);
  let divs: string[] = [];

  let heightPx = 35;
  
  for (let [type, count] of sortedHistogramEntries) {
    divs.push(`
      <div style="height: ${heightPx}px; width: ${(count / valuesSum) * 100}%; background-color: ${getOrCreateColor(type)};">
      </div>
    `);
  }

  let relativeAllocations: number = valuesSum / aggregatedData.totalAllocatedBytes;
  let borderColor = getPerformanceAnnotationColor(relativeAllocations);

  let borderWidthPx = 3;

  return `
    <div style="width: 100px; height: ${heightPx + 2 * borderWidthPx}px; display: flex; flex-direction: row;
                border-style: solid; border-width: ${borderWidthPx}px; border-color: ${borderColor}">
        ${divs.join("\n")}
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

function createNodeDisplayNameString(node: GraphNode, sortedHistogramEntries: [string, number][]): string {
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

function createPieChartEntries(sortedHistogramEntries: [string, number][]) {
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
  return createTracesStringDescription(tracesIds).map(t => `<div class="graph-node-trace-id-container">${t}</div>`)
}

function createTracesStringDescription(tracesIds: number[]) {
  let result = [];
  let index = 0;
  let groupStartIndex = 0;

  while (index < tracesIds.length) {
    groupStartIndex = index;
    while (index < tracesIds.length - 1 && tracesIds[index] + 1 === tracesIds[index + 1]) {
      index += 1;
    }

    if (groupStartIndex === index) {
      result.push(`${tracesIds[groupStartIndex]}`)
    } else {
      result.push(`${tracesIds[groupStartIndex]}..${tracesIds[index]}`)
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

  let patternInfos = extractPatternsInfo(node);
  for (let [_, info] of patternInfos) {
    let baseSequence = info.baseSequence.map((c, index) => `
        <div style="width: 20px; height: 20px; background-color: ${getOrCreateColor(c)}; margin-left: ${index == 0 ? 0 : 1}px;"></div>
    `);

    let tracesIds = info.traces.map(t => t.traceId);
    tracesIds.sort((f, s) => f - s);

    patterns.push(`
        <div style="display: flex; flex-direction: column; margin-top: 5px;">
            <div>
                At ${createTracesStringDescription(tracesIds)}:
            </div>
            <div style="display: flex; flex-direction: row;">
                ${baseSequence.join("\n")}
            </div>
        </div>
      `);
  }

  return `
    <div style="margin-top: 5px; margin-left: 5px;">
      <div>
        Pattern type: ${node.additionalData.find(d => d.patternInfo != null).patternInfo.patternKind}
      </div>
      <div>
        ${patterns.join("\n")}
      </div>
    </div>
  `
}

interface TracePatternInfo {
  traceId: number,
  repeatCount: number
}

interface GroupedPatternInfo {
  baseSequence: string[]
  traces: TracePatternInfo[]
}

function extractPatternsInfo(node: GraphNode): [string, GroupedPatternInfo][] {
  let result = new Map<string, GroupedPatternInfo>();

  for (let data of node.additionalData) {
    if (data.patternInfo != null) {
      let baseSequenceKey = data.patternInfo.baseSequence.join();
      if (!result.has(baseSequenceKey)) {
        result.set(baseSequenceKey, {traces: [], baseSequence: data.patternInfo.baseSequence});
      }

      let info = result.get(baseSequenceKey);
      info.traces.push({
        traceId: data.originalEventCoordinates.traceId,
        repeatCount: data.patternInfo.graph.nodes.length / data.patternInfo.baseSequence.length
      });
    }
  }

  return [...result.entries()];
}
