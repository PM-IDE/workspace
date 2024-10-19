import cytoscape from 'cytoscape';
import {petriNetColors, lightTheme} from "./colors";
import {createBreadthFirstLayout} from "./utils";
import dagre from 'cytoscape-dagre';

export default setDrawPetriNet;

const placeType = "place";
const transitionType = "transition";
const arcType = "arc";
const netColors = petriNetColors(lightTheme);

function setDrawPetriNet() {
  window.drawPetriNet = function (id, net, annotation) {
    cytoscape.use(dagre);
    cytoscape(createCytoscapeOptions(id, net, annotation));
  }
}

function createCytoscapeOptions(id, net, annotation) {
  return {
    container: document.getElementById(id),
    elements: createElementsFromNet(net, annotation),
    style: createStylesList(),
    layout: createBreadthFirstLayout()
  }
}

function createElementsFromNet(net, annotation) {
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

function createArcElement(arcId, id, source, target, maxAnnotation, annotation) {
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
    data.annotation = annotation[arcId];

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

function findMaxAnnotation(annotation) {
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

function createStylesList() {
  return [
    createCommonNodeStyle(),
    createTransitionNodeStyle(),
    createEdgeStyle()
  ];
}

function createCommonNodeStyle() {
  return {
    selector: 'node',
    style: {
      'background-opacity': '0',
      'border-width': '1px',
      'border-style': 'solid',
      'border-color': netColors.borderLine
    }
  };
}

function createTransitionNodeStyle() {
  return {
    selector: `node[type="${transitionType}"]`,
    style: {
      'shape': 'rectangle',
      'label': 'data(name)',
      'background-opacity': '1',
      'background-color': netColors.transitionBackground
    },
  };
}

function createEdgeStyle() {
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