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
  SYNC_EDGES = "SYNC_EDGES",
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
      // Create a map for quick lookup of updated edges by their ID
      const updatedEdgesMap = new Map(
        action.payload.map((edge) => [edge.id, edge]),
      );

      // Iterate over the current state and update edges as needed
      return state.map((edge) => {
        const updatedEdge = updatedEdgesMap.get(edge.id);
        return updatedEdge ? { ...edge, ...updatedEdge } : edge;
      });
    case EdgeActionType.SYNC_EDGES:
      const updates = action.payload.reduce(
        (acc, edge) => {
          acc[edge.id] = edge;
          return acc;
        },
        {} as Record<string, Edge>,
      );

      // Filter out edges not present in updates and merge existing edges with updates
      return state
        .filter((edge) => updates[edge.id]) // Remove edges not present in updates
        .map((edge) => ({ ...edge, ...updates[edge.id] })) // Merge updates
        .concat(
          // Add new edges that weren't in the original state
          action.payload.filter(
            (edge) =>
              !state.some((existingEdge) => existingEdge.id === edge.id),
          ),
        );
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
