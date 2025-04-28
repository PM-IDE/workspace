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
        ${createRectangleHistogram(toSortedArray(softwareData.allocations), aggregatedData)}
      </div>
    `
}
