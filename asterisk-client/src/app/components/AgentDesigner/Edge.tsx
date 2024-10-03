import { EdgeLabelRenderer, EdgeProps, getBezierPath } from "@xyflow/react";
import colors from "tailwindcss/colors";

//--------------------------------------------------------------------------------------------------
// Component
//--------------------------------------------------------------------------------------------------

const ConnectedEdge = ({
  id,
  sourceX,
  sourceY,
  targetX,
  targetY,
}: EdgeProps) => {
  const adjustedTargetY = targetY - 20;
  const [edgePath, labelX, labelY] = getBezierPath({
    sourceX,
    sourceY,
    targetX,
    targetY: adjustedTargetY,
  });

  return (
    <>
      <svg className="group">
        <g key={id}>
          {/* Edge Path */}
          <path
            d={edgePath}
            stroke={colors.gray[300]}
            strokeWidth={2}
            fill="none"
            className="group-hover:stroke-gray-400"
            pointerEvents="all"
          />

          {/* Edge Arrow Head */}
          <path
            d={`M${targetX - 7},${adjustedTargetY} h14 l-7,12 Z`}
            fill={colors.gray[300]}
            className="group-hover:fill-gray-400"
            pointerEvents="all"
          />

          {/* Dummy Element to Extend Edge Hover Zone */}
          <path
            d={edgePath}
            stroke="transparent"
            fill="none"
            strokeWidth={40}
            pointerEvents="all"
          />
        </g>
      </svg>

      <EdgeLabelRenderer>
        <button
          style={{
            position: "absolute",
            left: labelX,
            top: labelY,
            transform: "translate(-50%, -50%)",
            pointerEvents: "all",
          }}
        />
      </EdgeLabelRenderer>
    </>
  );
};

const UnconnectedEdge = ({ id, sourceX, sourceY }: EdgeProps) => {
  const pathHeight = 80;
  const labelSize = 25;

  return (
    <>
      <svg className="group">
        <g key={id}>
          {/* Edge Path */}
          <path
            d={`M${sourceX},${sourceY} v${pathHeight}`}
            stroke={colors.gray[300]}
            strokeWidth={2}
            fill="none"
            strokeDasharray="5,5"
            className="group-hover:stroke-gray-400"
            pointerEvents="all"
          />

          {/* Dummy Element to Extend Edge Hover Zone */}
          <path
            d={`M${sourceX},${sourceY} v${pathHeight}`}
            stroke="transparent"
            fill="none"
            strokeWidth={40}
            pointerEvents="all"
          />
        </g>
      </svg>

      <EdgeLabelRenderer>
        <div
          style={{
            position: "absolute",
            top: sourceY + pathHeight,
            left: sourceX - labelSize / 2,
            height: labelSize,
            width: labelSize,
            pointerEvents: "all",
          }}
          className={`
            group/label rounded-md bg-white
            flex items-center justify-center
            border-2 border-gray-400
            hover:border-gray-500 hover:shadow-md active:scale-90
          `}
          onClick={(event) => {
            event.preventDefault();
            console.log("Edge clicked");
          }}
        >
          <span className="icon-[carbon--add] text-gray-400 size-5 group-hover/label:text-gray-500" />
        </div>
      </EdgeLabelRenderer>
    </>
  );
};

//--------------------------------------------------------------------------------------------------
// Exports
//--------------------------------------------------------------------------------------------------

export { ConnectedEdge, UnconnectedEdge };
