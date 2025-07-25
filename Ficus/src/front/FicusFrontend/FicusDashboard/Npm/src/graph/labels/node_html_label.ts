import {
  belongsToRootSequence,
  findAllRelatedTraceIds,
  getPerformanceAnnotationColor
} from "../util";
import {darkTheme, graphColors} from "../../colors";
import {nodeHeightPx, nodeWidthPx} from "../constants";
import {getOrCreateColor} from "../../utils";
import {AggregatedData, GraphNode, MergedSoftwareData, SoftwareEnhancementKind} from "../types";
import {GrpcUnderlyingPatternKind} from "../../protos/ficus/GrpcUnderlyingPatternKind";
import {
  createArrayPoolEnhancement,
  createEnhancementContainer, createNumberInformation,
  createPieChart,
  createThreadsEnhancement,
  getPercentExecutionTime,
  toSortedArray
} from "./util";

const graphColor = graphColors(darkTheme);

export function createNodeHtmlLabelId(frontendId: number): string {
  return `node-html-label-${frontendId}`;
}

export function createNodeHtmlLabel(node: GraphNode, enhancements: SoftwareEnhancementKind[]) {
  let softwareData = node.softwareData;
  let label_id = createNodeHtmlLabelId(node.frontendId);

  if (softwareData == null) {
    return `
        <div id="${label_id}">
            ${createNodeDisplayName(node, node.label)}
            <div style='min-width: ${nodeWidthPx}px; min-height: ${nodeHeightPx}px;
                background-color: ${graphColor.rootSequenceColor}'>
            </div>
        </div>
    `;
  }

  let sortedHistogramEntries = toSortedArray(softwareData.histogram);
  let nodeColor = belongsToRootSequence(node) ? graphColor.rootSequenceColor : graphColor.nodeBackground;
  let timeAnnotationColor = getPerformanceAnnotationColor(node.executionTime / node.aggregatedData.totalExecutionTime);
  let allTraceIds = [...findAllRelatedTraceIds(node).values()];
  allTraceIds.sort((f, s) => f - s);

  return `
          <div id="${label_id}">
            ${createNodeDisplayName(node, createNodeDisplayNameString(node, sortedHistogramEntries))}
            <div style="background: ${nodeColor}; min-width: ${nodeWidthPx}px; border-width: 5px; 
                        border-style: solid; border-color: ${timeAnnotationColor};">
              <div style="width: 100%; height: 25px; text-align: center; color: ${graphColor.labelColor}; background-color: ${timeAnnotationColor}">
                  Exec. time: ${node.executionTime} (${getPercentExecutionTime(node.executionTime, node.aggregatedData.totalExecutionTime)}%)
              </div>

              <div style="padding-left: 10px;">
                <div style="display: flex; flex-wrap: wrap; margin-top: 10px; gap: 10px;">
                  ${createEventClassesPieChart(softwareData.histogram)}
                  ${createNodeEnhancements(enhancements, softwareData, node.aggregatedData)}
                  ${isPatternNode(node) ? createPatternInformation(node) : ""}
                  ${isMultithreadedNode(node) ? createMultithreadedNodeInformation(node) : ""}
                </div>

                <div style="display: flex; flex-direction: row;">
                  ${createTracesDescription(allTraceIds).join("\n")}
                </div>
              </div>
            </div>
          </div>
         `;
}

function createEventClassesPieChart(data: Map<string, number>) {
  if (data.size == 0) {
    return "";
  }

  return `
    <div class="graph-content-container" style="display: flex; flex-direction: column">
      <div class="graph-title-label">
        Event classes:
      </div>
      <div style="margin-top: 5px;">
        ${createPieChart(toSortedArray(data), null)}
      </div>
    </div>
  `;
}

function createNodeEnhancements(enhancements: SoftwareEnhancementKind[], softwareData: MergedSoftwareData, aggregatedData: AggregatedData): string {
  // @ts-ignore
  let enhancementsHtmls: [SoftwareEnhancementKind, string][] = enhancements
    .map(e => [e, createNodeEnhancementContent(softwareData, aggregatedData, e)])
    .filter(res => (<any>res[1]).length > 0);

  if (enhancementsHtmls.length == 0) {
    return "";
  }

  return enhancementsHtmls
    .map(([e, html]) => createEnhancementContainer(e, html))
    .join("\n");
}

