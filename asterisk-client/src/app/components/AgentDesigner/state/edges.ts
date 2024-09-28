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
// Constants
//--------------------------------------------------------------------------------------------------

const initialEdges: Edge[] = [];

//--------------------------------------------------------------------------------------------------
// Reducer
//--------------------------------------------------------------------------------------------------

const edgeReducer = (state: Edge[], action: EdgesAction): Edge[] => {
  switch (action.type) {
    case EdgeActionType.ADD_EDGES:
      return [...state, ...action.payload];
    case EdgeActionType.UPDATE_EDGES:
      return action.payload;
    case EdgeActionType.REMOVE_EDGES:
      const idsToRemove = new Set(action.payload.map((edge) => edge.id));
      return state.filter((edge) => !idsToRemove.has(edge.id));
    default:
      return state;
  }
};

//--------------------------------------------------------------------------------------------------
// Exports
//--------------------------------------------------------------------------------------------------

export { edgeReducer, initialEdges, EdgeActionType };
export type { Edge, EdgeData, EdgesAction };
