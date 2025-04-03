import { type Node, type NodeProps } from "@xyflow/react";
import { memo } from "react";

import IONode, { type IONodeData } from "@/components/nodes/IONode";

/** Node used for workflow outputs. */
const OutputNode = memo((props: NodeProps<Node<IONodeData>>) => {
  return <IONode title="Outputs" handleType="source" {...props} />;
});

export default OutputNode;
