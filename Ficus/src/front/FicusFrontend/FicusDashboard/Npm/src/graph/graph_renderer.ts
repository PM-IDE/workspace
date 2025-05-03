import cytoscape from 'cytoscape';
import {darkTheme, graphColors} from "../colors";
import dagre from 'cytoscape-dagre';
import {createNodeHtmlLabel, createNodeHtmlLabelId} from "./labels/node_html_label";
import {createEdgeHtmlLabel} from "./labels/edge_html_label";
import {createGraphElementForDagre} from "./other_layouts";
import {createDagreLayout} from "./util";
import {nodeHeightPx, nodeWidthPx} from "./constants";
import {GrpcGraph} from "../protos/ficus/GrpcGraph";
import {GrpcAnnotation} from "../protos/ficus/GrpcAnnotation";
import {GraphEdge, GraphNode, SoftwareEnhancementKind} from "./types";

export default setDrawGraph;
cytoscape.use(dagre);

let htmlLabel = require('../html-label/html_label');
htmlLabel(cytoscape);

const graphColor = graphColors(darkTheme);

function setDrawGraph() {
  (<any>window).drawGraph = drawGraph;
}

function drawGraph(
  id: string, 
  graph: GrpcGraph, 
  annotation: GrpcAnnotation, 
  enhancements: (keyof typeof SoftwareEnhancementKind)[],
  filter: string | null
) {
  let regex = filter == null ? null : new RegExp(filter);
  let cy = cytoscape(createCytoscapeOptions(id, graph, annotation, regex));
  setNodeRenderer(cy, enhancements.map(e => SoftwareEnhancementKind[e]));

  cy.ready(() => setTimeout(() => updateNodesDimensions(cy), 0));

  return cy;
}

function updateNodesDimensions(cy: cytoscape.Core) {
  cy.nodes().forEach(node => {
    let element = document.getElementById(createNodeHtmlLabelId(node.data().frontendId));
    if (element != null) {
      let rect = element.getBoundingClientRect();
      node.style('width', `${rect.width / cy.zoom()}px`);
      node.style('height', `${rect.height / cy.zoom()}px`);
    }
  });

  cy.layout(createDagreLayout()).run();
}

function setNodeRenderer(cy: cytoscape.Core, enhancements: SoftwareEnhancementKind[]) {
  (<any>cy).htmlLabel(
    [
      {
        query: 'node',
        tpl: function (data: GraphNode) {
          return createNodeHtmlLabel(data, enhancements);
        }
      },
      {
        query: 'edge',
        ealign: 'midpoint',
        autorotate: true,
        valignBox: 'center',

        tpl: function (data: GraphEdge) {
          return createEdgeHtmlLabel(data, enhancements);
        }
      }
    ],
    {
      enablePointerEvents: true
    }
  );
}

function createCytoscapeOptions(id: string, graph: GrpcGraph, annotation: GrpcAnnotation, filter: RegExp | null): cytoscape.CytoscapeOptions {
  return {
    container: document.getElementById(id),
    elements: createGraphElementForDagre(graph, annotation, filter),
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