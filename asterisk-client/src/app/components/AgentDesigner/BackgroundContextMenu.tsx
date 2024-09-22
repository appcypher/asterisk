import { useEffect, useRef, useState } from "react";

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

type BackgroundContextMenuProps = {
  event: React.MouseEvent<HTMLDivElement>;
  onAddTrigger: () => void;
  onAddAction: () => void;
  onAddNote: () => void;
};

//--------------------------------------------------------------------------------------------------
// Component
//--------------------------------------------------------------------------------------------------

/**
 * BackgroundContextMenu is a component that renders a context menu for the background of the
 * agent designer.
 */
const BackgroundContextMenu = ({
  event,
  onAddTrigger,
  onAddAction,
  onAddNote,
}: BackgroundContextMenuProps) => {
  const [x, setX] = useState(0);
  const [y, setY] = useState(0);

  const ref = useRef<HTMLMenuElement>(null);

  // Update the position of the menu when the event changes
  useEffect(() => {
    const x = event.clientX;
    const y = event.clientY;

    const bounds = ref.current?.getBoundingClientRect();
    const width = bounds?.width ?? 0;
    const height = bounds?.height ?? 0;

    const windowWidth = window.innerWidth;
    const windowHeight = window.innerHeight;

    const notEnoughRightSpace = x + width > windowWidth;
    if (notEnoughRightSpace) {
      setX(x - 2 - width);
    } else {
      setX(x + 2);
    }

    const notEnoughBottomSpace = y + height > windowHeight;
    if (notEnoughBottomSpace) {
      setY(y - 2 - height);
    } else {
      setY(y + 2);
    }
  }, [event]);

  return (
    <menu
      ref={ref}
      style={{
        position: "absolute",
        left: x,
        top: y,
      }}
      className="
      bg-white p-2 rounded-lg shadow-lg
      flex flex-col gap-1
      border border-gray-200
      z-10"
    >
      <MenuItem
        icon="icon-[carbon--lightning]"
        text="Add Trigger Node"
        onClick={onAddTrigger}
      />
      <MenuItem
        icon="icon-[carbon--play]"
        text="Add Action Node"
        onClick={onAddAction}
      />
      <MenuItem
        icon="icon-[carbon--align-box-bottom-right]"
        text="Add Note"
        onClick={onAddNote}
      />
    </menu>
  );
};

const MenuItem = ({
  icon,
  text,
  onClick,
}: {
  icon: string;
  text: string;
  onClick: () => void;
}) => {
  return (
    <li
      className="
      flex items-center gap-2 w-44 p-2 h-8 rounded-md
      hover:bg-purple-100 hover:text-gray-700 hover:cursor-pointer
      active:bg-purple-200 active:text-gray-800 active:scale-[0.98]
      group/menu-item
      "
      onClick={onClick}
    >
      <span
        className={`h-4 ${icon} text-gray-400 group-hover/menu-item:text-gray-800`}
      ></span>
      <p className="text-sm text-gray-700 group-hover/menu-item:text-black select-none">
        {text}
      </p>
    </li>
  );
};

//--------------------------------------------------------------------------------------------------
// Exports
//--------------------------------------------------------------------------------------------------

export default BackgroundContextMenu;
