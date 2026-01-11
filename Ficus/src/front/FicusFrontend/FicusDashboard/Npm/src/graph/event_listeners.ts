import tippy, {Instance, Props} from "tippy.js";
import {getOrCreateColor} from "../utils";
import {createTippyTooltipProps} from "../tooltip";

export function setGraphEventListeners() {
  addEventListener("mouseover", event => {
    executeIfHasTooltip(event, (entries, element) => {
      createTooltip(element, entries, undefined, false);
    })
  })

  addEventListener("click", event => {
    executeIfHasTooltip(event, (entries, element) => {
      let tooltip = createTooltip(element, entries, "manual", true);
      tooltip.show();
    })
  });
}

function executeIfHasTooltip(event: MouseEvent, handler: (entries: [string, number][], element: HTMLElement) => void) {
  let element = event.target;

  if (element instanceof HTMLElement) {
    let rawData = element.dataset.histogramTooltip;

    if (rawData != null && event.type === element.dataset.tooltipEventType) {
      let histogramEntries: [string, number][] = JSON.parse(rawData);
      handler(histogramEntries, element);
    }
  }
}

function createTooltip(element: HTMLElement, histogramEntries: [string, number][], trigger: string, interactive: boolean): Instance {
  let props = createTippyTooltipProps(createEventClassesDescription(histogramEntries).join('\n'));
  props.trigger = trigger;
  props.interactive = interactive;

  return tippy(element, props);
}


function createEventClassesDescription(sortedHistogramEntries: [string, number][]): string[] {
  let currentSum = sortedHistogramEntries.reduce((a, b) => a + b[1], 0);

  return sortedHistogramEntries.map((entry) => {
    let percent = ((entry[1] / currentSum) * 100).toFixed(2);

    return `
        <div style="display: flex; flex-direction: row; width: fit-content; height: fit-content; align-items: center">
            <div style="width: 15px; height: 15px; background-color: ${getOrCreateColor(entry[0])}"></div>
            <div style="margin-left: 5px; width: fit-content; white-space: nowrap">(${entry[1]}, ${percent}%)</div>
            <div style="margin-left: 5px; width: fit-content; white-space: nowrap">${entry[0]}</div>
        </div>
      `;
  });
}