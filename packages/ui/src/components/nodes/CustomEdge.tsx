import { BaseEdge, type EdgeProps, getBezierPath } from "@xyflow/react";

export default function CustomEdge({
  sourcePosition,
  sourceX,
  sourceY,
  targetPosition,
  targetX,
  targetY,
  ...props
}: EdgeProps) {
  const edgePathParams = {
    sourcePosition,
    sourceX,
    sourceY,
    targetPosition,
    targetX,
    targetY,
  };
  return <BaseEdge path={getBezierPath(edgePathParams)[0]} {...props} />;
}
