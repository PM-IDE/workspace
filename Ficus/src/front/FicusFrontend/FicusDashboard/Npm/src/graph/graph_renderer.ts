import cytoscape from 'cytoscape';
import {darkTheme, graphColors} from "../colors";
import dagre from 'cytoscape-dagre';
import {createEdgeHtmlLabel, createNodeHtmlLabel} from "./html_label";
import {createGraphElementForDagre} from "./other_layouts";
import {createDagreLayout} from "./util";
import {nodeHeightPx, nodeWidthPx} from "./constants";
import {GrpcGraph} from "../protos/ficus/GrpcGraph";
import {GrpcAnnotation} from "../protos/ficus/GrpcAnnotation";
import {GraphNode, SoftwareEnhancementKind} from "./types";

export default setDrawGraph;

const graphColor = graphColors(darkTheme);

function setDrawGraph() {
  (<any>window).drawGraph = function (id: string, graph: GrpcGraph, annotation: GrpcAnnotation, enhancement: keyof typeof SoftwareEnhancementKind) {
    cytoscape.use(dagre);

    let htmlLabel = require('../html-label/html_label');
    htmlLabel(cytoscape);

    let cy = cytoscape(createCytoscapeOptions(id, graph, annotation));
    setNodeRenderer(cy, SoftwareEnhancementKind[enhancement]);

    return cy;
  }
}

function setNodeRenderer(cy: cytoscape.Core, enhancement: SoftwareEnhancementKind) {
  (<any>cy).htmlLabel(
    [
      {
        query: 'node',
        tpl: function (data: GraphNode) {
          return createNodeHtmlLabel(data, enhancement);
        }
      },
      {
        query: 'edge',
        ealign: 'midpoint',
        autorotate: true,
        valignBox: 'center',

        tpl: function (data: GraphNode) {
          return createEdgeHtmlLabel(data, enhancement);
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
      'color': graphColor.labelColor,
      'events': 'no'
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
      'events': 'no'
    }
  }
}