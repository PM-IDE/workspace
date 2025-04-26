import cytoscape from "cytoscape";
import {saveAs} from 'file-saver'

const svg = require('cytoscape-svg');
cytoscape.use(svg);

export function rgbToHex(r: number, g: number, b: number) {
  return "#" + (1 << 24 | r << 16 | g << 8 | b).toString(16).slice(1);
}

export function calculateGradient(redMin: number, redMax: number, greenMin: number, greenMax: number, blueMin: number, blueMax: number, weightRatio: number) {
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
  (<any>window).exportCytoscapeToSvg = exportCytoscapeToSvg;
}

function exportCytoscapeToSvg(cy: cytoscape.Core, fileName: string) {
  let svgContent = (<any>cy).svg({full: false});
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


let colorsCache: Map<string, string> = new Map<string, string>();

export function getOrCreateColor(name: string) {
  if (!colorsCache.has(name)) {
    colorsCache.set(name, generateRandomColor());
  }

  return colorsCache.get(name);
}

let nextId = 0;

export function createNextFrontendUniqueId(): number {
  nextId += 1;
  return nextId;
}