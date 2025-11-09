export function setOcelSankeyFunctions() {
  (<any>window).createOcelSankeyConnections = createOcelSankeyConnections;
}

interface ObjectsRelation {
  id: string
  currentNodeId: number
  fromNodeId: number
  relatedObjectsIds: string[]
}

function createOcelSankeyConnections(parentContainerId: string, baseIdPart: string, relations: ObjectsRelation[]) {
  let getInitialClass = (isInitialState: boolean) => isInitialState ? "initial" : "final";
  let createId = (nodeId: number, objectId: string, isInitialState: boolean) => {
    const Delimiter = "-";
    return baseIdPart + Delimiter + nodeId + Delimiter + getInitialClass(isInitialState) + Delimiter + objectId;
  };

  let getElement = (nodeId: number, objectId: string, isInitialState: boolean) => {
    let id = createId(nodeId, objectId, isInitialState);
    let element = document.getElementById(id);

    if (element == null) {
      console.warn(`Failed to get element with id ${id}`);
    }

    return element;
  };

  let parentContainer = document.getElementById(parentContainerId);
  if (parentContainer == null) return;

  for (let relation of relations) {
    let firstElement = getElement(relation.currentNodeId, relation.id, true);
    if (firstElement == null) continue;

    for (let relatedId of relation.relatedObjectsIds) {
      let secondElement = getElement(relation.fromNodeId, relatedId, false);
      if (secondElement == null) continue;

      connect(parentContainer, firstElement, secondElement, "red", 5);
    }
  }
}

function getOffset(el: HTMLElement) {
  return {
    left: el.offsetLeft,
    top: el.offsetTop,
    width: el.offsetWidth,
    height: el.offsetHeight,
  };
}

function connect(parentContainer: HTMLElement, first: HTMLElement, second: HTMLElement, color: string, thickness: number) {
  let firstOffset = getOffset(first);
  let secondOffset = getOffset(second);

  let firstX = firstOffset.left + firstOffset.width;
  let firstY = firstOffset.top + firstOffset.height;

  let secondX = secondOffset.left + secondOffset.width;
  let secondY = secondOffset.top;

  let length = Math.sqrt(((secondX - firstX) * (secondX - firstX)) + ((secondY - firstY) * (secondY - firstY)));

  let cx = ((firstX + secondX) / 2) - (length / 2);
  let cy = ((firstY + secondY) / 2) - (thickness / 2);

  let angle = Math.atan2((firstY - secondY), (firstX - secondX)) * (180 / Math.PI);

  let html = `
    <div style="padding: 0;
                margin: 0;
                height: ${thickness}px;
                background-color: ${color};
                line-height: 1px;
                position: absolute;
                left: ${cx}px;
                top: ${cy}px;
                width: ${length}px;
                -ms-transform:rotate(${angle}deg);
                transform:rotate(${angle}deg);">
    </div>
  `;

  parentContainer.innerHTML += html;
}