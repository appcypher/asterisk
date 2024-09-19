import type { Component } from "solid-js";

//--------------------------------------------------------------------------------------------------
// Component
//--------------------------------------------------------------------------------------------------

const Canvas: Component = () => {
  return (
    <div
      class="
      absolute inset-0
      h-full w-full
      bg-[radial-gradient(#9ca3af_1px,transparent_1px)]
      [background-size:30px_30px]
      "
    >
      {/* <div class="absolute inset-0 h-full w-full bg-white" /> */}
    </div>
  );
};

//--------------------------------------------------------------------------------------------------
// Exports
//--------------------------------------------------------------------------------------------------

export default Canvas;
