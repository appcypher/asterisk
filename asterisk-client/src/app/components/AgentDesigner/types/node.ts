import { Node as RFNode } from "@xyflow/react";

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

type Node = RFNode<NodeData>;

type NodeData = {
  label: string;
};

enum NodeType {
  EMPTY = "EMPTY",
  TRIGGER = "TRIGGER",
  ACTION = "ACTION",
  TERMINAL = "TERMINAL",
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
// Exports
//--------------------------------------------------------------------------------------------------

export { NodeType, NodeActionType };
export type { Node, NodeData, NodesAction };
