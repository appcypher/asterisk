import type { Component } from "solid-js";
import Canvas from "./Canvas";

//--------------------------------------------------------------------------------------------------
// Component
//--------------------------------------------------------------------------------------------------

const AgentDesigner: Component = () => {
  return (
    // Viewport
    <div class="h-[1000px] w-[1000px] bg-purple-50 overflow-hidden">
      <Canvas />
    </div>
  );
};

//--------------------------------------------------------------------------------------------------
// Exports
//--------------------------------------------------------------------------------------------------

export default AgentDesigner;
