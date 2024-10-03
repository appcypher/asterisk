import { Handle, Position } from "@xyflow/react";

//--------------------------------------------------------------------------------------------------
// Component
//--------------------------------------------------------------------------------------------------

const NodeHandle = ({
  type,
  position,
}: {
  type: "source" | "target";
  position: Position;
}) => {
  return (
    <Handle
      type={type}
      position={position}
      className={`
      ${type === "source" ? "!size-2 !bg-gray-300 !border-none !-bottom-2" : "!w-3 !h-1 !bg-gray-300 !rounded-sm !border-none !-top-2"}
      `}
    />
  );
};

//--------------------------------------------------------------------------------------------------
// Exports
//--------------------------------------------------------------------------------------------------

export default NodeHandle;
