import cytoscape from 'cytoscape';
import {darkTheme, graphColors} from "../colors";
import {createNodeHtmlLabel, createNodeHtmlLabelId} from "./labels/node_html_label";
import {createEdgeHtmlLabel} from "./labels/edge_html_label";
import {createGraphElements} from "./graph_elements";
import {nodeHeightPx, nodeWidthPx} from "./constants";
import {GrpcGraph} from "../protos/ficus/GrpcGraph";
import {GrpcAnnotation} from "../protos/ficus/GrpcAnnotation";
import {GraphEdge, GraphNode, SoftwareEnhancementKind} from "./types";
import {createLayout} from "./util";
import {GrpcGraphKind} from "../protos/ficus/GrpcGraphKind";

let htmlLabel = require('../html-label/html_label');
htmlLabel(cytoscape);

const graphColor = graphColors(darkTheme);

export default setDrawGraph;
function setDrawGraph() {
  (<any>window).drawGraph = drawGraph;
}

function drawGraph(
  id: string,
  graph: GrpcGraph,
  annotation: GrpcAnnotation,
  enhancements: (keyof typeof SoftwareEnhancementKind)[],
  filter: string | null,
  spacingFactor: number
) {
  let regex = filter == null ? null : new RegExp(filter);
  let cy = cytoscape(createCytoscapeOptions(id, graph, annotation, regex, spacingFactor));
  setNodeRenderer(cy, enhancements.map(e => SoftwareEnhancementKind[e]));

  cy.ready(() => setTimeout(() => updateNodesDimensions(cy, graph.kind, spacingFactor), 0));

  return cy;
}

function updateNodesDimensions(cy: cytoscape.Core, kind: GrpcGraphKind, spacingFactor: number) {
  cy.nodes().forEach(node => {
    let element = document.getElementById(createNodeHtmlLabelId(node.data().frontendId));
    if (element != null) {
      let rect = element.getBoundingClientRect();
      node.style('width', `${rect.width / cy.zoom()}px`);
      node.style('height', `${rect.height / cy.zoom()}px`);
    }
  });

  cy.layout(createLayout(kind, spacingFactor)).run();
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

function createCytoscapeOptions(
  id: string,
  graph: GrpcGraph,
  annotation: GrpcAnnotation,
  filter: RegExp | null,
  spacingFactor: number
): cytoscape.CytoscapeOptions {
  return {
    container: document.getElementById(id),
    elements: createGraphElements(graph, annotation, filter),
    layout: createLayout(graph.kind, spacingFactor),
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