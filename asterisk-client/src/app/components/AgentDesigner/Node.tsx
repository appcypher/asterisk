import { Handle, NodeProps, NodeResizer, Position } from "@xyflow/react";
import { Node } from "./types/node";
import {
  headingsPlugin,
  linkPlugin,
  listsPlugin,
  markdownShortcutPlugin,
  MDXEditor,
  quotePlugin,
  tablePlugin,
  thematicBreakPlugin,
} from "@mdxeditor/editor";
import "@mdxeditor/editor/style.css";

//--------------------------------------------------------------------------------------------------
// Component
//--------------------------------------------------------------------------------------------------

const TriggerNode = ({ data: { label } }: NodeProps<Node>) => {
  return (
    <>
      <NodeBox label={label ?? "Trigger"} />
      <Handle type="target" position={Position.Bottom} />
    </>
  );
};

const ActionNode = ({ data: { label } }: NodeProps<Node>) => {
  return (
    <>
      <Handle type="source" position={Position.Top} />
      <NodeBox label={label ?? "Action"} />
      <Handle type="target" position={Position.Bottom} />
    </>
  );
};

const NoteNode = () => {
  return (
    <div
      // className="
      //   w-60 p-2 bg-yellow-200 rounded-md border border-yellow-300 shadow-sm
      //   hover:cursor-pointer hover:shadow-md hover:border-purple-400 active:bg-yellow-300
      //   active:scale-[0.98]
      //   group/node-box
      //   "
      className="size-full overflow-auto"
    >
      <NodeResizer
        lineStyle={{
          borderWidth: 2,
          borderColor: "transparent",
        }}
        handleStyle={{
          border: "none",
          height: 10,
          width: 10,
          background: "transparent",
        }}
      />
      <MDXEditor
        markdown={""}
        plugins={[
          headingsPlugin(),
          listsPlugin(),
          linkPlugin(),
          quotePlugin(),
          thematicBreakPlugin(), // TODO: Not working
          tablePlugin(), // TODO: Not working
          markdownShortcutPlugin(),
        ]}
        // Using tailwind typography with some customizations
        contentEditableClassName="prose prose-mdxeditor"
      />
    </div>
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
        <p className="text-sm flex-auto text-gray-600 group-hover/node-box:text-black font-semibold">
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

export { TriggerNode, ActionNode, NoteNode };
