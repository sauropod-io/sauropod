import { type Node, type NodeProps, Position } from "@xyflow/react";
import { Parentheses } from "lucide-react";
import { memo } from "react";

import { BaseNode } from "@/components/nodes/BaseNode";
import {
  NodeHeader,
  NodeHeaderIcon,
  NodeHeaderTitle,
} from "@/components/nodes/NodeHeader";

import { LabeledHandle } from "./LabeledHandle";

export type IONodeProps = {
  /** The title of the node. */
  title: string;
  /**
   * The type of the handles.
   */
  handleType: "source" | "target";
};

export type IONodeData = {
  /**
   * A list of variable names that node exposes.
   */
  names: string[];
};

/** Base node used for workflow inputs and outputs. */
const IONode = memo(
  ({
    title,
    handleType: type,
    data,
    selected,
  }: IONodeProps & NodeProps<Node<IONodeData>>) => {
    return (
      <BaseNode selected={selected} className="px-3 py-2">
        <NodeHeader className="-mx-3 -mt-2 border-b">
          <NodeHeaderIcon>
            <Parentheses className="h-6 w-6" />
          </NodeHeaderIcon>
          <NodeHeaderTitle>{title}</NodeHeaderTitle>
        </NodeHeader>
        {data.names.map((x) => (
          <LabeledHandle
            key={x}
            title={x}
            type={type}
            position={type == "source" ? Position.Right : Position.Left}
            id={x}
          />
        ))}
      </BaseNode>
    );
  },
);

export default IONode;
