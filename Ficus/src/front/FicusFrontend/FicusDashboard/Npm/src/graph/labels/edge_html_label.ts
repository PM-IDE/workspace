import {AggregatedData, GraphEdge, MergedSoftwareData, SoftwareEnhancementKind} from "../types";
import {
  createEnhancementContainer, createGroupedEnhancements, createNumberInformation,
  createRectangleHistogram, createTimeSpanString,
  EnhancementCreationResult,
  getPercentExecutionTime,
  toSortedArray
} from "./util";
import {isNullOrEmpty} from "../../utils";
import {GrpcDurationKind} from "../../protos/ficus/GrpcDurationKind";

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
    <div style="font-size: 25px; font-weight: 900;">
      ${edge.weight} times
    </div>
  `;

  if (edge.executionTime != null) {
    executionInfo += `
      <div style="font-size: 25px; font-weight: 900;">
        ${createTimeSpanString(edge.executionTime, GrpcDurationKind.Nanos)}
      </div>
      <div style="font-size: 25px; font-weight: 900;">
        ${getPercentExecutionTime(edge.executionTime, edge.aggregatedData.totalExecutionTimeNs)}%
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
  if (softwareData.histograms.has(enhancement)) {
    let histogram = softwareData.histograms.get(enhancement);
    let globalSum = aggregatedData.globalSoftwareData.histograms.get(enhancement).value.values().reduce((a, b) => a + b, 0);

    let html = createEdgeSoftwareEnhancementPart(
      enhancement,
      histogram.value,
      histogram.units,
      globalSum
    );

    return new EnhancementCreationResult(html, histogram.group);
  }

  if (softwareData.counters.has(enhancement)) {
    let counter = softwareData.counters.get(enhancement);
    let html = createNumberInformation(
      enhancement,
      counter.units,
      counter.value,
      counter.value.toString(),
      aggregatedData.globalSoftwareData.counters.get(enhancement).value
    );

    if (isNullOrEmpty(counter.group)) {
      html = createEnhancementContainer(enhancement, html);
    }

    return new EnhancementCreationResult(html, counter.group, false);
  }

  if (softwareData.activitiesDurations.has(enhancement)) {
    let duration = softwareData.activitiesDurations.get(enhancement);

    let html = createNumberInformation(
      enhancement,
      duration.units,
      duration.value.value,
      createTimeSpanString(duration.value.value, duration.value.kind),
      aggregatedData.globalSoftwareData.activitiesDurations.get(enhancement).value.value
    );

    if (isNullOrEmpty(duration.group)) {
      html = createEnhancementContainer(enhancement, html);
    }

    return new EnhancementCreationResult(html, duration.group, false);
  }

  return new EnhancementCreationResult("", null);
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
        <div class="graph-title-label" style="display: flex; flex-direction: column; justify-content: center; align-items: center;">
          <div>${title}</div>
          <div>
            ${valuesSum}${units != null ? ` ${units}` : ""}
          </div>
          <div>
            ${percent != null ? `${percent}%` : ""}
          </div>
        </div>
        ${createRectangleHistogram(toSortedArray(data), totalSum)}
      </div>
    </div>
  `
}
