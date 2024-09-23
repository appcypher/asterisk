import { ReactFlowProvider } from "@xyflow/react";
import Canvas from "./Canvas";

//--------------------------------------------------------------------------------------------------
// Component
//--------------------------------------------------------------------------------------------------

const AgentDesigner = () => {
  return (
    <div className="size-full bg-purple-50 overflow-hidden">
      <ReactFlowProvider>
        <Canvas />
      </ReactFlowProvider>
    </div>
  );
};

//--------------------------------------------------------------------------------------------------
// Exports
//--------------------------------------------------------------------------------------------------

export default AgentDesigner;
