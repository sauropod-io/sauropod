import { memo } from "react";
import { Position, type NodeProps, type Node } from "@xyflow/react";
import { Parentheses } from "lucide-react";

import { BaseNode } from "@/components/nodes/BaseNode";
import {
  NodeHeader,
  NodeHeaderTitle,
  NodeHeaderActions,
  NodeHeaderIcon,
  NodeHeaderDeleteAction,
} from "@/components/nodes/NodeHeader";
import { LabeledHandle } from "./LabeledHandle";

export type OutputNodeData = {
  name: string;
};

/** Node used for workflow inputs. */
const OutputNode = memo(
  ({ data, selected }: NodeProps<Node<OutputNodeData>>) => {
    return (
      <BaseNode selected={selected} className="px-3 py-2">
        <NodeHeader className="-mx-3 -mt-2 border-b">
          <NodeHeaderIcon>
            <Parentheses className="h-6 w-6" />
          </NodeHeaderIcon>
          <NodeHeaderTitle>{data.name}</NodeHeaderTitle>
          <NodeHeaderActions>
            <NodeHeaderDeleteAction />
          </NodeHeaderActions>
        </NodeHeader>
        <LabeledHandle
          title="Workflow Output"
          type="target"
          position={Position.Right}
          id="output"
        />
      </BaseNode>
    );
  },
);

export default OutputNode;
