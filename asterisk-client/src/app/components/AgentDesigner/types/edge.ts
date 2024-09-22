import { Edge as RFEdge } from "@xyflow/react";

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

type Edge = RFEdge<EdgeData>;

type EdgeData = {
  label: string;
};

type EdgesAction = {
  type: EdgeActionType;
  payload: Edge[];
};

enum EdgeActionType {
  ADD_EDGES = "ADD_EDGES",
  REMOVE_EDGES = "REMOVE_EDGES",
  UPDATE_EDGES = "UPDATE_EDGES",
}

//--------------------------------------------------------------------------------------------------
// Exports
//--------------------------------------------------------------------------------------------------

export { EdgeActionType };
export type { Edge, EdgeData, EdgesAction };
