import {AggregatedData, GraphEdge, SoftwareEnhancementKind} from "../types";
import {MergedSoftwareData} from "../util";
import {createArrayPoolEnhancement, createRectangleHistogram, toSortedArray} from "./util";

export function createEdgeHtmlLabel(edge: GraphEdge, enhancements: SoftwareEnhancementKind[]): string {
  let softwareData = edge.softwareData;
  if (softwareData == null) {
    return "";
  }

  return `
    <div style="display: flex; flex-direction: column; align-items: center; margin-top: 20px;">
      <div style="display: flex; flex-direction: row; align-items: center;">
        ${enhancements.map(e => createEdgeEnhancement(softwareData, edge, e)).join("\n")}
      </div>
      <div>
        Execution time: ${edge.executionTime}
      </div>
      <div>
        Executed ${edge.weight} times
      </div>
    </div>
  `
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
      return createArrayPoolEnhancement(softwareData, edge.aggregatedData);
    default:
      return "";
  }
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

  return `
    <div>
      <div style="width: fit-content; height: fit-content; display: flex; flex-direction: column; justify-content: center; align-items: center;">
        <div class="graph-title-label">${title}</div>
        ${createRectangleHistogram(toSortedArray(data), totalSum)}
      </div>
    </div>
  `
}
