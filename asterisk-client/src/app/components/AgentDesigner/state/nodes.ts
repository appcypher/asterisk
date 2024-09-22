import { Node, NodeActionType, NodeFactory, NodesAction } from "../types/node";

//--------------------------------------------------------------------------------------------------
// State
//--------------------------------------------------------------------------------------------------

const initialNodes: Node[] = [
  NodeFactory.createTriggerNode("trigger-1", { x: 400, y: 400 }),
];

//--------------------------------------------------------------------------------------------------
// Reducer
//--------------------------------------------------------------------------------------------------

const nodeReducer = (state: Node[], action: NodesAction): Node[] => {
  switch (action.type) {
    case NodeActionType.ADD_NODES:
      return [...state, ...action.payload];
    case NodeActionType.UPDATE_NODES:
      return action.payload;
    case NodeActionType.REMOVE_NODES:
      const idsToRemove = new Set(action.payload.map((node) => node.id));
      return state.filter((node) => !idsToRemove.has(node.id));
    default:
      return state;
  }
};

//--------------------------------------------------------------------------------------------------
// Exports
//--------------------------------------------------------------------------------------------------

export { nodeReducer, initialNodes };
