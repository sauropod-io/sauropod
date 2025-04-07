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
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from "@/components/ui/tooltip";
import { JsonSchemaBase, assertIsJsonSchemaObject } from "@/lib/jsonSchema";
import { NODE_X_SPACING } from "@/lib/workflowGraph";

export type TaskNodeData = {
  taskId: number;
};

export default function TaskNode({
  selected,
  data: { taskId },
  id,
}: NodeProps<Node<TaskNodeData>>) {
  const updateNodeInternals = useUpdateNodeInternals();
  const { data: schemaData, isLoading: schemaLoading } = api.useQuery(
    "get",
    `/api/task/{id}/schema`,
    {
      params: { path: { id: `${taskId}` } },
    },
  );
  const { data: taskData, isLoading: inputTaskLoading } = api.useQuery(
    "get",
    `/api/task/{id}`,
    {
      params: { path: { id: `${taskId}` } },
    },
  );

  const isLoading = schemaLoading || inputTaskLoading;

  useEffect(() => {
    updateNodeInternals(id);
  }, [id, updateNodeInternals, schemaData]);

  const inputs = [];
  const outputs = [];
  if (schemaData) {
    const inputSchema = assertIsJsonSchemaObject(
      schemaData.inputSchema as JsonSchemaBase,
    );
    for (const [input, inputType] of Object.entries(
      inputSchema["properties"],
    )) {
      inputs.push(
        <LabeledHandle
          key={input}
          title={input}
          type="target"
          position={Position.Left}
          description={inputType.description}
          id={input}
        />,
      );
    }

    const outputSchema = assertIsJsonSchemaObject(
      schemaData.outputSchema as JsonSchemaBase,
    );
    for (const [output, outputType] of Object.entries(
      outputSchema["properties"],
    )) {
      outputs.push(
        <LabeledHandle
          key={output}
          title={output}
          type="source"
          position={Position.Right}
          description={outputType.description}
          id={output}
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
        <NodeHeaderTitle
          className="truncate"
          style={{ maxWidth: NODE_X_SPACING - 100 }}
        >
          <TooltipProvider>
            <Tooltip>
              <TooltipTrigger asChild className="truncate">
                <div>{taskData?.name}</div>
              </TooltipTrigger>
              <TooltipContent>
                <p>{taskData?.name}</p>
              </TooltipContent>
            </Tooltip>
          </TooltipProvider>
        </NodeHeaderTitle>
        <NodeHeaderActions>
          <NodeHeaderDeleteAction />
        </NodeHeaderActions>
      </NodeHeader>
      <div className="flex gap flex-row">
        <div>{inputs}</div>
        <div className="flex-grow" />
        <div>{outputs}</div>
      </div>
    </BaseNode>
  );
}
