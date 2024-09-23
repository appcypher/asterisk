import { useReactFlow } from "@xyflow/react";

//--------------------------------------------------------------------------------------------------
// Component
//--------------------------------------------------------------------------------------------------

const Controls = () => {
  const { zoomIn, zoomOut, zoomTo, fitView } = useReactFlow();

  return (
    <div
      className="
    absolute bottom-5 left-5 flex z-10 border border-gray-200 rounded-md
    hover:border-gray-300 hover:shadow-sm
    "
    >
      <ControlButton icon="icon-[fluent--add-16-regular]" onClick={zoomIn} />
      <ControlButton
        icon="icon-[fluent--minimize-16-regular]"
        onClick={zoomOut}
      />
      <ControlButton
        icon="icon-[fluent--zoom-fit-16-regular]"
        onClick={() =>
          fitView({ padding: 0.2, duration: 500, minZoom: 0.05, maxZoom: 1 })
        }
      />
      <ControlButton
        icon="icon-[fluent--arrow-counterclockwise-16-regular]"
        onClick={() => zoomTo(1)}
      />
    </div>
  );
};

const ControlButton = ({
  icon,
  onClick,
}: {
  icon: string;
  onClick: () => void;
}) => {
  return (
    <button
      className="
      group/button
      size-10 flex items-center justify-center bg-white
      [&:not(:first-child)]:border-l [&:not(:first-child)]:border-gray-200
      hover:bg-purple-50 active:bg-purple-100
      first:rounded-l-md last:rounded-r-md
      "
      onClick={onClick}
    >
      <span
        className={`${icon} size-5 bg-gray-600 group-hover/button:bg-black group-active/button:scale-90`}
      />
    </button>
  );
};

//--------------------------------------------------------------------------------------------------
// Exports
//--------------------------------------------------------------------------------------------------

export default Controls;
