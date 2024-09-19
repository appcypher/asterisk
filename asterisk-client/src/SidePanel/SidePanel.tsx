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
        absolute flex flex-col gap-1 h-[calc(100%-0.5rem)] bg-bg-passive
        border border-border-passive
        m-1 w-80 rounded-2xl p-0.5"
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
