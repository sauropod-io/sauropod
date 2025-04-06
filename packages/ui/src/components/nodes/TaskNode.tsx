import {
  type Node,
  type NodeProps,
  Position,
  useUpdateNodeInternals,
} from "@xyflow/react";
import { PencilRuler } from "lucide-react";
import { useEffect } from "react";

import api from "@/api";
import Spinner from "@/components/icons/Spinner";
import { BaseNode } from "@/components/nodes/BaseNode";
import { LabeledHandle } from "@/components/nodes/LabeledHandle";
import {
  NodeHeader,
  NodeHeaderActions,
  NodeHeaderDeleteAction,
  NodeHeaderIcon,
  NodeHeaderTitle,
} from "@/components/nodes/NodeHeader";

export type TaskNodeData = {
  taskId: number;
  taskName: string;
};

export default function TaskNode({
  selected,
  data: { taskId, taskName },
  id,
}: NodeProps<Node<TaskNodeData>>) {
  const updateNodeInternals = useUpdateNodeInternals();
  const { data, isLoading } = api.useQuery(
    "get",
    `/api/task/{id}/inputSchema`,
    {
      params: { path: { id: `${taskId}` } },
    },
  );

  useEffect(() => {
    updateNodeInternals(id);
  }, [id, updateNodeInternals, data]);

  const inputs = [];
  if (data) {
    for (const input in data["properties"] as string[]) {
      inputs.push(
        <LabeledHandle
          key={input}
          title={input}
          type="target"
          position={Position.Left}
          id={input}
        />,
      );
    }
  }

  const HeaderIcon = isLoading ? Spinner : PencilRuler;
  return (
    <BaseNode selected={selected}>
      <NodeHeader className="border-b">
        <NodeHeaderIcon>
          <HeaderIcon className="h-6 w-6" />
        </NodeHeaderIcon>
        <NodeHeaderTitle>{taskName}</NodeHeaderTitle>
        <NodeHeaderActions>
          <NodeHeaderDeleteAction />
        </NodeHeaderActions>
      </NodeHeader>
      <div className="flex gap flex-row">
        <div>{inputs}</div>
        <div className="flex-grow" />
        <LabeledHandle
          title="output"
          type="source"
          position={Position.Right}
          id="value"
        />
      </div>
    </BaseNode>
  );
}
