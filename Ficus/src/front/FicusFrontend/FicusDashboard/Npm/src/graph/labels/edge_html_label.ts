import {AggregatedData, GraphEdge, SoftwareEnhancementKind} from "../types";
import {MergedSoftwareData} from "../util";
import {
  createArrayPoolEnhancement,
  createEnhancementContainer,
  createRectangleHistogram,
  createThreadsEnhancement,
  getPercentExecutionTime,
  toSortedArray
} from "./util";

export function createEdgeHtmlLabel(edge: GraphEdge, enhancements: SoftwareEnhancementKind[]): string {
  let softwareData = edge.softwareData;
  if (softwareData == null) {
    return `
      <div style="margin-top: 120px;">
        ${createEdgeExecutionInfo(edge)}
      </div>
    `;
  }

  return `
    <div style="display: flex; flex-direction: column; align-items: center; margin-top: 80px;">
      <div style="display: flex; flex-direction: row; align-items: center;">
        ${enhancements.map(e => createEdgeEnhancement(softwareData, edge, e)).join("\n")}
      </div>
      ${createEdgeExecutionInfo(edge)}
    </div>
  `
}

function createEdgeExecutionInfo(edge: GraphEdge): string {
  let executionInfo = `
    <div style="font-size: 45px; font-weight: 900;">
      ${edge.weight} times
    </div>
  `;

  if (edge.executionTime != null) {
    executionInfo += `
      <div style="font-size: 45px; font-weight: 900;">
        ${edge.executionTime} (${getPercentExecutionTime(edge.executionTime, edge.aggregatedData.totalExecutionTime)}%)
      </div>
    `;
  }

  return executionInfo;
}

function createEdgeEnhancement(softwareData: MergedSoftwareData, edge: GraphEdge, enhancement: SoftwareEnhancementKind) {
  switch (enhancement) {
    case SoftwareEnhancementKind.Allocations:
      return createEdgeAllocationsEnhancement(softwareData, edge.aggregatedData);
    case SoftwareEnhancementKind.MethodsInlinings:
      return createMethodsInliningEnhancements(softwareData);
    case SoftwareEnhancementKind.MethodsLoadUnload:
      return createMethodsLoadUnloadEnhancement(softwareData);
    case SoftwareEnhancementKind.Exceptions:
      return createExceptionsEnhancement(softwareData);
    case SoftwareEnhancementKind.ArrayPools:
      return createEnhancementContainer("ArrayPools", createArrayPoolEnhancement(softwareData, edge.aggregatedData));
    case SoftwareEnhancementKind.Threads:
      return createEnhancementContainer("Threads", createThreadsEnhancement(softwareData));
    case SoftwareEnhancementKind.Http:
      return createHttpEnhancement(softwareData);
    default:
      return "";
  }
}

function createHttpEnhancement(softwareData: MergedSoftwareData): string {
  return `
    <div>
        ${createEdgeSoftwareEnhancementPart("HTTP", softwareData.httpRequests, null)}
    </div>
  `;
}

function createExceptionsEnhancement(softwareData: MergedSoftwareData): string {
  if (softwareData.exceptions.size == 0) {
    return "";
  }

  let totalSum = softwareData.exceptions.values().reduce((a, b) => a + b, 0);

  return `
    <div>
      ${createEdgeSoftwareEnhancementPart("Exceptions", softwareData.exceptions, totalSum)}
    </div>
  `
}

function createMethodsLoadUnloadEnhancement(softwareData: MergedSoftwareData): string {
  return `
    <div style="display: flex; flex-direction: row;">
      ${createEdgeSoftwareEnhancementPart("Load", softwareData.methodsLoads, null)}
      ${createEdgeSoftwareEnhancementPart("Unload", softwareData.methodsUnloads, null)}
    </div>
  `;
}

function createEdgeAllocationsEnhancement(softwareData: MergedSoftwareData, aggregatedData: AggregatedData): string {
  if (softwareData.allocations.size == 0) {
    return "";
  }

  return `
      <div>
        ${createEdgeSoftwareEnhancementPart("Allocations", softwareData.allocations, aggregatedData.totalAllocatedBytes)}
      </div>
    `;
}

function createMethodsInliningEnhancements(softwareData: MergedSoftwareData): string {
  return `
    <div style="display: flex; flex-direction: row;">
      ${createEdgeSoftwareEnhancementPart("Succeeded", softwareData.inliningSucceeded, null)}
      ${createEdgeSoftwareEnhancementPart("Failed", softwareData.inliningFailed, null)}
      ${createEdgeSoftwareEnhancementPart("Reasons", softwareData.inliningFailedReasons, null)}
    </div>
  `
}

function createEdgeSoftwareEnhancementPart(title: string, data: Map<string, number>, totalSum: number | null) {
  if (data.size == 0) {
    return '';
  }

  let valuesSum = data.values().reduce((a, b) => a + b, 0);
  let percent = totalSum != null ? ((valuesSum / totalSum) * 100).toFixed(2) : null;

  return `
    <div>
      <div style="width: fit-content; height: fit-content; display: flex; flex-direction: column; justify-content: center; align-items: center;">
        <div class="graph-title-label" style="display: flex; flex-direction: column;">
          <div>${title}</div>
          <div>${valuesSum} ${percent != null ? `(${percent}%)` : ""}</div>
        </div>
        ${createRectangleHistogram(toSortedArray(data), totalSum)}
      </div>
    </div>
  `
}
