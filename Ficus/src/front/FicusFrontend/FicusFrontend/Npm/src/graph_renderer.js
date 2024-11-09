import cytoscape from 'cytoscape';
import {graphColors, lightTheme} from "./colors";
import {calculateGradient, createBreadthFirstLayout, rgbToHex} from "./utils";
import dagre from 'cytoscape-dagre';

export default setDrawGraph;

const graphColor = graphColors(lightTheme);

function setDrawGraph() {
  window.drawGraph = function (id, graph, annotation) {
    cytoscape.use(dagre);
    return cytoscape(createCytoscapeOptions(id, graph, annotation));
  }
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
    }
  }
}

function createEdgeStyle() {
  return {
    selector: 'edge',
    style: {
      'label': "data(label)",
      'width': "data(width)",
      'line-color': 'data(color)',
      'target-arrow-color': 'data(color)',
      'target-arrow-shape': 'triangle',
      'curve-style': 'bezier'
    }
  }
}

function createGraphElements(graph, annotation) {
  console.log(graph, annotation);
  let elements = [];

  for (let node of graph.nodes) {
    elements.push({
      data: {
        label: node.data,
        id: node.id.toString(),
      }
    })
  }
  
  let propertiesMap = createEdgesPropertiesMap(graph.edges, annotation);

  for (let edge of graph.edges) {
    elements.push({
      data: {
        color: propertiesMap[edge.id].color,
        width: propertiesMap[edge.id].width,
        label: edge.data,
        id: edge.id,
        source: edge.fromNode.toString(),
        target: edge.toNode.toString(),
      }
    })
  }

  return elements;
}

function createEdgesPropertiesMap(edges, annotation) {
  let propertiesMap = {};
  let maxWeight = Math.max(...edges.map(e => e.weight));
  
  for (let edge of edges) {
    propertiesMap[edge.id] = {};
  }

  const minWidth = 5;
  const maxWidth = 15;

  for (let edge of edges) {
    let weightRatio = edge.weight / maxWeight
    let width = minWidth + (maxWidth - minWidth) * weightRatio;

    if (isNaN(width)) {
      width = 1;
    }
    
    propertiesMap[edge.id].width = width;

    let blueMin = graphColor.blueMin;
    let blueMax = graphColor.blueMax;

    let greenMin = graphColor.greenMin;
    let greenMax = graphColor.greenMax;

    let redMin = graphColor.redMin;
    let redMax = graphColor.redMax;
    
    propertiesMap[edge.id].color = calculateGradient(redMin, redMax, greenMin, greenMax, blueMin, blueMax, weightRatio);
  }

  if (annotation.timeAnnotation !== undefined) {
    let minTime = null;
    let maxTime = null;
    let idsToTime = {};

    for (let timeAnnotation of annotation.timeAnnotation.annotations) {
      let time = timeAnnotation.interval.nanoseconds;
      if (minTime === null || time < minTime) {
        minTime = time;
      }
      
      if (maxTime == null || timeAnnotation.interval > maxTime) {
        maxTime = time;
      }
      
      idsToTime[timeAnnotation.entityId] = time;
    }

    for (let edge of edges) {
      propertiesMap[edge.id].timeAnnotation = 1 - (maxTime - idsToTime[edge.id]) / (maxTime - minTime);
    }
  }
  
  return propertiesMap;
}