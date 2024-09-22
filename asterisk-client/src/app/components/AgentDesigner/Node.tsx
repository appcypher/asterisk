import { Handle, NodeProps, Position } from "@xyflow/react";
import { Node } from "./types/node";

//--------------------------------------------------------------------------------------------------
// Component
//--------------------------------------------------------------------------------------------------

const TriggerNode = ({ data: { label } }: NodeProps<Node>) => {
  return (
    <div>
      <Handle type="source" position={Position.Top} />
      <p>{label}</p>
      <Handle type="target" position={Position.Bottom} />
    </div>
  );
};

//--------------------------------------------------------------------------------------------------
// Exports
//--------------------------------------------------------------------------------------------------

export default TriggerNode;