function createNodeEnhancementContent(softwareData: MergedSoftwareData, aggregatedData: AggregatedData, enhancement: SoftwareEnhancementKind): string {
  switch (enhancement) {
    case "Allocations":
      return createNodeAllocationsEnhancement(softwareData, aggregatedData);
    case "MethodsInlinings":
      return createMethodsInliningEnhancement(softwareData);
    case "MethodsLoadUnload":
      return createMethodsLoadUnloadEnhancement(softwareData);
    case "ArrayPools":
      return createArrayPoolEnhancement(softwareData, aggregatedData);
    case "Exceptions":
      return createExceptionEnhancement(softwareData);
    case "Threads":
      return createThreadsEnhancement(softwareData);
    case "Http":
      return createHttpEnhancement(softwareData);
    default: {
      if (softwareData.histograms.has(enhancement)) {
        let sum = softwareData.histograms.get(enhancement).value.values().reduce((a, b) => a + b, 0);
        return createSoftwareEnhancementHistogram(
          enhancement,
          softwareData.histograms.get(enhancement).value,
          getPerformanceAnnotationColor(sum / aggregatedData.totalHistogramsCount.get(enhancement))
        );
      }

      if (softwareData.counters.has(enhancement)) {
        return createNumberInformation(
          "",
          softwareData.counters.get(enhancement).units,
          softwareData.counters.get(enhancement).value,
          aggregatedData.totalCountersCount.get(enhancement)
        );
      }

      return "";
    }
  }
}

function createHttpEnhancement(softwareData: MergedSoftwareData): string {
  if (softwareData.httpRequests.size == 0) {
    return "";
  }

  return `
    <div>
      ${createSoftwareEnhancementHistogram("Requests", softwareData.httpRequests, null)}
    </div>
  `
}

function createExceptionEnhancement(softwareData: MergedSoftwareData): string {
  if (softwareData.exceptions.size == 0) {
    return "";
  }

  return `
    <div>
      ${createSoftwareEnhancementHistogram("Exceptions", softwareData.exceptions, getPerformanceAnnotationColor(1))}
    </div>
  `
}

function createMethodsLoadUnloadEnhancement(softwareData: MergedSoftwareData): string {
  if (softwareData.methodsUnloads.size == 0 && softwareData.methodsLoads.size == 0) {
    return "";
  }

  return `
    <div style="display: flex; flex-direction: row;">
      ${createSoftwareEnhancementHistogram("Load", softwareData.methodsLoads, null)} 
      ${createSoftwareEnhancementHistogram("Unload", softwareData.methodsUnloads, null)}
    </div> 
  `;
}

function createMethodsInliningEnhancement(softwareData: MergedSoftwareData): string {
  if (softwareData.inliningSucceeded.size == 0 && softwareData.inliningFailed.size == 0 && softwareData.inliningFailedReasons.size == 0) {
    return "";
  }

  return `
    <div style="display: flex; flex-direction: row;">
      ${createSoftwareEnhancementHistogram("Succeeded", softwareData.inliningSucceeded, null)} 
      ${createSoftwareEnhancementHistogram("Failed", softwareData.inliningFailed, null)} 
      ${createSoftwareEnhancementHistogram("Failed Reasons", softwareData.inliningFailedReasons, null)} 
    </div>
  `;
}

function createSoftwareEnhancementHistogram(title: string, data: Map<string, number>, perfColor: null | string) {
  if (data.size == 0) {
    return "";
  }

  return `
      <div style="width: fit-content; display: flex; flex-direction: column; justify-content: center; align-items: center;">
        <div class="graph-title-label graph-title-label-lighter" style="display: flex; flex-direction: column;">
          <div>
            ${title}
          </div>
          <div>
            (${data.values().reduce((a, b) => a + b, 0)})
          </div>
          <div>
            ${createPieChart(toSortedArray(data), perfColor)}
          </div>
        </div>
      </div>
  `;
}

function createNodeDisplayName(node: GraphNode, name: string): string {
  return `
      <div style="font-size: 60px; font-weight: 900; 
                  background-color: transparent; color: ${graphColor.labelColor}; text-align: left;">
          ${name}
      </div>
    `;
}

