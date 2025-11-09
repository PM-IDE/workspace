export function setOcelSankeyFunctions() {
  (<any>window).createOcelSankeyConnections = createOcelSankeyConnections;
}

interface ObjectsRelation {
  id: string
  currentNodeId: number
  fromNodeId: number
  relatedObjectsIds: string[]
}

interface Position {
  left: number
  top: number
  width: number
  height: number
}

function createOcelSankeyConnections(parentContainerId: string, baseIdPart: string, relations: ObjectsRelation[]) {
  let getInitialClass = (isInitialState: boolean) => isInitialState ? "initial" : "final";
  let createId = (nodeId: number, objectId: string, isInitialState: boolean) => {
    const Delimiter = "-";
    return baseIdPart + Delimiter + nodeId + Delimiter + getInitialClass(isInitialState) + Delimiter + objectId;
  };

  let offsetsMap = new Map<string, Position>();

  let getElementPosition = (nodeId: number, objectId: string, isInitialState: boolean): Position | null => {
    let id = createId(nodeId, objectId, isInitialState);
    if (!offsetsMap.has(id)) {
      let element = document.getElementById(id);

      if (element == null) {
        console.warn(`Failed to get element with id ${id}`);
        return null;
      }

      offsetsMap.set(id, getPosition(element));
    }

    return offsetsMap.get(id);
  };

  let parentContainer = document.getElementById(parentContainerId);
  if (parentContainer == null) return;

  for (let relation of relations) {
    let firstPos = getElementPosition(relation.currentNodeId, relation.id, true);
    if (firstPos == null) continue;

    for (let relatedId of relation.relatedObjectsIds) {
      let secondPos = getElementPosition(relation.fromNodeId, relatedId, false);
      if (secondPos == null) continue;

      connect(parentContainer, firstPos, secondPos, "red", 5);
    }
  }
}

function getPosition(el: HTMLElement): Position {
  return {
    left: el.offsetLeft,
    top: el.offsetTop,
    width: el.offsetWidth,
    height: el.offsetHeight,
  };
}

function connect(parentContainer: HTMLElement, firstPos: Position, secondPos: Position, color: string, thickness: number) {
  let firstX = firstPos.left + firstPos.width;
  let firstY = firstPos.top + firstPos.height;

  let secondX = secondPos.left + secondPos .width;
  let secondY = secondPos.top;

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