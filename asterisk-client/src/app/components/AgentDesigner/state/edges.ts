import { Edge, EdgeActionType, EdgesAction } from "../types/edge";

//--------------------------------------------------------------------------------------------------
// State
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

export { edgeReducer, initialEdges };
