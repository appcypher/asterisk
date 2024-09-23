import { useEffect, useRef, useState } from "react";

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

type ContextMenuProps = {
  event: React.MouseEvent<Element> | MouseEvent;
  items: ItemData[];
};

type ItemData = {
  icon: string;
  warn?: boolean;
  text: string;
  onClick: (event: React.MouseEvent<HTMLLIElement>) => void;
};

//--------------------------------------------------------------------------------------------------
// Hooks
//--------------------------------------------------------------------------------------------------

/**
 * useContextMenuPosition is a hook that updates the position of a context menu based on the event
 * and the dimensions of the menu.
 */
const useContextMenuPosition = (
  event: React.MouseEvent<Element> | MouseEvent,
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
 * useOutsideClickAndEscape is a hook that handles the closing of a context menu when the user
 * clicks outside of it or presses the escape key.
 */
const useOutsideClickAndEscape = (
  menuRef: React.RefObject<HTMLMenuElement>,
  setX: React.Dispatch<React.SetStateAction<number>>,
  setY: React.Dispatch<React.SetStateAction<number>>,
  setVisible: React.Dispatch<React.SetStateAction<boolean>>,
) => {
  useEffect(() => {
    const handleMouseDown = (event: MouseEvent) => {
      if (!menuRef.current?.contains(event.target as Node)) {
        setX(0);
        setY(0);
        setVisible(false);
      }
    };

    const handleEscape = (event: KeyboardEvent) => {
      if (event.key === "Escape") {
        setX(0);
        setY(0);
        setVisible(false);
      }
    };

    document.addEventListener("mousedown", handleMouseDown);
    document.addEventListener("keydown", handleEscape);

    return () => {
      document.removeEventListener("mousedown", handleMouseDown);
      document.removeEventListener("keydown", handleEscape);
    };
  }, [menuRef, setX, setY, setVisible]);
};

//--------------------------------------------------------------------------------------------------
// Component
//--------------------------------------------------------------------------------------------------

const ContextMenu = ({ event, items }: ContextMenuProps) => {
  const [x, setX] = useState(0);
  const [y, setY] = useState(0);
  const menuRef = useRef<HTMLMenuElement>(null);
  const [visible, setVisible] = useState(false);

  useContextMenuPosition(event, menuRef, setX, setY, setVisible);
  useOutsideClickAndEscape(menuRef, setX, setY, setVisible);

  return (
    <div
      className={`absolute left-0 top-0 size-full z-10 ${visible ? "visible" : "invisible"}`}
    >
      <menu
        ref={menuRef}
        style={{
          position: "absolute",
          left: x,
          top: y,
        }}
        className={`
        bg-white p-2 rounded-lg shadow-sm
        flex flex-col gap-1
        border border-gray-200
        `}
        onContextMenu={(e) => e.preventDefault()}
      >
        {items.map((item, index) => (
          <MenuItem
            key={index}
            icon={item.icon}
            warn={item.warn}
            text={item.text}
            onClick={item.onClick}
          />
        ))}
      </menu>
    </div>
  );
};

const MenuItem = ({ icon, warn, text, onClick }: ItemData) => {
  return (
    <li
      className={`
      group/menu-item
      flex items-center gap-2 w-44 p-2 h-8 rounded-md
      hover:cursor-pointer active:text-gray-800 active:scale-[0.98]
      ${
        warn
          ? "hover:bg-red-50 active:bg-red-100"
          : "hover:bg-purple-100 active:bg-purple-200"
      }
      `}
      onClick={onClick}
    >
      <span
        className={`
          h-4
          ${icon}
          ${
            warn
              ? "text-red-400 group-hover/menu-item:text-red-700"
              : "text-gray-400 group-hover/menu-item:text-black"
          }
        `}
      ></span>
      <p
        className={`
          text-sm select-none
          ${
            warn
              ? "text-red-600 group-hover/menu-item:text-red-700"
              : "text-gray-700 group-hover/menu-item:text-black"
          }`}
      >
        {text}
      </p>
    </li>
  );
};

//--------------------------------------------------------------------------------------------------
// Exports
//--------------------------------------------------------------------------------------------------

export default ContextMenu;
