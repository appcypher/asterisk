import Project from "./Project";

//--------------------------------------------------------------------------------------------------
// Component
//--------------------------------------------------------------------------------------------------

const ProjectGroup = () => {
  return (
    <div
      className="
        flex flex-col gap-1 mt-2
        "
    >
      <div
        className="
          flex items-center gap-2
          "
      >
        <span className="h-0.5 flex-auto bg-gradient-to-l from-gray-100 to-gray-50" />
        <span className="text-gray-400 text-sm truncate select-none flex-none">
          Project Group
        </span>
        <span className="h-0.5 flex-auto bg-gradient-to-r from-gray-100 to-gray-50" />
      </div>
      <li className="flex flex-col gap-0.5">
        {Array.from({ length: 10 }).map((_, index) => (
          <ul key={index}>
            <Project />
          </ul>
        ))}
      </li>
    </div>
  );
};

//--------------------------------------------------------------------------------------------------
// Exports
//--------------------------------------------------------------------------------------------------

export default ProjectGroup;
