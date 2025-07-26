import cytoscape from 'cytoscape';
import {darkTheme, graphColors} from "../colors";
import {createNodeHtmlLabel, createNodeHtmlLabelId} from "./labels/node_html_label";
import {createEdgeHtmlLabel} from "./labels/edge_html_label";
import {createAggregatedData, createGraphElements} from "./graph_elements";
import {GrpcGraph} from "../protos/ficus/GrpcGraph";
import {GrpcAnnotation} from "../protos/ficus/GrpcAnnotation";
import {AggregatedData, GraphEdge, GraphNode, SoftwareEnhancementKind} from "./types";
import {createLayout} from "./util";
import {GrpcGraphKind} from "../protos/ficus/GrpcGraphKind";
import {nodeHeightPx, nodeWidthPx} from "./constants";

let htmlLabel = require('../html-label/html_label');
htmlLabel(cytoscape);

const graphColor = graphColors(darkTheme);

export default setDrawGraph;

function setDrawGraph() {
  (<any>window).drawGraph = drawGraph;
  (<any>window).createAggregatedData = createAggregatedData;
}

function drawGraph(
  id: string,
  graph: GrpcGraph,
  annotation: GrpcAnnotation,
  aggregatedData: AggregatedData,
  enhancements: SoftwareEnhancementKind[],
  filter: string | null,
  spacingFactor: number,
  isRichUiGraph: boolean,
  useLROrientation: boolean
) {
  try {
    //Dictionary from C# not eventually deserialized to Map in JS)
    aggregatedData.totalHistogramsCount = new Map<string, number>(Object.entries(aggregatedData.totalHistogramsCount));
    aggregatedData.totalCountersCount = new Map<string, number>(Object.entries(aggregatedData.totalCountersCount));

    let regex = filter == null ? null : new RegExp(filter);
    let cy = cytoscape(createCytoscapeOptions(id, graph, annotation, aggregatedData, regex, spacingFactor, isRichUiGraph, useLROrientation));

    if (isRichUiGraph) {
      setNodeEdgeHtmlRenderer(cy, enhancements);
    }

    cy.ready(() => setTimeout(() => updateNodesDimensions(cy, graph.kind, spacingFactor, useLROrientation), 0));

    return cy;
  } catch (e) {
    console.error(e);
    return null;
  }
}

function updateNodesDimensions(cy: cytoscape.Core, kind: GrpcGraphKind, spacingFactor: number, useLROrientation: boolean) {
  cy.nodes().forEach(node => {
    let element = document.getElementById(createNodeHtmlLabelId(node.data().frontendId));
    if (element != null) {
      let rect = element.getBoundingClientRect();
      node.style('width', `${rect.width / cy.zoom()}px`);
      node.style('height', `${rect.height / cy.zoom()}px`);
    }
  });

  cy.layout(createLayout(kind, spacingFactor, useLROrientation)).run();
}

function setNodeEdgeHtmlRenderer(cy: cytoscape.Core, enhancements: SoftwareEnhancementKind[]) {
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
  aggregatedData: AggregatedData,
  filter: RegExp | null,
  spacingFactor: number,
  addLabel: boolean,
  useLROrientation: boolean
): cytoscape.CytoscapeOptions {
  return {
    container: document.getElementById(id),
    elements: createGraphElements(graph, annotation, aggregatedData, filter),
    layout: createLayout(graph.kind, spacingFactor, useLROrientation),
    style: [
      createNodeStyle(addLabel),
      createEdgeStyle(),
    ]
  }
}

function createNodeStyle(isRichUi: boolean): cytoscape.Stylesheet {
  if (isRichUi) {
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

  return {
    selector: 'node',
    style: {
      'label': "data(label)",
      'background-color': graphColor.nodeBackground,
      'text-valign': 'top',
      'text-halign': 'center',
      'shape': 'round-rectangle',
      'width': `40px`,
      'height': `40px`,
      'color': graphColor.labelColor,
    }
  };
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