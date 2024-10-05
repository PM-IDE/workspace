import cytoscape from 'cytoscape';
import klay from 'cytoscape-klay';

export default set_draw_petri_net;

const placeType = "place";
const transitionType = "transition";
const arcType = "arc";

function set_draw_petri_net() {
  window.drawPetriNet = function (id, net) {
    cytoscape.use(klay);
    cytoscape(createCytoscapeOptions(id, net));
  } 
}

function createCytoscapeOptions(id, net) {
  return {
    container: document.getElementById(id),
    elements: createElementsFromNet(net),
    style: createStylesList(),

    layout: {
      name: 'klay',
    }
  }
}

function createElementsFromNet(net) {
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
      elements.push({
        data: {
          type: arcType,
          id: arc.placeId.toString() + "::" + transition.id.toString(),
          source: arc.placeId.toString(),
          target: transition.id.toString(),
        }
      });
    }

    for (const arc of transition.outgoingArcs) {
      elements.push({
        data: {
          type: arcType,
          id: transition.id.toString() + "::" + arc.placeId.toString(),
          target: arc.placeId.toString(),
          source: transition.id.toString(),
        }
      })
    }
  }
  
  return elements;
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
      'background-color': '#666'
    }
  };
}

function createTransitionNodeStyle() {
  return {
    selector: `node[type="${transitionType}"]`,
    style: {
      'shape': 'rectangle',
      'label': 'data(name)'
    }
  };
}

function createEdgeStyle() {
  return {
    selector: 'edge',
    style: {
      'width': 3,
      'line-color': '#ccc',
      'target-arrow-color': '#ccc',
      'target-arrow-shape': 'triangle',
      'curve-style': 'bezier'
    }
  };
}