//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

type IconTrayItemProps = {
  filled_icon: string;
  regular_icon: string;
  selected?: boolean;
};

//--------------------------------------------------------------------------------------------------
// Component
//--------------------------------------------------------------------------------------------------

const IconTray = () => {
  return (
    <ul className="flex flex-row gap-2 justify-start">
      <IconTrayItem
        filled_icon="icon-[fluent--bot-sparkle-20-filled]"
        regular_icon="icon-[fluent--bot-sparkle-20-regular]"
        selected
      />
      <IconTrayItem
        filled_icon="icon-[fluent--shield-lock-16-filled]"
        regular_icon="icon-[fluent--shield-lock-16-regular]"
        // selected
      />
      <IconTrayItem
        filled_icon="icon-[fluent--box-16-filled]"
        regular_icon="icon-[fluent--box-16-regular]"
        // selected
      />
    </ul>
  );
};

const IconTrayItem = ({
  filled_icon,
  regular_icon,
  selected = false,
}: IconTrayItemProps) => {
  return (
    <li>
      <button
        className={`
          rounded-lg  active:scale-95
          size-10 flex items-center justify-center ${selected ? "bg-violet-600 hover:bg-violet-700 " : "bg-white hover:bg-purple-100 active:bg-purple-200"}
          group/icon-tray-item
        `}
      >
        <span
          className={`
            size-6
            ${selected ? filled_icon : regular_icon}
            ${selected ? "text-white" : "text-gray-500 group-hover/icon-tray-item:text-gray-900"}
          `}
        />
      </button>
    </li>
  );
};

//--------------------------------------------------------------------------------------------------
// Exports
//--------------------------------------------------------------------------------------------------

export default IconTray;
