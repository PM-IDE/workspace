import cytoscape from 'cytoscape';
import {darkTheme, graphColors, performanceColors} from "./colors";
import {calculateGradient, createBreadthFirstLayout, generateRandomColor} from "./utils";
import dagre from 'cytoscape-dagre';
import nodeHtmlLabel from 'cytoscape-node-html-label'

export default setDrawGraph;

const graphColor = graphColors(darkTheme);
const performanceColor = performanceColors(darkTheme);

function setDrawGraph() {
  window.drawGraph = function (id, graph, annotation) {
    cytoscape.use(dagre);
    nodeHtmlLabel(cytoscape);

    let cy = cytoscape(createCytoscapeOptions(id, graph, annotation));
    setNodeRenderer(cy);

    return cy;
  }
}

let colorsCache = {};

function getOrCreateColor(name) {
  if (!(name in colorsCache)) {
    colorsCache[name] = generateRandomColor();
  }

  return colorsCache[name];
}

function setNodeRenderer(cy) {
  cy.nodeHtmlLabel([
    {
      query: 'node',
      halign: 'center',
      valign: 'center',
      halignBox: 'center',
      valignBox: 'center',
      tpl: function (data) {
        if (data.softwareData == null) {
          return null;
        }

        let nodeWidth = 100;
        let nodeHeight = 100;

        let summedCount = Math.max(...data.softwareData.histogram.map(entry => entry.count));
        let histogramDivs = data.softwareData.histogram.toSorted((f, s) => s.count - f.count).map((entry) => {
            let divWidth = nodeWidth * (entry.count / summedCount);
            return `<div style="width: ${divWidth}px; height: 10px; background-color: ${getOrCreateColor(entry.name)}"></div>`;
          }
        );

        let nodeColor = data.softwareData.belongsToRootSequence ? graphColor.rootSequenceColor : graphColor.nodeBackground;

        return `
            <div style="width: ${nodeWidth}px; height: ${nodeHeight}px; background: ${nodeColor}">
                <div style="width: 100%; text-align: center; color: ${graphColor.labelColor}">
                    ${data.label}
                </div>
                <div style="width: 100%; display: flex; flex-direction: row;">
                    ${histogramDivs.join('\n')}
                </div>
            </div>
          `;
      }
    }
  ]);
}

function createCytoscapeOptions(id, graph, annotation) {
  return {
    container: document.getElementById(id),
    elements: createGraphElements(graph, annotation),
    layout: createBreadthFirstLayout(),
    style: [
      createNodeStyle(),
      createEdgeStyle(),
    ]
  }
}

function createNodeStyle() {
  return {
    selector: 'node',
    style: {
      'background-color': graphColor.nodeBackground,
      'label': 'data(label)',
      'text-valign': 'center',
      'text-halign': 'right',
      'shape': 'round-rectangle',
      'color': graphColor.labelColor
    }
  }
}

function createEdgeStyle() {
  return {
    selector: 'edge',
    style: {
      'label': "data(label)",
      'color': graphColor.labelColor,
      'width': "data(width)",
      'line-color': 'data(color)',
      'target-arrow-color': 'data(color)',
      'target-arrow-shape': 'triangle',
      'curve-style': 'bezier'
    }
  }
}

function createGraphElements(graph, annotation) {
  let elements = [];

  for (let node of graph.nodes) {
    elements.push({
      data: {
        label: node.data,
        id: node.id.toString(),
        softwareData: node.additionalData?.softwareData
      }
    })
  }

  elements.push(...createGraphEdgesElements(graph.edges, annotation));

  return elements;
}

function createGraphEdgesElements(edges, annotation) {
  let edgesMap = {};

  for (let edge of edges) {
    edgesMap[edge.id] = {};
  }

  processEdgesWidths(edges, edgesMap);

  if (annotation !== null && annotation.timeAnnotation !== null) {
    processTimeAnnotation(annotation.timeAnnotation, edges, edgesMap);
  }

  let elements = [];
  for (let edge of edges) {
    elements.push({
      data: {
        color: edgesMap[edge.id].color,
        width: edgesMap[edge.id].width,
        label: edge.data,
        id: edge.id,
        source: edge.fromNode.toString(),
        target: edge.toNode.toString(),
      }
    })
  }

  return elements;
}

function processEdgesWidths(edges, edgesMap) {
  const minWidth = 1;
  const maxWidth = 15;
  let maxWeight = Math.max(...edges.map(e => e.weight));

  for (let edge of edges) {
    let weightRatio = edge.weight / maxWeight
    let width = minWidth + (maxWidth - minWidth) * weightRatio;

    if (isNaN(width)) {
      width = 1;
    }

    edgesMap[edge.id].width = width;

    let blueMin = graphColor.blueMin;
    let blueMax = graphColor.blueMax;

    let greenMin = graphColor.greenMin;
    let greenMax = graphColor.greenMax;

    let redMin = graphColor.redMin;
    let redMax = graphColor.redMax;

    edgesMap[edge.id].color = calculateGradient(redMin, redMax, greenMin, greenMax, blueMin, blueMax, weightRatio);
  }
}

function processTimeAnnotation(annotation, edges, edgesMap) {
  let minTime = null;
  let maxTime = null;
  let idsToTime = {};

  for (let timeAnnotation of annotation.annotations) {
    let time = timeAnnotation.interval.nanoseconds;
    if (minTime === null || time < minTime) {
      minTime = time;
    }

    if (maxTime == null || time > maxTime) {
      maxTime = time;
    }

    idsToTime[timeAnnotation.entityId] = time;
  }

  for (let edge of edges) {
    let timeAnnotation = (idsToTime[edge.id] - minTime) / (maxTime - minTime);
    edgesMap[edge.id].timeAnnotation = timeAnnotation;

    let colorName = `color${(Math.floor(timeAnnotation * 10) % 100).toString()}`;
    edgesMap[edge.id].color = performanceColor[colorName];
  }
}