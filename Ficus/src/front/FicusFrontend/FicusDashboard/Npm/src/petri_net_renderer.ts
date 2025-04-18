import cytoscape, {Stylesheet} from 'cytoscape';
import {petriNetColors, darkTheme} from "./colors";
import dagre from 'cytoscape-dagre';
import {createDagreLayout} from "./graph/util";
import {GrpcPetriNet} from "./protos/ficus/GrpcPetriNet";

export default setDrawPetriNet;

const placeType = "place";
const transitionType = "transition";
const arcType = "arc";
const netColors = petriNetColors(darkTheme);

function setDrawPetriNet() {
  (<any>window).drawPetriNet = function (id: string, net: GrpcPetriNet, annotation: Record<number, number>) {
    cytoscape.use(dagre);
    return cytoscape(createCytoscapeOptions(id, net, annotation));
  }
}

function createCytoscapeOptions(id: string, net: GrpcPetriNet, annotation: Record<number, number>): cytoscape.CytoscapeOptions {
  return {
    container: document.getElementById(id),
    elements: createElementsFromNet(net, annotation),
    style: createStylesList(),
    layout: createDagreLayout()
  }
}

function createElementsFromNet(net: GrpcPetriNet, annotation: Record<number, number>): cytoscape.ElementDefinition[] {
  let maxAnnotation = findMaxAnnotation(annotation);
  const elements = [];

  for (const place of net.places) {
    elements.push({
      data: {
        type: placeType,
        id: place.id.toString()
      }
    });
  }

  for (const transition of net.transitions) {
    elements.push({
      data: {
        type: transitionType,
        id: transition.id.toString(),
        name: transition.data
      }
    });
  }

  for (const transition of net.transitions) {
    for (const arc of transition.incomingArcs) {
      elements.push(createArcElement(
        arc.id,
        arc.placeId.toString() + "::" + transition.id.toString(),
        arc.placeId.toString(),
        transition.id.toString(),
        maxAnnotation,
        annotation
      ));
    }

    for (const arc of transition.outgoingArcs) {
      elements.push(createArcElement(
        arc.id,
        transition.id.toString() + "::" + arc.placeId.toString(),
        transition.id.toString(),
        arc.placeId.toString(),
        maxAnnotation,
        annotation
      ));
    }
  }

  return elements;
}

function createArcElement(arcId: number, id: string, source: string, target: string, maxAnnotation: number, annotation: Record<number, number>) {
  let data = {
    type: arcType,
    id: id,
    source: source,
    target: target,
    width: 1,
  }

  const minWidth = 3;
  const maxWidth = 6;

  if (maxAnnotation != null) {
    (<any>data).annotation = annotation[arcId];

    if (maxAnnotation === 0) {
      data.width = 1;
    } else {
      data.width = minWidth + (annotation[arcId] / maxAnnotation) * (maxWidth - minWidth);
    }
  }

  return {
    data: data
  };
}

function findMaxAnnotation(annotation: Record<number, number>) {
  if (annotation == null) {
    return null;
  }

  let maxValue = 0;
  for (let key in annotation) {
    let value = annotation[key];
    maxValue = Math.max(value, maxValue);
  }

  return maxValue;
}

function createStylesList(): Stylesheet[] {
  return [
    createCommonNodeStyle(),
    createTransitionNodeStyle(),
    createEdgeStyle()
  ];
}

function createCommonNodeStyle(): Stylesheet {
  return {
    selector: 'node',
    style: {
      'border-width': '1px',
      'border-style': 'solid',
      'border-color': netColors.borderLine,
      'background-color': netColors.placeBackground,
    }
  };
}

function createTransitionNodeStyle(): Stylesheet {
  return {
    selector: `node[type="${transitionType}"]`,
    style: {
      'shape': 'rectangle',
      'label': 'data(name)',
      'background-opacity': 1,
      'background-color': netColors.transitionBackground,
      'color': netColors.labelColor
    },
  };
}

function createEdgeStyle(): Stylesheet {
  return {
    selector: 'edge',
    style: {
      'label': 'data(annotation)',
      'width': 'data(width)',
      'line-color': netColors.arcLine,
      'target-arrow-color': netColors.arcLine,
      'target-arrow-shape': 'triangle',
      'curve-style': 'bezier'
    }
  };
}