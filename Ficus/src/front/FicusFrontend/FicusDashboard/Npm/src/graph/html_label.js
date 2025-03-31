import {belongsToRootSequence, getOrCreateColor, getSoftwareDataOrNull} from "./util";
import {darkTheme, graphColors} from "../colors";
import {nodeWidthPx, nodeHeightPx} from "./constants";

const graphColor = graphColors(darkTheme);

export function createHtmlLabel(node) {
  let softwareData = getSoftwareDataOrNull(node);
  if (softwareData == null) {
    return null;
  }

  let sortedHistogramEntries = softwareData.histogram.toSorted((f, s) => s.count - f.count);
  let eventClassesDescription = sortedHistogramEntries.map((entry) => {
    return `
        <div style="display: flex; flex-direction: row; height: 20px; align-items: center">
            <div style="width: 15px; height: 15px; background-color: ${getOrCreateColor(entry.name)}"></div>
            <div>${entry.name}</div>
        </div>
      `;
  });

  let summedCount = Math.max(...softwareData.histogram.map(entry => entry.count));
  let histogramDivs = sortedHistogramEntries.map((entry) => {
    let divWidth = (entry.count / summedCount) * 100;
    return `<div style="width: ${divWidth}%; height: 10px; background-color: ${getOrCreateColor(entry.name)}"></div>`;
  });

  let nodeColor = belongsToRootSequence(node) ? graphColor.rootSequenceColor : graphColor.nodeBackground;

  return `
          <div style="background: ${nodeColor}; min-width: ${nodeWidthPx}px; min-height: ${nodeHeightPx}px">
              <div style="width: 100%; text-align: center; color: ${graphColor.labelColor}">
                  ${node.label}
              </div>
              <div style="width: 100%; display: flex; flex-direction: row;">
                  ${histogramDivs.join('\n')}
              </div>
              <div style="width: 100%; display: flex; flex-direction: column;">
                  ${eventClassesDescription.join('\n')}
              </div>
          </div>
        `;
}
