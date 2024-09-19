import type { Component } from "solid-js";

//--------------------------------------------------------------------------------------------------
// Component
//--------------------------------------------------------------------------------------------------

const Project: Component = () => {
  return (
    <div class="flex flex-row gap-2 cursor-pointer rounded-lg p-1 h-10 hover:bg-bg-selected active:bg-bg-active active:border-border-highlight">
      <div class="size-6 bg-gray-100 rounded-lg" />
      <div class=" flex items-center flex-auto overflow-hidden">
        <p class="text-sm truncate">Project Name</p>
      </div>
      <div class="flex items-center justify-center">
        <span class="i-carbon-chevron-down h-4" />
      </div>
    </div>
  );
};

//--------------------------------------------------------------------------------------------------
// Exports
//--------------------------------------------------------------------------------------------------

export default Project;
