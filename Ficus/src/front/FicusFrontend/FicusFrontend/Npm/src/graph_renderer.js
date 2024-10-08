import cytoscape from 'cytoscape';
import {graphColors, lightTheme} from "./colors";

export default setDrawGraph;

const graphColor = graphColors(lightTheme);

function setDrawGraph() {
  window.drawGraph = function (id, graph) {
    cytoscape(createCytoscapeOptions(id, graph));
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
      'label': 'data(label)'
    }
  }
}

function createEdgeStyle() {
  return {
    selector: 'edge',
    style: {
      'label': "data(label)",
      'width': "data(width)",
      'line-color': graphColor.arcLine,
      'target-arrow-color': graphColor.arcLine,
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

  const minWidth = 3;
  const maxWidth = 8;
  
  let maxWeight = Math.max(...graph.edges.map(e => e.weight));
  
  for (let edge of graph.edges) {
    let width = minWidth + (maxWidth - minWidth) * (edge.weight / maxWeight);
    if (isNaN(width)) {
      width = 1;
    }

    elements.push({
      data: {
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