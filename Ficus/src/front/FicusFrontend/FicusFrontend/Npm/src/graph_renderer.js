import cytoscape from 'cytoscape';
import {graphColors, lightTheme} from "./colors";
import {calculateGradient, createBreadthFirstLayout, rgbToHex} from "./utils";
import dagre from 'cytoscape-dagre';

export default setDrawGraph;

const graphColor = graphColors(lightTheme);

function setDrawGraph() {
  window.drawGraph = function (id, graph) {
    cytoscape.use(dagre);
    return cytoscape(createCytoscapeOptions(id, graph));
  }
}

function createCytoscapeOptions(id, graph) {
  return {
    container: document.getElementById(id),
    elements: createGraphElements(graph),
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

function createGraphElements(graph) {
  let elements = [];

  for (let node of graph.nodes) {
    elements.push({
      data: {
        label: node.data,
        id: node.id.toString(),
      }
    })
  }

  const minWidth = 5;
  const maxWidth = 15;
  
  let maxWeight = Math.max(...graph.edges.map(e => e.weight));
  
  for (let edge of graph.edges) {
    let weightRatio = edge.weight / maxWeight
    let width = minWidth + (maxWidth - minWidth) * weightRatio;
    if (isNaN(width)) {
      width = 1;
    }
    
    let blueMin = graphColor.blueMin;
    let blueMax = graphColor.blueMax;
    
    let greenMin = graphColor.greenMin;
    let greenMax = graphColor.greenMax;
    
    let redMin = graphColor.redMin;
    let redMax = graphColor.redMax;

    elements.push({
      data: {
        color: calculateGradient(redMin, redMax, greenMin, greenMax, blueMin, blueMax, weightRatio),
        width: width,
        label: edge.data,
        id: edge.fromNode.toString() + "::" + edge.toNode.toString(),
        source: edge.fromNode.toString(),
        target: edge.toNode.toString(),
      }
    })
  }

  return elements;
}