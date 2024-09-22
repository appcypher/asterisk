import { useEffect, useRef, useState } from "react";

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

type BackgroundContextMenuProps = {
  event: React.MouseEvent<HTMLDivElement>;
  onAddTriggerNode: (event: React.MouseEvent<HTMLLIElement>) => void;
  onAddActionNode: (event: React.MouseEvent<HTMLLIElement>) => void;
  onAddNote: (event: React.MouseEvent<HTMLLIElement>) => void;
};

//--------------------------------------------------------------------------------------------------
// Hooks
//--------------------------------------------------------------------------------------------------

/**
 * useContextMenuPosition is a hook that updates the position of a context menu based on the event
 * and the dimensions of the menu.
 */
const useContextMenuPosition = (
  event: React.MouseEvent<HTMLDivElement>,
  ref: React.RefObject<HTMLMenuElement>,
  setX: React.Dispatch<React.SetStateAction<number>>,
  setY: React.Dispatch<React.SetStateAction<number>>,
  setVisible: React.Dispatch<React.SetStateAction<boolean>>,
) => {
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

    setVisible(true);
  }, [event, ref, setX, setY, setVisible]);
};

/**
 * useOutsideClick is a hook that updates the position of a context menu based on the event
 * and the dimensions of the menu.
 */
const useOutsideClick = (
  ref: React.RefObject<HTMLMenuElement>,
  setX: React.Dispatch<React.SetStateAction<number>>,
  setY: React.Dispatch<React.SetStateAction<number>>,
  setVisible: React.Dispatch<React.SetStateAction<boolean>>,
) => {
  useEffect(() => {
    const handleClickOutside = (event: MouseEvent) => {
      if (ref.current && !ref.current.contains(event.target as Node)) {
        setX(0);
        setY(0);
        setVisible(false);
      }
    };

    window.addEventListener("mousedown", handleClickOutside);
    return () => window.removeEventListener("mousedown", handleClickOutside);
  }, [ref, setX, setY, setVisible]);
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
  onAddTriggerNode,
  onAddActionNode,
  onAddNote,
}: BackgroundContextMenuProps) => {
  const [x, setX] = useState(0);
  const [y, setY] = useState(0);
  const ref = useRef<HTMLMenuElement>(null);
  const [visible, setVisible] = useState(false);

  useContextMenuPosition(event, ref, setX, setY, setVisible);
  useOutsideClick(ref, setX, setY, setVisible);

  return (
    <menu
      ref={ref}
      style={{
        position: "absolute",
        left: x,
        top: y,
      }}
      className={`
      bg-white p-2 rounded-lg shadow-lg
      flex flex-col gap-1
      border border-gray-200
      ${visible ? "visible" : "invisible"}
      z-10
      `}
    >
      <MenuItem
        icon="icon-[carbon--lightning]"
        text="Add Trigger Node"
        onClick={onAddTriggerNode}
      />
      <MenuItem
        icon="icon-[carbon--play]"
        text="Add Action Node"
        onClick={onAddActionNode}
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
  onClick: (event: React.MouseEvent<HTMLLIElement>) => void;
}) => {
  return (
    <li
      className="
      flex items-center gap-2 w-44 p-2 h-8 rounded-md
      hover:bg-purple-100 hover:text-gray-700 hover:cursor-pointer
      active:bg-purple-200 active:text-gray-800 active:scale-[0.98]
      group/menu-item
      "
      onClick={(event: React.MouseEvent<HTMLLIElement>) => {
        onClick(event);
      }}
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
