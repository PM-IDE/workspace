import {AggregatedData, GraphEdge, MergedEnhancementData, MergedSoftwareData, SoftwareEnhancementKind} from "../types";
import {
  createArrayPoolEnhancement,
  createEnhancementContainer, createGroupedEnhancements, createNumberInformation,
  createRectangleHistogram,
  createThreadsEnhancement, EnhancementCreationResult,
  getPercentExecutionTime,
  toSortedArray
} from "./util";
import {isNullOrEmpty} from "../../utils";

export function createEdgeHtmlLabel(edge: GraphEdge, enhancements: SoftwareEnhancementKind[]): string {
  let enhancementData = edge.enhancementData;
  if (enhancementData == null) {
    return `
      <div style="margin-top: 140px;">
        ${createEdgeExecutionInfo(edge)}
      </div>
    `;
  }

  return `
    <div style="display: flex; flex-direction: column; align-items: center; margin-top: 80px;">
      <div style="display: flex; flex-direction: row; align-items: center;">
        ${createGroupedEnhancements(enhancements, enhancementData, edge.aggregatedData, false, createEdgeEnhancement)}
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
        ${getPercentExecutionTime(edge.executionTime, edge.aggregatedData.totalExecutionTime)}%
      </div>
    `;
  }

  return executionInfo;
}

function createEdgeEnhancement(
  softwareData: MergedSoftwareData,
  aggregatedData: AggregatedData,
  enhancement: SoftwareEnhancementKind
): EnhancementCreationResult {
  switch (enhancement) {
    case "Allocations":
      return new EnhancementCreationResult(createEdgeAllocationsEnhancement(softwareData, aggregatedData), null);
    case "Methods Inlinings":
      return new EnhancementCreationResult(createMethodsInliningEnhancements(softwareData), null);
    case "Methods (Un)Loads":
      return new EnhancementCreationResult(createMethodsLoadUnloadEnhancement(softwareData), null);
    case "Exceptions":
      return new EnhancementCreationResult(createExceptionsEnhancement(softwareData), null);
    case "ArrayPools":
      return new EnhancementCreationResult(createEnhancementContainer("ArrayPools", createArrayPoolEnhancement(softwareData, aggregatedData)), null);
    case "Threads":
      return new EnhancementCreationResult(createEnhancementContainer("Threads", createThreadsEnhancement(softwareData)), null);
    case "Http":
      return new EnhancementCreationResult(createHttpEnhancement(softwareData), null);
    default: {
      if (softwareData.histograms.has(enhancement)) {
        let histogram = softwareData.histograms.get(enhancement);
        let globalSum = aggregatedData.globalSoftwareData.histograms.get(enhancement).value.values().reduce((a, b) => a + b, 0);

        let html = createEdgeSoftwareEnhancementPart(
          !isNullOrEmpty(histogram.group) ? enhancement : "",
          histogram.value,
          histogram.units,
          globalSum
        );

        return new EnhancementCreationResult(html, histogram.group);
      }

      if (softwareData.counters.has(enhancement)) {
        let counter = softwareData.counters.get(enhancement);
        let html = createNumberInformation(
          !isNullOrEmpty(counter.group) ? enhancement : "",
          counter.units,
          counter.value,
          aggregatedData.globalSoftwareData.counters.get(enhancement).value
        );

        if (counter.group == null) {
          html = createEnhancementContainer(enhancement, html);
        }

        return new EnhancementCreationResult(html, counter.group);
      }

      if (softwareData.activitiesDurations.has(enhancement)) {
        let duration = softwareData.activitiesDurations.get(enhancement);

        let html = createNumberInformation(
          !isNullOrEmpty(duration.group) ? enhancement : "",
          duration.units,
          duration.value,
          aggregatedData.globalSoftwareData.activitiesDurations.get(enhancement).value
        );

        if (duration.group == null) {
          html = createEnhancementContainer(enhancement, html);
        }

        return new EnhancementCreationResult(html, duration.group);
      }

      return new EnhancementCreationResult("", null);
    }
  }
}

function createHttpEnhancement(softwareData: MergedSoftwareData): string {
  if (softwareData.httpRequests.size == 0) {
    return "";
  }

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
      ${createEdgeSoftwareEnhancementPart("Exceptions", softwareData.exceptions, null, totalSum)}
    </div>
  `
}

function createMethodsLoadUnloadEnhancement(softwareData: MergedSoftwareData): string {
  if (softwareData.methodsLoads.size == 0 && softwareData.methodsUnloads.size == 0) {
    return "";
  }

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
        ${createEdgeSoftwareEnhancementPart("Allocations", softwareData.allocations, "bytes", aggregatedData.totalAllocatedBytes)}
      </div>
    `;
}

function createMethodsInliningEnhancements(softwareData: MergedSoftwareData): string {
  if (softwareData.inliningSucceeded.size == 0 && softwareData.inliningFailed.size == 0 && softwareData.inliningFailedReasons.size == 0) {
    return "";
  }

  return `
    <div style="display: flex; flex-direction: row;">
      ${createEdgeSoftwareEnhancementPart("Succeeded", softwareData.inliningSucceeded, null)}
      ${createEdgeSoftwareEnhancementPart("Failed", softwareData.inliningFailed, null)}
      ${createEdgeSoftwareEnhancementPart("Reasons", softwareData.inliningFailedReasons, null)}
    </div>
  `
}

function createEdgeSoftwareEnhancementPart(
  title: string,
  data: Map<string, number>,
  units: string | null = null,
  totalSum: number | null = null
) {
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
          <div>${valuesSum}${units != null ? ` ${units}` : ""} ${percent != null ? `(${percent}%)` : ""}</div>
        </div>
        ${createRectangleHistogram(toSortedArray(data), totalSum)}
      </div>
    </div>
  `
}
