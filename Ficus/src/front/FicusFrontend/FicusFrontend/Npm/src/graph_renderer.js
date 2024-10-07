import cytoscape from 'cytoscape';
import klay from 'cytoscape-klay';
import {graphColors, lightTheme} from "./colors";

export default setDrawGraph;

const graphColor = graphColors(lightTheme);

function setDrawGraph() {
  window.drawGraph = function (id, graph) {
    cytoscape.use(klay);
    cytoscape(createCytoscapeOptions(id, graph));
  }
}

function createCytoscapeOptions(id, graph) {
  return {
    container: document.getElementById(id),
    elements: createGraphElements(graph),
    style: [
      createNodeStyle(),
      createEdgeStyle(),
    ],
    
    layout: {
      name: 'klay'
    }
  }
}

function createNodeStyle() {
  return {
    selector: 'node',
    style: {
      'background-color': graphColor.nodeBackground,
      'label': 'data(id)'
    }
  }
}

function createEdgeStyle() {
  return {
    selector: 'edge',
    style: {
      'width': 3,
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

  for (let edge of graph.edges) {
    elements.push({
      data: {
        label: edge.data,
        id: edge.fromNode.toString() + "::" + edge.toNode.toString(),
        source: edge.fromNode.toString(),
        target: edge.toNode.toString(),
      }
    })
  }

  return elements;
}