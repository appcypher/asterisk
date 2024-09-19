import AgentDesigner from "./components/AgentDesigner/ AgentDesigner";
import HoverZone from "./components/SidePanel/HoverZone";
import SidePanel from "./components/SidePanel/SidePanel";

//--------------------------------------------------------------------------------------------------
// Component
//--------------------------------------------------------------------------------------------------

const Home = () => {
  return (
    <div className="relative flex h-full">
      {false && <HoverZone />}
      {true && <SidePanel />}
      <AgentDesigner />
    </div>
  );
};

//--------------------------------------------------------------------------------------------------
// Exports
//--------------------------------------------------------------------------------------------------

export default Home;
