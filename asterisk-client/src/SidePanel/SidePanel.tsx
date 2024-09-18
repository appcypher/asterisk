import type { Component } from "solid-js";
import Header from "./Header";
import Main from "./Main";

//--------------------------------------------------------------------------------------------------
// Component
//--------------------------------------------------------------------------------------------------

const SidePanel: Component = () => {
  return (
    <section
      class="
        absolute flex flex-col gap-1 bg-gray-500 h-[calc(100%-0.5rem)]
        m-1 w-80 rounded-2xl p-0.5 left-[-100%]
        peer-hover/hover-zone:flex
        peer-hover/hover-zone:left-[0]
        hover:flex hover:left-[0]
        transition-all ease-in-out duration-300"
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
