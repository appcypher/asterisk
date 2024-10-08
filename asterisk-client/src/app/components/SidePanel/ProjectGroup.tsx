import Project from "./Project";

//--------------------------------------------------------------------------------------------------
// Component
//--------------------------------------------------------------------------------------------------

const ProjectGroup = () => {
  return (
    <div
      className="
        flex flex-col gap-1
        "
    >
      <div className="p-1">
        <div className="h-px bg-gray-100" />
      </div>
      <ul className="flex flex-col gap-1">
        {Array.from({ length: 10 }).map((_, index) => (
          <li key={index}>
            <Project />
          </li>
        ))}
      </ul>
    </div>
  );
};

//--------------------------------------------------------------------------------------------------
// Exports
//--------------------------------------------------------------------------------------------------

export default ProjectGroup;
