import { For } from "solid-js";
import type { Component } from "solid-js";
import Project from "./Project";

//--------------------------------------------------------------------------------------------------
// Component
//--------------------------------------------------------------------------------------------------

const ProjectGroup: Component = () => {
  return (
    <div
      class="
        flex flex-col gap-1 mt-2
        "
    >
      <div
        class="
          flex items-center gap-2
          "
      >
        <span class="h-0.5 flex-auto bg-gradient-to-l from-gray-100 to-gray-50" />
        <span class="text-gray-400 text-sm truncate select-none flex-none">
          Project Group
        </span>
        <span class="h-0.5 flex-auto bg-gradient-to-r from-gray-100 to-gray-50" />
      </div>
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
    </div>
  );
};

//--------------------------------------------------------------------------------------------------
// Exports
//--------------------------------------------------------------------------------------------------

export default ProjectGroup;
