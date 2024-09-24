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
import { NodesAction, Node, NodeActionType, NodeType } from "./types/node";
import { Edge, EdgeActionType, EdgesAction } from "./types/edge";
import { edgeReducer, initialEdges } from "./state/edges";
import { TriggerNode, ActionNode, NoteNode } from "./Node";
import ContextMenu from "./ContextMenu";
import Controls from "./Controls";

//--------------------------------------------------------------------------------------------------
// State
//--------------------------------------------------------------------------------------------------

const nodeTypes: NodeTypes = {
  TRIGGER: TriggerNode,
  ACTION: ActionNode,
  NOTE: NoteNode,
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
// Component
//--------------------------------------------------------------------------------------------------

const Canvas = () => {
  // == Hooks ==
  const viewport = useViewport();

  const [nodes, nodesDispatch] = useReducer<Reducer<Node[], NodesAction>>(
    nodeReducer,
    initialNodes,
  );

  const [edges, edgesDispatch] = useReducer<Reducer<Edge[], EdgesAction>>(
    edgeReducer,
    initialEdges,
  );

  const [paneContextMenuEvent, setPaneContextMenuEvent] = useState<
    React.MouseEvent<Element> | MouseEvent | null
  >(null);

  const [nodeContextMenuData, setNodeContextMenuData] = useState<{
    event: React.MouseEvent<Element>;
    node: Node;
  } | null>(null);

  // == Handlers ==
  const { onNodesChange, onEdgesChange, onConnect } = useCanvas(
    nodes,
    edges,
    nodesDispatch,
    edgesDispatch,
  );

  const onPaneContextMenu = (event: React.MouseEvent<Element> | MouseEvent) => {
    event.preventDefault();
    setPaneContextMenuEvent(event);
  };

  const onNodeContextMenu = (event: React.MouseEvent<Element>, node: Node) => {
    event.preventDefault();
    setNodeContextMenuData({ event, node });
  };

  // == Render ==
  return (
    <div className="h-full w-full">
      <ReactFlow
        panOnScroll
        selectionOnDrag
        panOnDrag={[1]}
        snapToGrid
        snapGrid={[10, 10]}
        selectionMode={SelectionMode.Partial}
        proOptions={{ hideAttribution: true }}
        nodes={nodes}
        edges={edges}
        onNodesChange={onNodesChange}
        onEdgesChange={onEdgesChange}
        onConnect={onConnect}
        onPaneContextMenu={onPaneContextMenu}
        onNodeContextMenu={onNodeContextMenu}
        nodeTypes={nodeTypes}
      >
        <Background />
        <Controls />
        {paneContextMenuEvent && (
          <ContextMenu
            event={paneContextMenuEvent}
            items={[
              {
                text: "Add Trigger Node",
                icon: "icon-[carbon--lightning]",
                onClick: (event) => {
                  setPaneContextMenuEvent(null);
                  addNewNode(
                    event,
                    viewport,
                    nodesDispatch,
                    NodeType.TRIGGER,
                    "Empty Trigger Node",
                  );
                },
              },
              {
                text: "Add Action Node",
                icon: "icon-[carbon--play]",
                onClick: (event) => {
                  setPaneContextMenuEvent(null);
                  addNewNode(
                    event,
                    viewport,
                    nodesDispatch,
                    NodeType.ACTION,
                    "Empty Action Node",
                  );
                },
              },
              {
                text: "Add Note",
                icon: "icon-[carbon--align-box-bottom-right]",
                onClick: (event) => {
                  setPaneContextMenuEvent(null);
                  addNewNode(event, viewport, nodesDispatch, NodeType.NOTE);
                },
              },
            ]}
          />
        )}
        {nodeContextMenuData && (
          <ContextMenu
            event={nodeContextMenuData.event}
            items={[
              {
                text: "Edit Node",
                icon: "icon-[carbon--edit]",
                onClick: () => {
                  setNodeContextMenuData(null);
                  console.log("edit node");
                },
              },
              {
                text: "Delete Node",
                icon: "icon-[carbon--trash-can]",
                warn: true,
                onClick: () => {
                  removeNode(nodesDispatch, nodeContextMenuData.node);
                  setNodeContextMenuData(null);
                },
              },
            ]}
          />
        )}
      </ReactFlow>
    </div>
  );
};

//--------------------------------------------------------------------------------------------------
// Helpers
//--------------------------------------------------------------------------------------------------

const addNewNode = <E extends HTMLElement>(
  event: React.MouseEvent<E>,
  viewport: Viewport,
  nodesDispatch: Dispatch<NodesAction>,
  type: NodeType,
  label?: string,
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
        className:
          type === NodeType.NOTE
            ? `
              w-60 p-2 bg-yellow-200 rounded-md border border-yellow-300 shadow-sm
              hover:cursor-pointer hover:shadow-md hover:border-purple-400 active:bg-yellow-300
              active:scale-[0.98]
              group/node-box
              `
            : "",
      },
    ],
  });
};

const removeNode = (nodesDispatch: Dispatch<NodesAction>, node: Node) => {
  nodesDispatch({
    type: NodeActionType.REMOVE_NODES,
    payload: [node],
  });
};

//--------------------------------------------------------------------------------------------------
// Exports
//--------------------------------------------------------------------------------------------------

export default Canvas;
