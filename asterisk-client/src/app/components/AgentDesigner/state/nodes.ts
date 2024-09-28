import { Node as RFNode } from "@xyflow/react";

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

type Node = RFNode<NodeData>;

type NodeData = {
  label?: string;
  pinned?: boolean;
};

enum NodeType {
  TRIGGER = "TRIGGER",
  ACTION = "ACTION",
  TERMINAL = "TERMINAL",
  NOTE = "NOTE",
}

type NodesAction = {
  type: NodeActionType;
  payload: Node[];
};

enum NodeActionType {
  ADD_NODES = "ADD_NODES",
  REMOVE_NODES = "REMOVE_NODES",
  UPDATE_NODES = "UPDATE_NODES",
}

//--------------------------------------------------------------------------------------------------
// Constants
//--------------------------------------------------------------------------------------------------

const initialNodes: Node[] = [];

//--------------------------------------------------------------------------------------------------
// Reducer
//--------------------------------------------------------------------------------------------------

const nodeReducer = (state: Node[], action: NodesAction): Node[] => {
  switch (action.type) {
    case NodeActionType.ADD_NODES:
      return [...state, ...action.payload];
    case NodeActionType.UPDATE_NODES:
      // Create a map for quick lookup of updated nodes by their ID
      const updatedNodesMap = new Map(
        action.payload.map((node) => [node.id, node]),
      );

      // Iterate over the current state and update nodes as needed
      return state.map((node) => {
        const updatedNode = updatedNodesMap.get(node.id);
        return updatedNode ? { ...node, ...updatedNode } : node;
      });
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

export { nodeReducer, initialNodes, NodeActionType, NodeType };
export type { Node, NodeData, NodesAction };
