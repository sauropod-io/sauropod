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
import "@xyflow/react/dist/style.css";
import { Save, Trash2 } from "lucide-react";
import { useCallback, useState } from "react";
import { useNavigate } from "react-router";

import { Schemas } from "@sauropod-io/client";

import { apiClient } from "@/api";
import { WorkflowConfigSheet } from "@/components/WorkflowConfigSheet";
import InputNode, { type InputNodeData } from "@/components/nodes/InputNode";
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
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import {
  GraphNode,
  INPUT_NODE_TYPE,
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

interface FlowProps {
  workflowId?: string;
  workflowData: Schemas["Workflow"];
}

function Flow({ workflowData, workflowId }: FlowProps) {
  const { nodes: originalNodes, edges: originalEdges } =
    workflowToGraph(workflowData);

  const createWorkflow = useCreateWorkflow();
  const updateWorkflow = useUpdateWorkflow();
  const deleteWorkflow = useDeleteWorkflow();
  const navigate = useNavigate();

  const [name, setName] = useState(workflowData?.name);
  const [nodes, setNodes, onNodesChange] =
    useNodesState<GraphNode>(originalNodes);
  const [edges, setEdges, onEdgesChange] = useEdgesState<Edge>(originalEdges);
  const [inputs, setInputs] = useState<{ id: string; name: string }[]>([]);
  const [isDeleteAlertOpen, setIsDeleteAlertOpen] = useState(false);
  const [isTasksSheetOpen, setIsTasksSheetOpen] = useState(false);
  const [selectedTasks, setSelectedTasks] = useState<
    { id: number; name: string }[]
  >([]);

  const onConnect = useCallback(
    (params: Connection | Edge) => setEdges((eds) => addEdge(params, eds)),
    [setEdges],
  );

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
    if (!selectedTasks.some((task) => task.id === taskId)) {
      setSelectedTasks([...selectedTasks, { id: taskId, name: taskName }]);

      const newNode: Node<TaskNodeData> = {
        id: `${taskId}-${Date.now()}`,
        type: TASK_NODE_TYPE,
        position: { x: 250, y: 250 },
        data: { taskId, taskName },
      };
      setNodes((nodes) => [...nodes, newNode]);
    }
  };

  const handleAddInput = (inputName: string) => {
    if (!inputName.trim()) return;
    const inputId = `input-${Date.now()}`;
    // Don't allow duplicatei inputs
    if (inputs.some((input) => input.name === inputName)) return;

    setInputs([...inputs, { id: inputId, name: inputName }]);
    const newNode: Node<InputNodeData> = {
      id: inputId,
      type: INPUT_NODE_TYPE,
      position: { x: 100, y: 100 },
      data: { name: inputName },
    };
    setNodes((nodes) => [...nodes, newNode]);
  };

  const handleRemoveInput = (inputId: string) => {
    setInputs(inputs.filter((input) => input.id !== inputId));
    setNodes(nodes.filter((node) => node.id !== inputId));
  };

  const handleRemoveTask = (taskId: number) => {
    setSelectedTasks(selectedTasks.filter((task) => task.id !== taskId));
    setNodes(nodes.filter((node) => !node.id.startsWith(`${taskId}-`)));
  };

  return (
    <div className="flex flex-col h-full">
      <div className="flex items-center justify-between p-4">
        <Input
          value={name}
          onChange={(e) => setName(e.target.value)}
          placeholder="Workflow name"
          className="text-xl font-bold h-10"
        />
        <div className="flex gap-2 pl-2">
          <Button onClick={handleSave} size="sm" variant="outline">
            <Save className="mr-2 h-4 w-4" />
            Save
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
                <Trash2 className="mr-2 h-4 w-4" />
                Delete
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
            open={isTasksSheetOpen}
            onOpenChange={setIsTasksSheetOpen}
            inputs={inputs}
            onAddInput={handleAddInput}
            onRemoveInput={handleRemoveInput}
            selectedTasks={selectedTasks}
            onAddTask={handleAddTask}
            onRemoveTask={handleRemoveTask}
          />
        </div>
      </div>

      <ReactFlow
        edges={edges}
        nodes={nodes}
        nodeTypes={nodeTypes}
        onConnect={onConnect}
        onEdgesChange={onEdgesChange}
        onNodesChange={onNodesChange}
        fitView
      >
        <Controls />
        <Background gap={12} size={1} />
      </ReactFlow>
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
        } as Schemas["Workflow"];
      const response = await apiClient.GET("/api/workflow/{id}", {
        params: { path: { id: workflowId } },
      });
      return response.data;
    },
  });

  if (isLoading) {
    return <div className="p-4">Loading...</div>;
  }
  if (error) {
    return <div className="p-4">Error: {error.message}</div>;
  }

  return (
    <ReactFlowProvider>
      <Flow workflowId={workflowId} workflowData={data!} />
    </ReactFlowProvider>
  );
}
