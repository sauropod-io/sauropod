import { type Node, type NodeProps, Position } from "@xyflow/react";
import { Parentheses } from "lucide-react";
import { memo } from "react";

import { BaseNode } from "@/components/nodes/BaseNode";
import {
  NodeHeader,
  NodeHeaderActions,
  NodeHeaderDeleteAction,
  NodeHeaderIcon,
  NodeHeaderTitle,
} from "@/components/nodes/NodeHeader";

import { LabeledHandle } from "./LabeledHandle";

export type InputNodeData = {
  name: string;
};

/** Node used for workflow inputs. */
const InputNode = memo(({ data, selected }: NodeProps<Node<InputNodeData>>) => {
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
        title="Variable"
        type="source"
        position={Position.Right}
        id="value"
      />
    </BaseNode>
  );
});

export default InputNode;
