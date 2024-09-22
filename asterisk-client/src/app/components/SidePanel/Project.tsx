//--------------------------------------------------------------------------------------------------
// Component
//--------------------------------------------------------------------------------------------------

const Project = () => {
  return (
    <div
      className="
      flex flex-row gap-2 items-center
      cursor-pointer rounded-md p-2 h-8 overflow-hidden
      bg-white
      hover:bg-purple-100 active:bg-purple-200 active:scale-[0.98]
      group/project
      "
    >
      <div className="size-4 bg-gray-100 flex-none" />
      <p
        className="
        text-sm truncate flex-auto select-none
        text-gray-700
        group-hover/project:text-black
        "
      >
        Project Name
      </p>
      <span className="hidden group-hover/project:block icon-[carbon--add] h-5" />
    </div>
  );
};

//--------------------------------------------------------------------------------------------------
// Exports
//--------------------------------------------------------------------------------------------------

export default Project;
