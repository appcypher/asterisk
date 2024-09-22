import { Node as RFNode, XYPosition } from "@xyflow/react";

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
// Methods
//--------------------------------------------------------------------------------------------------

const NodeFactory = {
  createTriggerNode: (id: string, position: XYPosition): Node => ({
    id,
    type: NodeType.TRIGGER,
    position,
    data: { label: "Triggering" },
  }),
};

//--------------------------------------------------------------------------------------------------
// Exports
//--------------------------------------------------------------------------------------------------

export { NodeType, NodeActionType, NodeFactory };
export type { Node, NodeData, NodesAction };
