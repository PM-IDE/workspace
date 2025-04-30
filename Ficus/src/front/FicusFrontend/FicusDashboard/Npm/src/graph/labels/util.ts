import {getOrCreateColor} from "../../utils";
import {getPerformanceAnnotationColor} from "../util";

export function createPieChart(sortedHistogramEntries: [string, number][], performanceColor: string): string {
  return `
    <div style="display: flex; flex-direction: row;">
       <div style='width: 64px; height: 64px;' class="graph-node-histogram graph-tooltip-hover">
          <div style="width: 100%; height: 100%; border-style: solid; 
                      border-width: 10px; border-color: ${performanceColor}; border-radius: 32px;"
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

  let borderColor = null;

  if (totalSum != null) {
    let relativeAllocations: number = valuesSum / totalSum;
    borderColor = getPerformanceAnnotationColor(relativeAllocations);
  }

  let borderWidthPx = 10;

  return `
    <div style="width: 100px; height: ${heightPx + 2 * borderWidthPx}px; display: flex; flex-direction: row;
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