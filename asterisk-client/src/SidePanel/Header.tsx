import type { Component } from "solid-js";

//--------------------------------------------------------------------------------------------------
// Component
//--------------------------------------------------------------------------------------------------

const Header: Component = () => {
  return (
    <div class="flex items-center cursor-pointer bg-cyan-100 h-16 rounded-2xl p-2 gap-1 border-2 border-red-400">
      <div class="size-10 bg-gray-400 rounded-lg flex-none" />
      <div class="bg-white h-full flex items-center flex-auto overflow-hidden">
        <p class="text-lg bg-white truncate">My Personal Workspace Here we </p>
      </div>
      <span class="i-carbon-chevron-sort h-5 bg-blue-200 hover:bg-blue-400" />
    </div>
  );
};

//--------------------------------------------------------------------------------------------------
// Exports
//--------------------------------------------------------------------------------------------------

export default Header;
