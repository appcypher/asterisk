import { Handle, NodeProps, Position } from "@xyflow/react";
import { Node } from "./types/node";

//--------------------------------------------------------------------------------------------------
// Component
//--------------------------------------------------------------------------------------------------

const TriggerNode = ({ data: { label } }: NodeProps<Node>) => {
  return (
    <>
      <NodeBox label={label} />
      <Handle type="target" position={Position.Bottom} />
    </>
  );
};

const ActionNode = ({ data: { label } }: NodeProps<Node>) => {
  return (
    <>
      <Handle type="source" position={Position.Top} />
      <NodeBox label={label} />
      <Handle type="target" position={Position.Bottom} />
    </>
  );
};

const NodeBox = ({ label }: { label: string }) => {
  return (
    <div
      className="
      bg-white border border-gray-300 rounded-lg p-0.5
      shadow-sm w-80 h-12
      hover:cursor-pointer hover:shadow-md hover:border-purple-400
      active:scale-[0.99]
      group/node-box
      "
    >
      <div
        className="
        flex flex-row gap-2 items-center justify-start h-full p-2 rounded-md
        group-hover/node-box:bg-purple-50 group-active/node-box:bg-purple-100
        "
      >
        <div className="size-6 bg-gray-300 rounded-md flex-none" />
        <p className="text-sm flex-auto text-gray-600 group-hover/node-box:text-black font-bold">
          {label}
        </p>
      </div>
      <div
        className="
        absolute flex items-center justify-center -top-2 right-3 size-6 bg-yellow-300 text-lg rounded-full
        group-hover/node-box:border group-hover/node-box:border-purple-400
        "
      >
        <span className="icon-[humbleicons--exclamation] size-4 text-yellow-600" />
      </div>
    </div>
  );
};

//--------------------------------------------------------------------------------------------------
// Exports
//--------------------------------------------------------------------------------------------------

export { TriggerNode, ActionNode };
