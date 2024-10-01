import { EdgeLabelRenderer, EdgeProps, getBezierPath } from "@xyflow/react";
import colors from "tailwindcss/colors";

//--------------------------------------------------------------------------------------------------
// Component
//--------------------------------------------------------------------------------------------------

const CustomEdge = ({ id, sourceX, sourceY, targetX, targetY }: EdgeProps) => {
  const adjustedTargetY = targetY - 20;
  const [edgePath, labelX, labelY] = getBezierPath({
    sourceX,
    sourceY,
    targetX,
    targetY: adjustedTargetY,
  });

  return (
    <>
      <svg>
        <g key={id} className="group/edge">
          {/* Edge Path */}
          <path
            d={edgePath}
            stroke={colors.gray[300]}
            strokeWidth={2}
            fill="none"
            className="group-hover/edge:stroke-purple-400"
            pointerEvents="all"
          />

          {/* Edge Arrow Head */}
          <path
            d={`M${targetX - 7},${adjustedTargetY} h14 l-7,12 Z`}
            fill={colors.gray[300]}
            className="group-hover/edge:fill-purple-400"
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
        >
          Delete
        </button>
      </EdgeLabelRenderer>
    </>
  );
};

//--------------------------------------------------------------------------------------------------
// Exports
//--------------------------------------------------------------------------------------------------

export default CustomEdge;
