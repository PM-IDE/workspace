import {getOrCreateColor} from "../../utils";
import {getPerformanceAnnotationColor, MergedSoftwareData} from "../util";
import {AggregatedData} from "../types";

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

  return `
    <div>
      ${createNumberInformation("Allocated", softwareData.bufferAllocatedBytes.sum, aggregatedData.totalBufferAllocatedBytes)}
      ${createNumberInformation("Rented", softwareData.bufferRentedBytes.sum, aggregatedData.totalBufferRentedBytes)}
      ${createNumberInformation("Returned", softwareData.bufferReturnedBytes.sum, aggregatedData.totalBufferReturnedBytes)}
    </div>
  `;
}


function createNumberInformation(category: string, value: number, totalValue: number | null): string {
  return `
    <div style="display: flex; flex-direction: row; margin-top: 3px;">
      <div class="graph-content-container" style="background-color: ${getPerformanceAnnotationColor(value / totalValue)} !important;">
        ${category} ${value} bytes
      </div>
    </div>
  `;
}