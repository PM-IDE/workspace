import svg from 'cytoscape-svg';
import cytoscape from "cytoscape";
import {saveAs} from 'file-saver'

cytoscape.use(svg);

export function createDagreLayout() {
  return {
    name: 'dagre',
    rankDir: 'LR',
    nodeDimensionsIncludeLabels: true,
    ranker: 'tight-tree',
    spacingFactor: 2
  }
}

export function rgbToHex(r, g, b) {
  return "#" + (1 << 24 | r << 16 | g << 8 | b).toString(16).slice(1);
}

export function calculateGradient(redMin, redMax, greenMin, greenMax, blueMin, blueMax, weightRatio) {
  let blue = Math.floor(blueMin + (blueMax - blueMin) * (1 - weightRatio));
  if (isNaN(blue)) {
    blue = blueMin;
  }

  let green = Math.floor(greenMin + (greenMax - greenMin) * (1 - weightRatio));
  if (isNaN(green)) {
    green = greenMin;
  }

  let red = Math.floor(redMin + (redMax - redMin) * (1 - weightRatio));
  if (isNaN(red)) {
    red = redMin;
  }

  return rgbToHex(red, green, blue);
}

export function setUtilitiesFunctions() {
  window.exportCytoscapeToSvg = exportCytoscapeToSvg;
}

function exportCytoscapeToSvg(cy, fileName) {
  let svgContent = cy.svg({full: false});
  let blob = new Blob([svgContent], {type: "image/svg+xml;charset=utf-8"});

  saveAs(blob, fileName);
}

export function generateRandomColor() {
  let letters = '0123456789ABCDEF';
  let color = '#';
  for (let i = 0; i < 6; i++) {
    color += letters[Math.floor(Math.random() * 16)];
  }

  return color;
}