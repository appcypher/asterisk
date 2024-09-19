import type { Component } from "solid-js";

//--------------------------------------------------------------------------------------------------
// Component
//--------------------------------------------------------------------------------------------------

const Header: Component = () => {
  return (
    <div class="flex items-center cursor-pointer h-16 rounded-2xl p-2 gap-1 border-2 ">
      <div class="size-10 bg-gray-100 rounded-lg flex-none" />
      <div class="h-full flex items-center flex-auto overflow-hidden">
        <p class="text-lg font-bold truncate">My Personal Workspace</p>
      </div>
      <span class="i-carbon-chevron-sort h-5" />
    </div>
  );
};

//--------------------------------------------------------------------------------------------------
// Exports
//--------------------------------------------------------------------------------------------------

export default Header;
