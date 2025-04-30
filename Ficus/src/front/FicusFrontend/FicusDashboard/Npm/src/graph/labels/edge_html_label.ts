import {AggregatedData, GraphEdge, SoftwareEnhancementKind} from "../types";
import {MergedSoftwareData} from "../util";
import {createRectangleHistogram, toSortedArray} from "./util";

export function createEdgeHtmlLabel(edge: GraphEdge, enhancement: SoftwareEnhancementKind): string {
  let softwareData = edge.softwareData;
  if (softwareData == null) {
    return "";
  }

  switch (enhancement) {
    case SoftwareEnhancementKind.Allocations:
      return createEdgeAllocationsEnhancement(softwareData, edge.aggregatedData);
    case SoftwareEnhancementKind.MethodsInlinings:
      return createMethodsInliningEnhancements(softwareData);
    default:
      return "";
  }
}

function createEdgeAllocationsEnhancement(softwareData: MergedSoftwareData, aggregatedData: AggregatedData): string {
  if (softwareData.allocations.size == 0) {
    return "";
  }

  return `
      <div>
        ${createRectangleHistogram(toSortedArray(softwareData.allocations), aggregatedData.totalAllocatedBytes)}
      </div>
    `
}

function createMethodsInliningEnhancements(softwareData: MergedSoftwareData): string {
  return `
    <div style="display: flex; flex-direction: row;">
      ${createSoftwareEnhancementPart("Succeeded", softwareData.inliningSucceeded)} 
      ${createSoftwareEnhancementPart("Failed", softwareData.inliningFailed)} 
      ${createSoftwareEnhancementPart("Failed Reasons", softwareData.inliningFailedReasons)} 
    </div>
  `
}

function createSoftwareEnhancementPart(title: string, data: Map<string, number>) {
  if (data.size == 0) {
    return '';
  }

  return `
    <div>
      <div style="width: fit-content; display: flex; flex-direction: column; justify-content: center; align-items: center;">
        <div>${title}</div>
        ${createRectangleHistogram(toSortedArray(data), null)}
      </div>
    </div>
  `
}
