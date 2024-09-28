import { createContext, Dispatch, Reducer, useReducer } from "react";
import { initialNodes, nodeReducer, NodesAction, Node } from "./state/nodes";
import { Edge, edgeReducer, EdgesAction, initialEdges } from "./state/edges";

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

type State = {
  nodes: Node[];
  edges: Edge[];
  nodesDispatch: Dispatch<NodesAction>;
  edgesDispatch: Dispatch<EdgesAction>;
};

//--------------------------------------------------------------------------------------------------
// Context
//--------------------------------------------------------------------------------------------------

const CanvasContext = createContext<State>({
  nodes: [],
  edges: [],
  nodesDispatch: () => {},
  edgesDispatch: () => {},
});

//--------------------------------------------------------------------------------------------------
// Component
//--------------------------------------------------------------------------------------------------

const CanvasContextProvider = ({ children }: { children: React.ReactNode }) => {
  const [nodes, nodesDispatch] = useReducer<Reducer<Node[], NodesAction>>(
    nodeReducer,
    initialNodes,
  );

  const [edges, edgesDispatch] = useReducer<Reducer<Edge[], EdgesAction>>(
    edgeReducer,
    initialEdges,
  );

  return (
    <CanvasContext.Provider
      value={{
        nodes,
        edges,
        nodesDispatch,
        edgesDispatch,
      }}
    >
      {children}
    </CanvasContext.Provider>
  );
};

//--------------------------------------------------------------------------------------------------
// Exports
//--------------------------------------------------------------------------------------------------

export { CanvasContext, CanvasContextProvider };
