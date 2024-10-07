import cytoscape from 'cytoscape';
import klay from 'cytoscape-klay';

export default setDrawGraph;

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
      {
        selector: 'node',
        style: {
          'background-color': '#666',
          'label': 'data(id)'
        }
      },

      {
        selector: 'edge',
        style: {
          'width': 3,
          'line-color': '#ccc',
          'target-arrow-color': '#ccc',
          'target-arrow-shape': 'triangle',
          'curve-style': 'bezier'
        }
      }
    ],
    
    layout: {
      name: 'klay'
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