import cytoscape from 'cytoscape';
import {darkTheme, graphColors} from "../colors";
import dagre from 'cytoscape-dagre';
import {createHtmlLabel} from "./html_label";
import {createGraphElementForDagre} from "./other_layouts";
import {createDagreLayout} from "./util";
import {nodeHeightPx, nodeWidthPx} from "./constants";
import {GrpcGraph} from "../protos/ficus/GrpcGraph";
import {GrpcAnnotation} from "../protos/ficus/GrpcAnnotation";
import {GraphNode} from "./types";

export default setDrawGraph;

const graphColor = graphColors(darkTheme);

function setDrawGraph() {
  (<any>window).drawGraph = function (id: string, graph: GrpcGraph, annotation: GrpcAnnotation) {
    cytoscape.use(dagre);

    let nodeHtmlLabel = require('cytoscape-node-html-label');
    nodeHtmlLabel(cytoscape);

    let cy = cytoscape(createCytoscapeOptions(id, graph, annotation));
    setNodeRenderer(cy);

    return cy;
  }
}

function setNodeRenderer(cy: cytoscape.Core) {
  (<any>cy).nodeHtmlLabel(
    [
      {
        query: 'node',
        tpl: function (data: GraphNode) {
          return createHtmlLabel(data);
        }
      }
    ],
    {
      enablePointerEvents: true
    }
  );
}

function createCytoscapeOptions(id: string, graph: GrpcGraph, annotation: GrpcAnnotation): cytoscape.CytoscapeOptions {
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

function createNodeStyle(): cytoscape.Stylesheet {
  return {
    selector: 'node',
    style: {
      "background-color": 'transparent',
      "background-opacity": 0,
      'text-valign': 'center',
      'text-halign': 'right',
      'shape': 'round-rectangle',
      'width': `${nodeWidthPx}px`,
      'height': `${nodeHeightPx}px`,
      'color': graphColor.labelColor
    }
  }
}

function createEdgeStyle(): cytoscape.Stylesheet {
  return {
    selector: 'edge',
    style: {
      'label': "data(label)",
      'color': graphColor.labelColor,
      'width': "data(width)",
      'line-color': 'data(color)',
      'target-arrow-color': 'data(color)',
      'target-arrow-shape': 'triangle',
      'curve-style': 'straight',
    }
  }
}