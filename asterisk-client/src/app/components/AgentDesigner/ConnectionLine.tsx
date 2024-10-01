import { ConnectionLineComponentProps, getBezierPath } from "@xyflow/react";
import colors from "tailwindcss/colors";

//--------------------------------------------------------------------------------------------------
// Component
//--------------------------------------------------------------------------------------------------

const ConnectionLine = ({
  fromX,
  fromY,
  toX,
  toY,
}: ConnectionLineComponentProps) => {
  const [d] = getBezierPath({
    sourceX: fromX,
    sourceY: fromY,
    targetX: toX,
    targetY: toY,
  });

  return (
    <svg>
      <g key={`(${fromX},${fromY})->(${toX},${toY})`}>
        <path
          d={d}
          stroke={colors.purple[400]}
          strokeWidth={2}
          fill="none"
          strokeDasharray="5,5"
        />
      </g>
    </svg>
  );
};

//--------------------------------------------------------------------------------------------------
// Exports
//--------------------------------------------------------------------------------------------------

export default ConnectionLine;
