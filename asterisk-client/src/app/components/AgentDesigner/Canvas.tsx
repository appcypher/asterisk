import {
  addEdge,
  applyEdgeChanges,
  applyNodeChanges,
  Background,
  Connection,
  EdgeChange,
  NodeChange,
  NodeTypes,
  ReactFlow,
  SelectionMode,
  useViewport,
  Viewport,
} from "@xyflow/react";
import "@xyflow/react/dist/style.css";
import { Dispatch, Reducer, useCallback, useReducer, useState } from "react";
import { initialNodes, nodeReducer } from "./state/nodes";
import BackgroundContextMenu from "./BackgroundContextMenu";
import { NodesAction, Node, NodeActionType, NodeType } from "./types/node";
import { Edge, EdgeActionType, EdgesAction } from "./types/edge";
import { edgeReducer, initialEdges } from "./state/edges";
import { TriggerNode, ActionNode } from "./Node";

//--------------------------------------------------------------------------------------------------
// State
//--------------------------------------------------------------------------------------------------

const nodeTypes: NodeTypes = {
  TRIGGER: TriggerNode,
  ACTION: ActionNode,
};

//--------------------------------------------------------------------------------------------------
// Hooks
//--------------------------------------------------------------------------------------------------

const useCanvas = (
  nodes: Node[],
  edges: Edge[],
  nodesDispatch: Dispatch<NodesAction>,
  edgesDispatch: Dispatch<EdgesAction>,
) => {
  const onNodesChange = useCallback(
    (changes: NodeChange<Node>[]) => {
      const n = applyNodeChanges(changes, nodes);
      nodesDispatch({ type: NodeActionType.UPDATE_NODES, payload: n });
    },
    [nodesDispatch, nodes],
  );

  const onEdgesChange = useCallback(
    (changes: EdgeChange<Edge>[]) => {
      const e = applyEdgeChanges(changes, edges);
      edgesDispatch({ type: EdgeActionType.UPDATE_EDGES, payload: e });
    },
    [edgesDispatch, edges],
  );

  const onConnect = useCallback(
    (conn: Connection) => {
      const edge: Edge = {
        ...conn,
        id: `${conn.source}-${conn.target}`,
      };

      const e = addEdge(edge, edges);
      edgesDispatch({ type: EdgeActionType.UPDATE_EDGES, payload: e });
    },
    [edgesDispatch, edges],
  );

  return {
    onNodesChange,
    onEdgesChange,
    onConnect,
  };
};

//--------------------------------------------------------------------------------------------------
// Handlers
//--------------------------------------------------------------------------------------------------

const onAddTriggerNode =
  (
    nodesDispatch: Dispatch<NodesAction>,
    setContextMenuMouseEvent: Dispatch<React.MouseEvent<HTMLDivElement> | null>,
    viewport: Viewport,
  ) =>
  <E extends HTMLElement>(event: React.MouseEvent<E>) => {
    addNewNode(
      event,
      nodesDispatch,
      NodeType.TRIGGER,
      "Empty Trigger Node",
      viewport,
    );
    setContextMenuMouseEvent(null);
  };

const onAddActionNode =
  (
    nodesDispatch: Dispatch<NodesAction>,
    setContextMenuMouseEvent: Dispatch<React.MouseEvent<HTMLDivElement> | null>,
    viewport: Viewport,
  ) =>
  <E extends HTMLElement>(event: React.MouseEvent<E>) => {
    addNewNode(
      event,
      nodesDispatch,
      NodeType.ACTION,
      "Empty Action Node",
      viewport,
    );
    setContextMenuMouseEvent(null);
  };

const addNewNode = <E extends HTMLElement>(
  event: React.MouseEvent<E>,
  nodesDispatch: Dispatch<NodesAction>,
  type: NodeType,
  label: string,
  viewport: Viewport,
) => {
  nodesDispatch({
    type: NodeActionType.ADD_NODES,
    payload: [
      {
        id: crypto.randomUUID(),
        type,
        position: {
          x: (event.clientX - viewport.x) / viewport.zoom - 160,
          y: (event.clientY - viewport.y) / viewport.zoom - 25,
        },
        data: { label },
      },
    ],
  });
};

//--------------------------------------------------------------------------------------------------
// Component
//--------------------------------------------------------------------------------------------------

const Canvas = () => {
  // == Hooks ==
  const [nodes, nodesDispatch] = useReducer<Reducer<Node[], NodesAction>>(
    nodeReducer,
    initialNodes,
  );

  const [edges, edgesDispatch] = useReducer<Reducer<Edge[], EdgesAction>>(
    edgeReducer,
    initialEdges,
  );

  const [contextMenuMouseEvent, setContextMenuMouseEvent] =
    useState<React.MouseEvent<HTMLDivElement> | null>(null);

  const viewport = useViewport();

  // == Handlers ==
  const { onNodesChange, onEdgesChange, onConnect } = useCanvas(
    nodes,
    edges,
    nodesDispatch,
    edgesDispatch,
  );

  const onContextMenu = (event: React.MouseEvent<HTMLDivElement>) => {
    event.preventDefault();
    setContextMenuMouseEvent(event);
  };

  // == Render ==
  return (
    <div className="h-full w-full">
      <ReactFlow
        panOnScroll
        selectionOnDrag
        panOnDrag={[1, 2]}
        snapToGrid
        snapGrid={[10, 10]}
        selectionMode={SelectionMode.Partial}
        proOptions={{ hideAttribution: true }}
        nodes={nodes}
        edges={edges}
        onNodesChange={onNodesChange}
        onEdgesChange={onEdgesChange}
        onConnect={onConnect}
        onContextMenu={onContextMenu}
        nodeTypes={nodeTypes}
      >
        <Background />
        {contextMenuMouseEvent && (
          <BackgroundContextMenu
            event={contextMenuMouseEvent}
            onAddTriggerNode={onAddTriggerNode(
              nodesDispatch,
              setContextMenuMouseEvent,
              viewport,
            )}
            onAddActionNode={onAddActionNode(
              nodesDispatch,
              setContextMenuMouseEvent,
              viewport,
            )}
            onAddNote={() => console.log("add note")}
          />
        )}
      </ReactFlow>
    </div>
  );
};

//--------------------------------------------------------------------------------------------------
// Exports
//--------------------------------------------------------------------------------------------------

export default Canvas;
