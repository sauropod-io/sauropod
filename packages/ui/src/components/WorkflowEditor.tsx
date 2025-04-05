import { useQuery } from "@tanstack/react-query";
import {
  Background,
  type Connection,
  Controls,
  type Edge,
  type Node,
  ReactFlow,
  ReactFlowProvider,
  addEdge,
  useEdgesState,
  useNodesState,
} from "@xyflow/react";
import "@xyflow/react/dist/base.css";
import { Play, Plus, Save, Trash2 } from "lucide-react";
import { useCallback, useEffect, useState } from "react";
import { useNavigate } from "react-router";

import { Schemas } from "@sauropod-io/client";

import { apiClient } from "@/api";
import { InvocationModal } from "@/components/InvocationModal";
import TaskSelector from "@/components/TaskSelector";
import { WorkflowConfigSheet } from "@/components/WorkflowConfigSheet";
import CustomEdge from "@/components/nodes/CustomEdge";
import { IONodeData } from "@/components/nodes/IONode";
import InputNode from "@/components/nodes/InputNode";
import OutputNode from "@/components/nodes/OutputNode";
import TaskNode, { type TaskNodeData } from "@/components/nodes/TaskNode";
import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
  AlertDialogTrigger,
} from "@/components/ui/alert-dialog";
import { Button, buttonVariants } from "@/components/ui/button";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { Input } from "@/components/ui/input";
import {
  GraphNode,
  INPUT_NODE_ID,
  INPUT_NODE_TYPE,
  OUTPUT_NODE_ID,
  OUTPUT_NODE_TYPE,
  TASK_NODE_TYPE,
  graphToWorkflow,
  workflowToGraph,
} from "@/lib/graphUtils";
import {
  useCreateWorkflow,
  useDeleteWorkflow,
  useUpdateWorkflow,
} from "@/mutations/workflowMutations";
import { WORKFLOW_PREFIX, workflowRoute } from "@/routes";

const nodeTypes = {
  [INPUT_NODE_TYPE]: InputNode,
  [TASK_NODE_TYPE]: TaskNode,
  [OUTPUT_NODE_TYPE]: OutputNode,
};

const edgeTypes = {
  edge: CustomEdge,
};

interface FlowProps {
  workflowId?: string;
  workflowData: Schemas["Workflow"];
}

