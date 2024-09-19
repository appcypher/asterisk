import Header from "./Header";
import Main from "./Main";

//--------------------------------------------------------------------------------------------------
// Component
//--------------------------------------------------------------------------------------------------

const SidePanel = () => {
  return (
    <section
      className="
        absolute flex flex-col gap-1 h-[calc(100%-0.5rem)] bg-white
        border border-border-passive
        m-1 w-70 rounded-lg p-2"
    >
      <Header />
      <Main />
    </section>
  );
};

//--------------------------------------------------------------------------------------------------
// Exports
//--------------------------------------------------------------------------------------------------

export default SidePanel;
