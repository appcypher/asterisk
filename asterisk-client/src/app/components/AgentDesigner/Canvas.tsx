import {
  addEdge,
  applyEdgeChanges,
  applyNodeChanges,
  Background,
  Connection,
  Controls,
  EdgeChange,
  NodeChange,
  NodeTypes,
  ReactFlow,
  SelectionMode,
} from "@xyflow/react";
import "@xyflow/react/dist/style.css";
import { Reducer, useCallback, useReducer, useState } from "react";
import { initialNodes, nodeReducer } from "./state/nodes";
import BackgroundContextMenu from "./BackgroundContextMenu";
import { NodesAction, Node, NodeActionType } from "./types/node";
import { Edge, EdgeActionType, EdgesAction } from "./types/edge";
import { edgeReducer, initialEdges } from "./state/edges";
import TriggerNode from "./Node";

//--------------------------------------------------------------------------------------------------
// State
//--------------------------------------------------------------------------------------------------

const nodeTypes: NodeTypes = {
  TRIGGER: TriggerNode,
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

  // == Handlers ==
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

  const onContextMenu = (event: React.MouseEvent<HTMLDivElement>) => {
    event.preventDefault();
    setContextMenuMouseEvent(event);
  };

  // == Render ==
  return (
    <div className="h-full w-full">
      <ReactFlow
        className="!cursor-pointer"
        panOnScroll
        selectionOnDrag
        panOnDrag={[1, 2]}
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
        <Controls /> {/* TODO: Add custom controls */}
        {contextMenuMouseEvent && (
          <BackgroundContextMenu
            event={contextMenuMouseEvent}
            onAddTrigger={() => {
              console.log("add trigger");
            }}
            onAddAction={() => {
              console.log("add action");
            }}
            onAddNote={() => {
              console.log("add note");
            }}
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