function createNodeAllocationsEnhancement(softwareData: MergedSoftwareData, aggregatedData: AggregatedData): string {
  if (softwareData.allocations.size > 0) {
    let relativeAllocatedBytes = softwareData.allocations.values().reduce((a, b) => a + b, 0) / aggregatedData.totalAllocatedBytes;
    let color = getPerformanceAnnotationColor(relativeAllocatedBytes);
    let totalAlloc = softwareData.allocations.values().reduce((a, b) => a + b, 0);
    let percent = ((totalAlloc / aggregatedData.totalAllocatedBytes) * 100).toFixed(2);

    return `
        <div>
          ${totalAlloc} (${percent}%)
        </div>
        <div>
          ${createPieChart(toSortedArray(softwareData.allocations), color)}
        </div>
      `
  }

  return "";
}

function createNodeDisplayNameString(node: GraphNode, sortedHistogramEntries: [string, number][]): string {
  let nodeNameParts: string[] = [];
  for (let i = 0; i < Math.min(3, sortedHistogramEntries.length); ++i) {
    nodeNameParts.push(`
      <div style="width: fit-content; text-overflow: ellipsis;">
        ${sortedHistogramEntries[i][0]}
      </div>
    `);
  }

  if (nodeNameParts.length == 0) {
    nodeNameParts.push(`<div>${node.label}</div>`)
  }

  return nodeNameParts.join("\n");
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

function isMultithreadedNode(node: GraphNode): boolean {
  return node.additionalData.find(d => d.multithreadedFragment != null) != null;
}

function isPatternNode(node: GraphNode): boolean {
  return node.additionalData.find(d => d.patternInfo != null) != null;
}

function createMultithreadedNodeInformation(node: GraphNode): string {
  let multithreaded_logs_htmls = [];
  for (let data of node.additionalData) {
    if (data.multithreadedFragment != null) {
      let log = data.multithreadedFragment.multithreadedLog.traces.map(t => t.events.map(e => e.name));
      let patterns = log.map(t => createSimpleTraceView(t.map(((e, index) => createSimpleEventView(e, index != 0)))));

      multithreaded_logs_htmls.push(`
        <div style="display: flex; flex-direction: column; margin-top: 5px;">
            <div>
                At ${createTracesStringDescription([data.originalEventCoordinates.traceId])}:
            </div>
            ${patterns.join("\n")}
        </div>
      `);
    }
  }

  return `
    <div class="graph-content-container">
      <div style="display: flex; flex-direction: row;" class="graph-title-label">
        <div>Multithreaded parts:</div>
      </div>
      <div>
        ${multithreaded_logs_htmls.join("\n")}
      </div>
    </div>
  `
}

function createSimpleEventView(name: string, addMargin: boolean): string {
  return `
    <div style="width: 18px; height: 18px; background-color: ${getOrCreateColor(name)}; margin-left: ${addMargin ? 1 : 0}px;
                border-style: solid; border-width: 1px; border-color: ${getOrCreateColor(name)}"
         class="graph-tooltip-hover"
         data-histogram-tooltip='${JSON.stringify([[name, 1]])}'
         data-tooltip-event-type='mouseover'>
    </div>
  `
}

function createSimpleTraceView(eventsHtmls: string[]): string {
  return `
    <div style="display: flex; flex-direction: row;">
      ${eventsHtmls.join("\n")}
    </div>
  `
}

function createPatternInformation(node: GraphNode): string {
  let patterns: string[] = [];

  let patternInfos = extractPatternsInfo(node);
  for (let [_, info] of patternInfos) {
    let baseSequence = info.baseSequence.map((e, index) => createSimpleEventView(e, index != 0));

    let tracesIds = info.traces.map(t => t.traceId);
    tracesIds.sort((f, s) => f - s);

    patterns.push(`
        <div style="display: flex; flex-direction: column; margin-top: 5px;">
            <div>
                At ${createTracesStringDescription(tracesIds)}:
            </div>
            ${createSimpleTraceView(baseSequence)}
        </div>
      `);
  }

  let propertyIndex = <number><unknown>node.additionalData.find(d => d.patternInfo != null).patternInfo.patternKind;

  return `
    <div class="graph-content-container">
      <div style="display: flex; flex-direction: row;" class="graph-title-label">
        <div>Pattern type:</div>
        <div style="margin-left: 5px;">${Object.values(GrpcUnderlyingPatternKind)[propertyIndex]}</div>
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
