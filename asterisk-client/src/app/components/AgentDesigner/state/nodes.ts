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
  SYNC_NODES = "SYNC_NODES",
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
      // Unset any existing selected node
      const updatedNodes = state.map((node) => ({
        ...node,
        selected: false,
      }));

      return [...updatedNodes, ...action.payload];
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

    case NodeActionType.SYNC_NODES:
      const updates = action.payload.reduce(
        (acc, node) => {
          acc[node.id] = node;
          return acc;
        },
        {} as Record<string, Node>,
      );

      // Filter out nodes not present in updates and merge existing nodes with updates
      return state
        .filter((node) => updates[node.id]) // Remove nodes not present in updates
        .map((node) => ({ ...node, ...updates[node.id] })) // Merge updates
        .concat(
          // Add new nodes that weren't in the original state
          action.payload.filter(
            (node) =>
              !state.some((existingNode) => existingNode.id === node.id),
          ),
        );
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
