export function setOcelSankeyFunctions() {
  (<any>window).createOcelSankeyConnections = createOcelSankeyConnections;
}

interface ObjectsRelation {
  id: string
  relatedObjectsIds: string[]
}

function createOcelSankeyConnections(parentContainerId: string, baseIdPart: string, relations: ObjectsRelation[]) {
  console.log(parentContainerId, baseIdPart, relations);
}
