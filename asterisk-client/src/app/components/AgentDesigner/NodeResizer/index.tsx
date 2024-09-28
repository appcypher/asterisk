import React, { CSSProperties, useState } from "react";
import { Line, LinePosition } from "./Line";
import { Handle, HandlePosition } from "./Handle";
import { useReactFlow } from "@xyflow/react";
import { Node } from "../state/nodes";

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

type NodeResizerProps = {
  id: string;
  children: React.ReactNode;
  hideControls?: boolean;
  minWidth?: number;
  minHeight?: number;
  maxWidth?: number;
  maxHeight?: number;
  keepAspectRatio?: boolean;
  handleStyle?: CSSProperties;
  lineStyle?: CSSProperties;
};

type CSSDimensions = {
  width: string;
  height: string;
};

//--------------------------------------------------------------------------------------------------
// Component
//--------------------------------------------------------------------------------------------------

const NodeResizer = ({
  id,
  children,
  hideControls,
  handleStyle,
  lineStyle,
}: NodeResizerProps) => {
  const [dimensions, setDimensions] = useState<CSSDimensions>({
    width: "auto",
    height: "auto",
  });
  const { getNode } = useReactFlow();
  const node: Node = getNode(id)!;

  return (
    <div
      style={{
        width: dimensions?.width,
        height: dimensions?.height,
      }}
    >
      {React.Children.only(children)}

      {/* Left Line */}
      <Line
        position={LinePosition.LEFT}
        style={lineStyle}
        hide={hideControls}
      />

      {/* Right Line */}
      <Line
        position={LinePosition.RIGHT}
        style={lineStyle}
        hide={hideControls}
      />

      {/* Bottom Line */}
      <Line
        position={LinePosition.BOTTOM}
        style={lineStyle}
        hide={hideControls}
      />

      {/* Top Line */}
      <Line position={LinePosition.TOP} style={lineStyle} hide={hideControls} />

      {/* Top Left Handle */}
      <Handle
        position={HandlePosition.TOP_LEFT}
        hide={hideControls}
        style={handleStyle}
        setDimensions={setDimensions}
        node={node}
      />

      {/* Top Right Handle */}
      <Handle
        position={HandlePosition.TOP_RIGHT}
        hide={hideControls}
        style={handleStyle}
        setDimensions={setDimensions}
        node={node}
      />

      {/* Bottom Left Handle */}
      <Handle
        position={HandlePosition.BOTTOM_LEFT}
        hide={hideControls}
        style={handleStyle}
        setDimensions={setDimensions}
        node={node}
      />

      {/* Bottom Right Handle */}
      <Handle
        position={HandlePosition.BOTTOM_RIGHT}
        hide={hideControls}
        style={handleStyle}
        setDimensions={setDimensions}
        node={node}
      />
    </div>
  );
};

//--------------------------------------------------------------------------------------------------
// Exports
//--------------------------------------------------------------------------------------------------

export { NodeResizer };
export type { CSSDimensions };
