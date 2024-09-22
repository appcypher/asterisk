//--------------------------------------------------------------------------------------------------
// Component
//--------------------------------------------------------------------------------------------------

import { ReactFlowProvider } from "@xyflow/react";
import Canvas from "./Canvas";

const AgentDesigner = () => {
  return (
    <div className="size-full bg-purple-50 overflow-hidden">
      {/* Needed in order to have access to ReactFlow internal state */}
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
