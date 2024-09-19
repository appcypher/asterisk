import { Show, type Component } from "solid-js";
import SidePanel from "./SidePanel/SidePanel";
import AgentDesigner from "./AgentDesigner/AgentDesigner";
import HoverZone from "./SidePanel/HoverZone";

//--------------------------------------------------------------------------------------------------
// Component
//--------------------------------------------------------------------------------------------------

const App: Component = () => {
  return (
    <div class="relative flex h-full">
      <Show when={false}>
        <HoverZone />
        <SidePanel />
      </Show>
      <AgentDesigner />
    </div>
  );
};

//--------------------------------------------------------------------------------------------------
// Exports
//--------------------------------------------------------------------------------------------------

export default App;
