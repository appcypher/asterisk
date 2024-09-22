"use client";

import { StrictMode } from "react";
import AgentDesigner from "./components/AgentDesigner/AgentDesigner";
import HoverZone from "./components/SidePanel/HoverZone";
import SidePanel from "./components/SidePanel/SidePanel";

//--------------------------------------------------------------------------------------------------
// Component
//--------------------------------------------------------------------------------------------------

const Home = () => {
  return (
    <StrictMode>
      <div className="relative flex h-full">
        {false && <HoverZone />}
        {true && <SidePanel />}
        <AgentDesigner />
      </div>
    </StrictMode>
  );
};

//--------------------------------------------------------------------------------------------------
// Exports
//--------------------------------------------------------------------------------------------------

export default Home;
