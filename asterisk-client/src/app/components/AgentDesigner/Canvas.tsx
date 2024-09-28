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
import { Dispatch, useCallback, useContext, useState } from "react";
import { Node, NodeActionType, NodesAction, NodeType } from "./state/nodes";
import { Edge, EdgesAction, EdgeActionType } from "./state/edges";
import { TriggerNode, ActionNode, NoteNode } from "./Node";
import ContextMenu from "./ContextMenu";
import Controls from "./Controls";
import { CanvasContext } from "./CanvasContextProvider";

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
  // ====== Hooks ======
  const viewport = useViewport();

  const { nodes, edges, nodesDispatch, edgesDispatch } =
    useContext(CanvasContext);

  const [paneContextMenuEvent, setPaneContextMenuEvent] = useState<
    React.MouseEvent<Element> | MouseEvent | null
  >(null);

  const [nodeContextMenuData, setNodeContextMenuData] = useState<{
    event: React.MouseEvent<Element>;
    node: Node;
  } | null>(null);

  // ====== Handlers ======
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

  // ====== Render ======
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
        // Specify where drag is allowed for Note Nodes.
        dragHandle: type === NodeType.NOTE ? ".note-rf-drag-area" : undefined,
        data: { label },
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
