import { type Node, type NodeProps } from "@xyflow/react";
import { memo } from "react";

import IONode, { type IONodeData } from "@/components/nodes/IONode";

/** Node used for workflow inputs. */
const InputNode = memo((props: NodeProps<Node<IONodeData>>) => {
  return <IONode title="Inputs" handleType="source" {...props} />;
});

export default InputNode;
