import type { Component } from "solid-js";

//--------------------------------------------------------------------------------------------------
// Component
//--------------------------------------------------------------------------------------------------

const Project: Component = () => {
  return (
    <div class="flex flex-row gap-1 cursor-pointer bg-orange-700 rounded-lg p-1 hover:bg-orange-500">
      <div class="size-8 bg-gray-400 rounded-lg" />
      <div class="bg-white h-full flex items-center flex-auto overflow-hidden">
        <p class="text-sm bg-white truncate">Project Name</p>
      </div>
      <div class="flex items-center justify-center bg-orange-200">
        <span class="i-carbon-chevron-down h-4 bg-blue-200 hover:bg-blue-400" />
      </div>
    </div>
  );
};

//--------------------------------------------------------------------------------------------------
// Exports
//--------------------------------------------------------------------------------------------------

export default Project;
