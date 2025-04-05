import { BaseEdge, type EdgeProps, getSmoothStepPath } from "@xyflow/react";

export default function CustomEdge({
  sourcePosition,
  sourceX,
  sourceY,
  targetPosition,
  targetX,
  targetY,
  id,
}: EdgeProps) {
  const [edgePath] = getSmoothStepPath({
    sourcePosition,
    sourceX,
    sourceY,
    targetPosition,
    targetX,
    targetY,
  });

  return <BaseEdge id={id} path={edgePath} />;
}
