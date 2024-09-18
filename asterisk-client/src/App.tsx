import type { Component } from "solid-js";
import SidePanel from "./SidePanel/SidePanel";
import AgentDesigner from "./AgentDesigner/AgentDesigner";
import HoverZone from "./SidePanel/HoverZone";

//--------------------------------------------------------------------------------------------------
// Component
//--------------------------------------------------------------------------------------------------

const App: Component = () => {
  return (
    <div class="relative flex h-full">
      <HoverZone />
      <SidePanel />
      <AgentDesigner />
    </div>
  );
};

//--------------------------------------------------------------------------------------------------
// Exports
//--------------------------------------------------------------------------------------------------

export default App;
