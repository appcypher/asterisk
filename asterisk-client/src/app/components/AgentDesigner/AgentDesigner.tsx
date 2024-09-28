import { ReactFlowProvider } from "@xyflow/react";
import Canvas from "./Canvas";
import { CanvasContextProvider } from "./CanvasContextProvider";

//--------------------------------------------------------------------------------------------------
// Component
//--------------------------------------------------------------------------------------------------

const AgentDesigner = () => {
  return (
    <div className="size-full bg-purple-50 overflow-hidden">
      <ReactFlowProvider>
        <CanvasContextProvider>
          <Canvas />
        </CanvasContextProvider>
      </ReactFlowProvider>
    </div>
  );
};

//--------------------------------------------------------------------------------------------------
// Exports
//--------------------------------------------------------------------------------------------------

export default AgentDesigner;
