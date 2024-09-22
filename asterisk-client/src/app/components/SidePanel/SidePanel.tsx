import Header from "./Header";
import IconTray from "./IconTray";
import Main from "./Main";

//--------------------------------------------------------------------------------------------------
// Component
//--------------------------------------------------------------------------------------------------

const SidePanel = () => {
  return (
    <section
      className="
        absolute flex flex-col gap-2 h-[calc(100%-0.5rem)] bg-white
        border border-gray-200
        m-1 w-70 rounded-lg p-2"
    >
      <Header />
      <IconTray />
      <Main />
    </section>
  );
};

//--------------------------------------------------------------------------------------------------
// Exports
//--------------------------------------------------------------------------------------------------

export default SidePanel;
