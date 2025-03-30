import cytoscape from 'cytoscape';
import {darkTheme, graphColors} from "../colors";
import dagre from 'cytoscape-dagre';
import nodeHtmlLabel from 'cytoscape-node-html-label'
import {createHtmlLabel} from "./html_label";
import {createGraphElementForDagre} from "./other_layouts";
import {createDagreLayout} from "./util";
import {nodeHeightPx, nodeWidthPx} from "./constants";

export default setDrawGraph;

const graphColor = graphColors(darkTheme);

function setDrawGraph() {
  window.drawGraph = function (id, graph, annotation) {
    cytoscape.use(dagre);
    nodeHtmlLabel(cytoscape);

    let cy = cytoscape(createCytoscapeOptions(id, graph, annotation));
    setNodeRenderer(cy);

    return cy;
  }
}

function setNodeRenderer(cy) {
  cy.nodeHtmlLabel(
    [
      {
        query: 'node',
        tpl: function (data) {
          return createHtmlLabel(data);
        }
      }
    ],
    {
      enablePointerEvents: true
    }
  );
}

function createCytoscapeOptions(id, graph, annotation) {
  return {
    container: document.getElementById(id),
    elements: createGraphElementForDagre(graph, annotation),
    layout: createDagreLayout(),
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
      'text-valign': 'center',
      'text-halign': 'right',
      'shape': 'round-rectangle',
      'width': `${nodeWidthPx}px`,
      'height': `${nodeHeightPx}px`,
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