import {
  CSSProperties,
  Dispatch,
  SetStateAction,
  useCallback,
  useContext,
  useEffect,
  useRef,
  useState,
} from "react";
import { Node, NodeActionType } from "../state/nodes";
import { CSSDimensions } from ".";
import { CanvasContext } from "../CanvasContextProvider";
import { useViewport } from "@xyflow/react";

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

type HandleProps = {
  position: HandlePosition;
  hide?: boolean;
  style?: CSSProperties;
  node: Node;
  setDimensions: Dispatch<SetStateAction<CSSDimensions>>;
};

enum HandlePosition {
  TOP_LEFT = "top-left",
  TOP_RIGHT = "top-right",
  BOTTOM_LEFT = "bottom-left",
  BOTTOM_RIGHT = "bottom-right",
}

//--------------------------------------------------------------------------------------------------
// Component
//--------------------------------------------------------------------------------------------------

const Handle = ({
  position,
  hide,
  style,
  node,
  // setDimensions,
}: HandleProps) => {
  const ref = useRef<HTMLDivElement>(null);
  const [midWidth, setMidWidth] = useState(0);
  const [midHeight, setMidHeight] = useState(0);
  const [isDragging, setIsDragging] = useState(false);
  const { nodesDispatch } = useContext(CanvasContext);
  const { zoom } = useViewport();

  const onMouseDown = useCallback(() => {
    console.log("Mouse down on handle: ", position);
    setIsDragging(true);
  }, [position]);

  const onMouseUp = useCallback(() => {
    console.log("Mouse up on handle: ", position);
    setIsDragging(false);
  }, [position]);

  const onMouseMove = useCallback(
    (event: MouseEvent) => {
      console.log("Mouse move on: ", event);
      if (isDragging) {
        nodesDispatch({
          type: NodeActionType.SYNC_NODES,
          payload: [
            {
              ...node,
              // TODO: Rather than adding the movement to the node's position, we should be
              // calculating the new position based on the mouse position. Will tackle this problem
              // later.
              position: {
                x: node.position.x + event.movementX / zoom,
                y: node.position.y + event.movementY / zoom,
              },
            },
          ],
        });
      }
    },
    [isDragging, nodesDispatch, node, zoom],
  );

  // Setting and removing event listeners
  useEffect(() => {
    if (isDragging) {
      window.addEventListener("mousemove", onMouseMove);
      window.addEventListener("mouseup", onMouseUp);
    } else {
      window.removeEventListener("mousemove", onMouseMove);
      window.removeEventListener("mouseup", onMouseUp);
    }

    return () => {
      window.removeEventListener("mousemove", onMouseMove);
      window.removeEventListener("mouseup", onMouseUp);
    };
  }, [isDragging, onMouseMove, onMouseUp]);

  // Setting mid width and height
  useEffect(() => {
    if (ref.current) {
      const { width, height } = ref.current.getBoundingClientRect();
      setMidWidth(width / 2);
      setMidHeight(height / 2);
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [hide]);

  const positionStyles = getPositionStyles(position, midWidth, midHeight);

  return (
    <div
      ref={ref}
      className={`absolute ${hide ? "bg-transparent" : "bg-purple-400"}`}
      style={{
        width: `8px`,
        height: `8px`,
        ...positionStyles,
        ...style,
      }}
      onMouseDown={onMouseDown}
    ></div>
  );
};

//--------------------------------------------------------------------------------------------------
// Helpers
//--------------------------------------------------------------------------------------------------

const getPositionStyles = (
  position: HandlePosition,
  midWidth: number,
  midHeight: number,
): CSSProperties => {
  switch (position) {
    case HandlePosition.TOP_LEFT:
      return {
        top: -midHeight,
        left: -midWidth,
        cursor: "nw-resize",
      };
    case HandlePosition.TOP_RIGHT:
      return {
        top: -midHeight,
        right: -midWidth,
        cursor: "ne-resize",
      };
    case HandlePosition.BOTTOM_LEFT:
      return {
        bottom: -midHeight,
        left: -midWidth,
        cursor: "sw-resize",
      };
    case HandlePosition.BOTTOM_RIGHT:
      return {
        bottom: -midHeight,
        right: -midWidth,
        cursor: "se-resize",
      };
    default:
      return {};
  }
};

//--------------------------------------------------------------------------------------------------
// Exports
//--------------------------------------------------------------------------------------------------

export { Handle, HandlePosition };
