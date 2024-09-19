import { For } from "solid-js";
import type { Component } from "solid-js";
import ProjectGroup from "./ProjectGroup";

//--------------------------------------------------------------------------------------------------
// Component
//--------------------------------------------------------------------------------------------------

const Main: Component = () => {
  return (
    <li class="flex flex-col gap-6 overflow-hidden">
      <For each={Array.from({ length: 2 })} fallback={<div>Loading...</div>}>
        {() => <ProjectGroup />}
      </For>
    </li>
  );
};

//--------------------------------------------------------------------------------------------------
// Exports
//--------------------------------------------------------------------------------------------------

export default Main;
