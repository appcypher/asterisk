import { CSSProperties, useCallback, useEffect, useRef, useState } from "react";

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

type LineProps = {
  position: LinePosition;
  hide?: boolean;
  style?: CSSProperties;
};

enum LinePosition {
  LEFT = "left",
  RIGHT = "right",
  BOTTOM = "bottom",
  TOP = "top",
}

//--------------------------------------------------------------------------------------------------
// Component
//--------------------------------------------------------------------------------------------------

const Line = ({ position, hide, style }: LineProps) => {
  const ref = useRef<HTMLDivElement>(null);
  const [midpoint, setMidpoint] = useState(0);
  const [isDragging, setIsDragging] = useState(false);

  const onMouseDown = useCallback(() => {
    console.log("Mouse down on handle: ", position);
    setIsDragging(true);
  }, [position]);

  const onMouseUp = useCallback(() => {
    console.log("Mouse up on handle: ", position);
    setIsDragging(false);
  }, [position]);

  const onMouseMove = useCallback((event: MouseEvent) => {
    console.log("Mouse move on: ", event);
  }, []);

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

  // Compute the midpoint of the line
  useEffect(() => {
    if (ref.current) {
      const bounds = ref.current.getBoundingClientRect();
      switch (position) {
        case LinePosition.BOTTOM:
        case LinePosition.TOP:
          setMidpoint(bounds.height / 2);
          break;
        case LinePosition.LEFT:
        case LinePosition.RIGHT:
          setMidpoint(bounds.width / 2);
      }
      console.log(`Midpoint: ${midpoint}`);
    }

    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  // Get the position styles
  const positionStyles = getPositionStyles(position, midpoint);

  return (
    <div
      ref={ref}
      className={`absolute ${hide ? "bg-transparent" : "bg-purple-400"}`}
      style={{ ...positionStyles, ...style }}
      onMouseDown={onMouseDown}
    ></div>
  );
};

//--------------------------------------------------------------------------------------------------
// Helpers
//--------------------------------------------------------------------------------------------------

const getPositionStyles = (
  position: LinePosition,
  midpoint: number,
): CSSProperties => {
  switch (position) {
    case LinePosition.LEFT:
      return {
        left: -midpoint,
        top: 0,
        height: "100%",
        width: "4px",
        cursor: "w-resize",
      };
    case LinePosition.RIGHT:
      return {
        right: -midpoint,
        top: 0,
        height: "100%",
        width: "4px",
        cursor: "e-resize",
      };
    case LinePosition.BOTTOM:
      return {
        bottom: -midpoint,
        left: 0,
        height: "4px",
        width: "100%",
        cursor: "s-resize",
      };
    case LinePosition.TOP:
      return {
        top: -midpoint,
        left: 0,
        height: "4px",
        width: "100%",
        cursor: "n-resize",
      };
    default:
      return {};
  }
};

//--------------------------------------------------------------------------------------------------
// Exports
//--------------------------------------------------------------------------------------------------

export { Line, LinePosition };
