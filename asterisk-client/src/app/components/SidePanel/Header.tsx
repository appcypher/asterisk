//--------------------------------------------------------------------------------------------------
// Component
//--------------------------------------------------------------------------------------------------

const Header = () => {
  return (
    <div className="m-1">
      <div
        className="
        flex items-center cursor-pointer h-10 rounded-lg p-2 gap-2
        ring-offset-2 ring-2 ring-purple-400
        hover:ring-purple-500 active:ring-purple-600
        hover:bg-purple-100 active:bg-purple-200
        active:scale-[0.99]
        group/header
        "
      >
        <div className="size-6 bg-gray-100 rounded-lg flex-none" />
        <p
          className="
          text-base truncate flex-auto select-none
          text-gray-700 group-hover/header:text-black
          "
        >
          Personal Workspace
        </p>
        <span className="icon-[carbon--chevron-sort] h-5" />
      </div>
    </div>
  );
};

//--------------------------------------------------------------------------------------------------
// Exports
//--------------------------------------------------------------------------------------------------

export default Header;
