import { Separator } from "../ui/separator";
import {
  type Node,
  type NodeProps,
  Position,
  useUpdateNodeInternals,
} from "@xyflow/react";
import { Parentheses } from "lucide-react";
import { useEffect } from "react";

import { BaseNode } from "@/components/nodes/BaseNode";
import { LabeledHandle } from "@/components/nodes/LabeledHandle";
import {
  NodeHeader,
  NodeHeaderIcon,
  NodeHeaderTitle,
} from "@/components/nodes/NodeHeader";

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
export default function IONode({
  title,
  handleType: type,
  data,
  selected,
  id,
}: IONodeProps & NodeProps<Node<IONodeData>>) {
  const updateNodeInternals = useUpdateNodeInternals();

  useEffect(() => {
    updateNodeInternals(id);
  }, [id, updateNodeInternals, data.names]);

  const elements = [];
  for (let i = 0; i < data.names.length; i++) {
    if (i > 0) {
      elements.push(<Separator key={i} />);
    }
    elements.push(
      <LabeledHandle
        key={data.names[i]}
        title={data.names[i]}
        type={type}
        position={type == "source" ? Position.Right : Position.Left}
        id={data.names[i]}
      />,
    );
  }

  return (
    <BaseNode selected={selected}>
      <NodeHeader className="border-b">
        <NodeHeaderIcon>
          <Parentheses className="h-6 w-6" />
        </NodeHeaderIcon>
        <NodeHeaderTitle>{title}</NodeHeaderTitle>
      </NodeHeader>
      {elements}
    </BaseNode>
  );
}
