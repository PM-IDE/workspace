import {getOrCreateColor} from "../../utils";
import {getPerformanceAnnotationColor} from "../util";
import {AggregatedData, MergedEnhancementData, MergedSoftwareData, SoftwareEnhancementKind} from "../types";

export let fallBackPerformanceColor = "#3d3d3d";

export function createPieChart(sortedHistogramEntries: [string, number][], performanceColor: string | null): string {
  performanceColor = performanceColor == null ? fallBackPerformanceColor : performanceColor;

  return `
    <div style="display: flex; flex-direction: row;">
       <div style='width: 64px; height: 64px;' class="graph-node-histogram graph-tooltip-hover">
          <div style="width: 100%; height: 100%; border-style: solid;
                      border-width: 6px; border-color: ${performanceColor}; border-radius: 32px;"
               data-histogram-tooltip='${JSON.stringify(sortedHistogramEntries)}'
               data-tooltip-event-type='click'>
            <svg-pie-chart style="pointer-events: none">
              ${createPieChartEntries(sortedHistogramEntries).join('\n')}
            </svg-pie-chart>
          </div>
       </div>
    </div>
  `
}

export function createEnhancementContainer(title: string, content: string): string {
  if (content.length == 0) {
    return "";
  }

  return `
    <div class="graph-content-container">
      <div class="graph-title-label" style="margin-bottom: 3px;">${title}</div>
      ${content}
    </div>
  `
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

export function createRectangleHistogram(sortedHistogramEntries: [string, number][], totalSum: number | null): string {
  let valuesSum: number = sortedHistogramEntries.map(x => x[1]).reduce((a, b) => a + b, 0);
  let divs: string[] = [];

  let heightPx = 35;

  for (let [type, count] of sortedHistogramEntries) {
    divs.push(`
      <div style="height: ${heightPx}px; width: ${(count / valuesSum) * 100}%; background-color: ${getOrCreateColor(type)};
                  pointer-events: none;">
      </div>
    `);
  }

  let borderColor = fallBackPerformanceColor;

  if (totalSum != null) {
    let relativeAllocations: number = valuesSum / totalSum;
    borderColor = getPerformanceAnnotationColor(relativeAllocations);
  }

  let borderWidthPx = 6;

  return `
    <div style="width: 70px; height: ${heightPx + 2 * borderWidthPx}px; display: flex; flex-direction: row;
                border-style: solid; border-width: ${borderWidthPx}px; border-color: ${borderColor}"
         class="graph-tooltip-hover"
         data-histogram-tooltip='${JSON.stringify(sortedHistogramEntries)}'
         data-tooltip-event-type='click'>
        ${divs.join("\n")}
    </div>
  `
}

export function toSortedArray(map: Map<string, number>): [string, number][] {
  return [...map.entries()].toSorted((f: [string, number], s: [string, number]) => s[1] - f[1]);
}

export function createArrayPoolEnhancement(softwareData: MergedSoftwareData, aggregatedData: AggregatedData): string {
  if (softwareData.bufferRentedBytes.count == 0 && softwareData.bufferAllocatedBytes.count == 0 && softwareData.bufferReturnedBytes.count == 0) {
    return "";
  }

  const units = "bytes";
  return `
    <div>
      ${createNumberInformation("Allocated", units, softwareData.bufferAllocatedBytes.sum, aggregatedData.totalBufferAllocatedBytes)}
      ${createNumberInformation("Rented", units, softwareData.bufferRentedBytes.sum, aggregatedData.totalBufferRentedBytes)}
      ${createNumberInformation("Returned", units, softwareData.bufferReturnedBytes.sum, aggregatedData.totalBufferReturnedBytes)}
    </div>
  `;
}

export function createThreadsEnhancement(softwareData: MergedSoftwareData): string {
  if (softwareData.createdThreads.size == 0 && softwareData.terminatedThreads.size == 0) {
    return "";
  }

  const units = "threads";
  return `
    <div>
      ${createNumberInformation("Created", units, softwareData.createdThreads.size, null)}
      ${createNumberInformation("Terminated", units, softwareData.terminatedThreads.size, null)}
    </div>
  `;
}

export function createNumberInformation(category: string, units: string, value: number, totalValue: number | null): string {
  if (value == 0) {
    return "";
  }

  let percentString = getPercentExecutionTime(value, totalValue);
  percentString = percentString.length > 0 ? `, ${percentString}%` : percentString;

  return `
    <div style="display: flex; flex-direction: row; margin-top: 3px;">
      <div class="graph-content-container" style="background-color: ${getPerformanceAnnotationColor(value / totalValue)} !important;">
        ${category} ${value} ${units}${percentString}
      </div>
    </div>
  `;
}

export function getPercentExecutionTime(executionTime: number, totalExecutionTime: number): string {
  if (totalExecutionTime == 0) {
    return "0";
  }

  let percent = (executionTime / totalExecutionTime) * 100;

  return Number.isFinite(percent) ? percent.toFixed(2) : "";
}

export class EnhancementCreationResult {
  html: string
  group: string | null = null

  constructor(html: string, group: string | null = null) {
    this.html = html;
    this.group = group;
  }
}

export function createGroupedEnhancements(
  enhancements: SoftwareEnhancementKind[],
  enhancementData: MergedEnhancementData,
  aggregatedData: AggregatedData,
  enhancementFactory: (softwareData: MergedSoftwareData, aggregatedData: AggregatedData, enhancement: string) => EnhancementCreationResult
): string {
  // @ts-ignore
  let enhancementsHtmls: [SoftwareEnhancementKind, EnhancementCreationResult][] = enhancements
    .map(e => [e, enhancementFactory(enhancementData.softwareData, aggregatedData, e)])
    .filter(res => (<EnhancementCreationResult>res[1]).html.length > 0);

  if (enhancementsHtmls.length == 0) {
    return "";
  }

  let groups = new Map();
  let uniqueEnhancements: [SoftwareEnhancementKind, string][] = [];
  for (let [e, result] of enhancementsHtmls) {
    if (result.group == null) {
      uniqueEnhancements.push([e, result.html]);
      continue;
    }

    if (!groups.has(result.group)) {
      groups.set(result.group, []);
    }

    groups.get(result.group).push(result.html);
  }
  
  return uniqueEnhancements
    .map(([e, html]) => createEnhancementContainer(e, html))
    .concat(...groups.entries().map(kv => createEnhancementContainer(kv[0], kv[1].join("\n"))))
    .join("\n");
}