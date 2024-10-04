window.drawPetriNet = function (id, net) {
  let element = document.getElementById(id);
  let elements = [];

  for (const place of net.places) {
    elements.push({
      data: {
        id: place.id.toString()
      }
    });
  }
  
  for (const transition of net.transitions) {
    elements.push({
      data: {
        id: transition.id.toString()
      }
    });
  }
  
  for (const transition of net.transitions) {
    for (const arc of transition.incomingArcs) {
      elements.push({
        data: {
          id: arc.placeId.toString() + "::" + transition.id.toString(),
          source: arc.placeId.toString(),
          target: transition.id.toString(),
        }
      });
    }
    
    for (const arc of transition.outgoingArcs) {
      elements.push({
        data: {
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
      'background-color': '#666',
      'label': 'data(id)'
    }
  };
  
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
      edgeStyle
    ],

    layout: {
      name: 'grid',
      rows: 1
    }
  });
}