function Flow({ workflowData, workflowId }: FlowProps) {
  const createWorkflow = useCreateWorkflow();
  const updateWorkflow = useUpdateWorkflow();
  const deleteWorkflow = useDeleteWorkflow();
  const navigate = useNavigate();

  const [name, setName] = useState(workflowData?.name || "");
  const [nodes, setNodes, onNodesChange] = useNodesState<GraphNode>([]);
  const [edges, setEdges, onEdgesChange] = useEdgesState<Edge>([]);
  const [isRunModalOpen, setIsRunModalOpen] = useState(false);

  useEffect(() => {
    const { nodes: graphNodes, edges: graphEdges } =
      workflowToGraph(workflowData);
    setNodes(graphNodes);
    setEdges(graphEdges);
  }, [workflowData, setNodes, setEdges]);

  const [inputs, setInputs] = useState<string[]>([]);
  const [outputs, setOutputs] = useState<string[]>(["output"]);
  const [isDeleteAlertOpen, setIsDeleteAlertOpen] = useState(false);
  const [isSettingsSheetOpen, setIsSettingsSheetOpen] = useState(false);

  const onConnect = useCallback(
    (params: Connection | Edge) => setEdges((eds) => addEdge(params, eds)),
    [setEdges],
  );

  useEffect(() => {
    const newNode: Node<IONodeData> = {
      id: INPUT_NODE_ID,
      type: INPUT_NODE_TYPE,
      position: { x: -100, y: 100 },
      data: { names: inputs },
    };
    setNodes((nodes) => [
      ...nodes.filter((x) => x.id != INPUT_NODE_ID),
      newNode,
    ]);
  }, [inputs, setNodes]);

  useEffect(() => {
    const newNode: Node<IONodeData> = {
      id: OUTPUT_NODE_ID,
      type: OUTPUT_NODE_TYPE,
      position: { x: 100, y: 100 },
      data: { names: outputs },
    };
    setNodes((nodes) => [
      ...nodes.filter((x) => x.id != OUTPUT_NODE_ID),
      newNode,
    ]);
  }, [outputs, setNodes]);

  const handleSave = async () => {
    const workflow = graphToWorkflow(name, nodes, edges);
    if (workflowId !== undefined) {
      await updateWorkflow.mutateAsync({
        params: {
          path: {
            id: workflowId,
          },
        },
        body: workflow,
      });
    } else {
      let newWorkflowId: number;
      try {
        newWorkflowId = await createWorkflow.mutateAsync({
          body: workflow,
        });
      } catch (error) {
        alert(`Error creating workflow: ${error}`);
        return;
      }
      await navigate(workflowRoute(newWorkflowId));
    }
  };

  const handleDelete = async () => {
    if (workflowId === undefined) return;

    deleteWorkflow.mutate({
      params: { path: { id: workflowId! } },
    });

    // Navigate back to the workflows page
    await navigate(WORKFLOW_PREFIX);
  };

  const handleAddTask = (taskId: number, taskName: string) => {
    const newNode: Node<TaskNodeData> = {
      id: `${taskId}-${Date.now()}`,
      type: TASK_NODE_TYPE,
      position: { x: 250, y: 250 },
      data: { taskId, taskName },
    };
    setNodes((nodes) => [...nodes, newNode]);
  };

  const handleAddInput = (inputName: string) => {
    if (!inputName.trim() || inputs.includes(inputName)) return;
    setInputs([...inputs, inputName]);
  };

  const handleRemoveInput = (inputId: string) => {
    setInputs(inputs.filter((input) => input !== inputId));
    setNodes(nodes.filter((node) => node.id !== inputId));
  };

  const handleAddOutput = (outputName: string) => {
    if (!outputName.trim() || outputs.includes(outputName)) return;
    setOutputs([...outputs, outputName]);
  };

  const handleRemoveOutput = (outputId: string) => {
    setOutputs(outputs.filter((output) => output !== outputId));
    setNodes(nodes.filter((node) => node.id !== outputId));
  };

  const handleRunClick = () => {
    if (workflowId === undefined) {
      alert("Please save the workflow first");
      return;
    }
    setIsRunModalOpen(true);
  };

  return (
    <div className="flex flex-col h-full">
      <div className="flex flex-col md:flex-row items-start md:items-center justify-between p-4">
        <Input
          value={name}
          onChange={(e) => setName(e.target.value)}
          placeholder="Workflow name"
          className="text-xl font-bold h-10 w-full md:w-auto flex-grow mb-2 md:mb-0"
        />
        <div className="flex flex-wrap gap-2 w-full pl-0 md:pl-4 md:w-auto justify-end">
          <Button
            onClick={handleRunClick}
            size="sm"
            variant="default"
            disabled={workflowId === undefined}
          >
            <Play className="h-4 w-4" />
            <span className="hidden md:inline">Run</span>
          </Button>

          <DropdownMenu>
            <DropdownMenuTrigger
              className={buttonVariants({ size: "sm", variant: "outline" })}
            >
              <Plus className="h-4 w-4" />
              <span className="hidden md:inline">Task</span>
            </DropdownMenuTrigger>
            <DropdownMenuContent className="border-0">
              <TaskSelector autoFocus={true} onSelect={handleAddTask} />
            </DropdownMenuContent>
          </DropdownMenu>

          <Button onClick={handleSave} size="sm" variant="outline">
            <Save className="h-4 w-4" />
          </Button>

          <AlertDialog
            open={isDeleteAlertOpen}
            onOpenChange={setIsDeleteAlertOpen}
          >
            <AlertDialogTrigger asChild>
              <Button
                size="sm"
                variant={workflowId !== undefined ? "destructive" : "ghost"}
                disabled={workflowId === undefined}
                onClick={() => setIsDeleteAlertOpen(true)}
              >
                <Trash2 className="h-4 w-4" />
              </Button>
            </AlertDialogTrigger>
            <AlertDialogContent>
              <AlertDialogHeader>
                <AlertDialogTitle>Are you sure?</AlertDialogTitle>
                <AlertDialogDescription>
                  Are you sure you want to delete{name ? ` ${name}` : ""}?
                </AlertDialogDescription>
              </AlertDialogHeader>
              <AlertDialogFooter>
                <AlertDialogCancel>Cancel</AlertDialogCancel>
                <AlertDialogAction onClick={handleDelete}>
                  Delete
                </AlertDialogAction>
              </AlertDialogFooter>
            </AlertDialogContent>
          </AlertDialog>

          <WorkflowConfigSheet
            open={isSettingsSheetOpen}
            onOpenChange={setIsSettingsSheetOpen}
            inputs={inputs}
            onAddInput={handleAddInput}
            onRemoveInput={handleRemoveInput}
            outputs={outputs}
            onAddOutput={handleAddOutput}
            onRemoveOutput={handleRemoveOutput}
            onAddTask={handleAddTask}
          />
        </div>
      </div>

      <ReactFlow
        edges={edges}
        nodes={nodes}
        nodeTypes={nodeTypes}
        edgeTypes={edgeTypes}
        defaultEdgeOptions={{
          type: "edge",
          animated: true,
        }}
        onConnect={onConnect}
        onEdgesChange={onEdgesChange}
        onNodesChange={onNodesChange}
        fitView
      >
        <Controls />
        <Background gap={12} size={1} />
      </ReactFlow>

      <InvocationModal
        workflowId={workflowId}
        workflowName={name}
        open={isRunModalOpen}
        onOpenChange={setIsRunModalOpen}
      />
    </div>
  );
}

export default function WorkflowEditor({
  workflowId,
}: {
  workflowId?: string;
}) {
  const { data, isLoading, error } = useQuery({
    queryKey: ["get", `/api/task/${workflowId}`],
    queryFn: async () => {
      if (!workflowId)
        return {
          name: "",
          actions: {},
          connections: [],
        } as Schemas["Workflow"];
      const response = await apiClient.GET("/api/workflow/{id}", {
        params: { path: { id: workflowId } },
      });
      return response.data;
    },
  });

  if (error) {
    return <div className="p-4">Error: {error.message}</div>;
  }

  if (isLoading || data === undefined) {
    return <div className="p-4">Loading...</div>;
  }

  return (
    <ReactFlowProvider>
      <Flow workflowId={workflowId} workflowData={data} />
    </ReactFlowProvider>
  );
}
