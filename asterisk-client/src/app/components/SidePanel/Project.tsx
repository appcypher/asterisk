//--------------------------------------------------------------------------------------------------
// Component
//--------------------------------------------------------------------------------------------------

const Project = () => {
  return (
    <div
      className="
      flex flex-row gap-2 items-center
      cursor-pointer rounded-lg p-2 h-10 overflow-hidden
      bg-white
      hover:bg-purple-100 active:bg-purple-200
      group/project
      "
    >
      <div className="size-6 bg-gray-100 flex-none rounded-lg" />
      <p
        className="
        text-sm truncate flex-auto select-none
        text-gray-800
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
