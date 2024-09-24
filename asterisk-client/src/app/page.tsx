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
        <AgentDesigner />
        {false && <HoverZone />}
        {true && <SidePanel />}
      </div>
    </StrictMode>
  );
};

//--------------------------------------------------------------------------------------------------
// Exports
//--------------------------------------------------------------------------------------------------

export default Home;
