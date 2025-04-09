// import {belongsToRootSequence} from "./util";
// import {createGraphEdgesElements} from "./other_layouts";
// import {nodeHeightPx, nodeWidthPx, nodeXDelta, nodeYDelta} from "./constants";
// import {GrpcGraphNode} from "../protos/ficus/GrpcGraphNode";
// import {GrpcGraph} from "../protos/ficus/GrpcGraph";
// import {GrpcAnnotation} from "../protos/ficus/GrpcAnnotation";
//
// export function createGraphElementsForPresetLayout(graph: GrpcGraph, annotation: GrpcAnnotation) {
//   let elements = [];
//
//   let nonRootSequenceNodes = graph.nodes.filter(n => !belongsToRootSequence(n));
//   let nonRootNodesByTraces = new Map();
//
//   for (let node of nonRootSequenceNodes) {
//     let traceId = getTraceDataOrNull(node).traceId;
//     if (nonRootNodesByTraces.has(traceId)) {
//       nonRootNodesByTraces.get(traceId).push(node);
//     } else {
//       nonRootNodesByTraces.set(traceId, [node]);
//     }
//   }
//
//   let rootNodesY = Math.ceil(nonRootNodesByTraces.size / 2) * (nodeHeightPx + nodeYDelta);
//   let rootSequenceNodes = graph.nodes.filter(n => belongsToRootSequence(n));
//
//   for (let [index, node] of sortNodesByEventIndex(rootSequenceNodes).entries()) {
//     elements.push({
//       renderedPosition: {
//         x: index * (nodeWidthPx + nodeXDelta),
//         y: rootNodesY,
//       },
//       data: createNodeData(node)
//     });
//   }
//
//   let traceIndex = 0;
//
//   for (let [traceId, nodes] of nonRootNodesByTraces) {
//     for (let [index, node] of sortNodesByEventIndex(nodes).entries()) {
//       elements.push({
//         renderedPosition: {
//           x: index * (nodeWidthPx + nodeXDelta),
//           y: (traceIndex > (nonRootNodesByTraces.size / 2) ? traceIndex + 1 : traceIndex) * (nodeHeightPx + nodeYDelta)
//         },
//         data: createNodeData(node)
//       });
//     }
//
//     traceIndex += 1;
//   }
//
//   elements.push(...createGraphEdgesElements(graph.edges, annotation));
//
//   return elements;
// }
//
// function sortNodesByEventIndex(nodes: GrpcGraphNode[]) {
//   return nodes.toSorted((f, s) => {
//     return getTraceDataOrNull(f).eventIndex - getTraceDataOrNull(s).eventIndex;
//   });
// }
//
// function createNodeData(node: GrpcGraphNode) {
//   return {
//     label: node.data,
//     id: node.id.toString(),
//     additionalData: node.additionalData
//   };
// }
