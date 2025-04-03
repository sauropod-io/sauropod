import { type Node, type NodeProps, Position } from "@xyflow/react";
import { PencilRuler } from "lucide-react";
import { memo } from "react";

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

const TaskNode = memo(
  ({ selected, data: { taskId, taskName } }: NodeProps<Node<TaskNodeData>>) => {
    const { data, isLoading } = api.useQuery(
      "get",
      `/api/task/{id}/inputSchema`,
      {
        params: { path: { id: `${taskId}` } },
      },
    );

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
      <BaseNode selected={selected} className="px-3 py-2">
        <NodeHeader className="-mx-3 -mt-2 border-b">
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
  },
);

export default TaskNode;
