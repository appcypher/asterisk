import type { Component } from "solid-js";

//--------------------------------------------------------------------------------------------------
// Component
//--------------------------------------------------------------------------------------------------

const Project: Component = () => {
  return (
    <div
      class="
      flex flex-row gap-2 items-center
      cursor-pointer rounded-lg p-2 h-10 overflow-hidden
      bg-white
      hover:bg-purple-100 active:bg-purple-200
      group/project
      "
    >
      <div class="size-6 bg-gray-100 flex-none rounded-lg" />
      <p
        class="
        text-sm truncate flex-auto select-none
        text-gray-800
        group-hover/project:text-black
        "
      >
        Project Name
      </p>
      <span class="hidden group-hover/project:block i-carbon-add h-5" />
    </div>
  );
};

//--------------------------------------------------------------------------------------------------
// Exports
//--------------------------------------------------------------------------------------------------

export default Project;
