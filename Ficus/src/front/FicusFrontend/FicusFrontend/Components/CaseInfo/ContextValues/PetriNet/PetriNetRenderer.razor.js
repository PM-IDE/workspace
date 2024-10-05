window.drawPetriNet = function (id, net) {
  let element = document.getElementById(id);
  let elements = [];

  const placeType = "place";
  const transitionType = "transition";
  const arcType = "arc";
  
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
  
  let nodeStyle = {
    selector: 'node',
    style: {
      'background-color': '#666'
    }
  };

  let transitionNodeStyle = {
    selector: `node[type="${transitionType}"]`,
    style: {
      'label': 'data(name)'
    }
  }
  
  let edgeStyle = {
    selector: 'edge',
    style: {
      'width': 3,
      'line-color': '#ccc',
      'target-arrow-color': '#ccc',
      'target-arrow-shape': 'triangle',
      'curve-style': 'bezier'
    }
  };

  let cy = cytoscape({
    container: element,
    elements: elements,
    style: [
      nodeStyle,
      transitionNodeStyle,
      edgeStyle
    ],

    layout: {
      name: 'grid',
      rows: 1
    }
  });
}