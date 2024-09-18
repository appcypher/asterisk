import { For } from "solid-js";
import type { Component } from "solid-js";
import Project from "./Project";

//--------------------------------------------------------------------------------------------------
// Component
//--------------------------------------------------------------------------------------------------

const ProjectGroup: Component = () => {
  return (
    <li class="flex flex-col gap-0.5">
      <For each={Array.from({ length: 10 })} fallback={<div>Loading...</div>}>
        {() => (
          <div>
            <Project />
            {/* <li class="odd:hidden">
              <For each={Array.from({ length: 10 })} fallback={<div>Loading...</div>}>
                {() => <Project />}
              </For>
            </li> */}
          </div>
        )}
      </For>
    </li>
  );
};

//--------------------------------------------------------------------------------------------------
// Exports
//--------------------------------------------------------------------------------------------------

export default ProjectGroup;
