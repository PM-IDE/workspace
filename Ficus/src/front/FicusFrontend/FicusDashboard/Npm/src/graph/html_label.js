import {belongsToRootSequence, getOrCreateColor, getSoftwareDataOrNull} from "./util";
import {darkTheme, graphColors} from "../colors";
import {nodeWidthPx, nodeHeightPx} from "./constants";

const graphColor = graphColors(darkTheme);

export function createHtmlLabel(node) {
  let softwareData = getSoftwareDataOrNull(node);
  if (softwareData == null) {
    return null;
  }

  let summedCount = Math.max(...softwareData.histogram.map(entry => entry.count));
  let histogramDivs = softwareData.histogram.toSorted((f, s) => s.count - f.count).map((entry) => {
      let divWidth = nodeWidthPx * (entry.count / summedCount);
      return `<div style="width: ${divWidth}px; height: 10px; background-color: ${getOrCreateColor(entry.name)}"></div>`;
    }
  );

  let nodeColor = belongsToRootSequence(node) ? graphColor.rootSequenceColor : graphColor.nodeBackground;

  return `
          <div style="width: ${nodeWidthPx}px; height: ${nodeHeightPx}px; background: ${nodeColor}">
              <div style="width: 100%; text-align: center; color: ${graphColor.labelColor}">
                  ${node.label}
              </div>
              <div style="width: 100%; display: flex; flex-direction: row;">
                  ${histogramDivs.join('\n')}
              </div>
          </div>
        `;
}
