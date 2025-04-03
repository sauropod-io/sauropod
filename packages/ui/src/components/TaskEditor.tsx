import { useQuery } from "@tanstack/react-query";
import { Trash2 } from "lucide-react";
import { useEffect, useState } from "react";
import { useNavigate } from "react-router";

import { type Schemas } from "@sauropod-io/client";

import { apiClient } from "@/api";
import PromptEditor from "@/components/PromptEditor";
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
import {
  Card,
  CardContent,
  CardFooter,
  CardHeader,
} from "@/components/ui/card";
import { Checkbox } from "@/components/ui/checkbox";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import {
  useCreateTask,
  useDeleteTask,
  useUpdateTask,
} from "@/mutations/taskMutations";
import { ToolConsumer } from "@/providers/ToolsProvider";
import { TASK_PREFIX, taskRoute } from "@/routes";

import ErrorCard from "./ErrorCard";

type ModelSize = "weak" | "strong";
const MODEL_SIZES: ModelSize[] = ["weak", "strong"];

export default function TaskEditor({ taskId }: { taskId?: string }) {
  const navigate = useNavigate();
  const deleteTask = useDeleteTask();
  const createTask = useCreateTask();
  const updateTask = useUpdateTask();

  const [formState, setFormState] = useState<{
    name: string;
    model: ModelSize;
    tools: string[];
  }>({
    name: "",
    model: MODEL_SIZES[0],
    tools: [],
  });
  const [promptText, setPromptText] = useState<string>("");

  // Fetch task data when editing an existing task
  const {
    data: taskData,
    isLoading,
    error: taskDataError,
  } = useQuery({
    queryKey: ["get", `/api/task/${taskId}`],
    queryFn: async () => {
      if (!taskId)
        return {
          name: "",
          action: {
            invokeLLM: {
              modelStrength: MODEL_SIZES[1],
              template: "",
            },
          },
        } as Schemas["Task"];
      const response = await apiClient.GET("/api/task/{id}", {
        params: { path: { id: taskId } },
      });
      return response.data;
    },
  });

  // Update form state when task data is loaded
  useEffect(() => {
    if (taskData) {
      setFormState({
        name: taskData.name,
        model:
          (taskData.action?.invokeLLM?.modelStrength as ModelSize) || "strong",
        tools: [],
      });

      if (taskData.action?.invokeLLM?.template) {
        setPromptText(taskData.action.invokeLLM.template);
      }
    }
  }, [taskData]);

  const handleDelete = async () => {
    // Delete the task
    deleteTask.mutate({
      params: { path: { id: taskId! } },
    });

    // Navigate back to the tasks page
    await navigate(TASK_PREFIX);
  };

  const saveTask = async () => {
    const taskData: Schemas["Task"] = {
      name: formState.name!,
      action: {
        invokeLLM: {
          modelStrength: formState.model,
          template: promptText,
        },
      },
    };

    if (taskId) {
      await updateTask.mutateAsync({
        params: {
          path: {
            id: taskId,
          },
        },
        body: taskData,
      });
    } else {
      let newTaskId: number;
      try {
        newTaskId = await createTask.mutateAsync({
          body: taskData,
        });
      } catch (error) {
        alert(`Error creating task: ${error}`);
        return;
      }

      await navigate(taskRoute(newTaskId));
    }
  };

  if (taskDataError) {
    return (
      <ErrorCard message="Error loading task data" error={taskDataError} />
    );
  }

  if (isLoading && taskId) {
    return <div className="p-4">Loading task data...</div>;
  }

  return (
    <div className="p-4">
      <Card>
        <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
          <Input
            value={formState.name || ""}
            onChange={(e) =>
              setFormState({ ...formState, name: e.target.value })
            }
            placeholder="Task name"
            className="text-xl font-bold h-10"
          />
          <Select
            value={formState.model}
            onValueChange={(value) =>
              setFormState({ ...formState, model: value as ModelSize })
            }
          >
            <SelectTrigger className="w-[180px]">
              <SelectValue placeholder="Select model" />
            </SelectTrigger>
            <SelectContent>
              {MODEL_SIZES.map((model) => (
                <SelectItem key={model} value={model}>
                  {`${model.substring(0, 1).toUpperCase()}${model.substring(1).toLowerCase()}`}
                </SelectItem>
              ))}
            </SelectContent>
          </Select>
        </CardHeader>
        <CardContent>
          <PromptEditor
            onChange={(prompt) => setPromptText(prompt!)}
            onValidate={(markers) => console.log("onValidate", markers)}
            value={promptText}
          />
          <div className="mt-4">
            <Label className="text-base">Available Tools</Label>
            <div className="grid grid-cols-2 gap-4 mt-2">
              <ToolConsumer>
                {(tools) =>
                  tools.map((tool) => (
                    <div
                      key={tool.name}
                      className="flex items-center space-x-2"
                    >
                      <Checkbox
                        id={`tool-${tool.name}`}
                        checked={formState.tools?.includes(tool.name)}
                      />
                      <Label htmlFor={`tool-${tool.name}`}>{tool.name}</Label>
                    </div>
                  ))
                }
              </ToolConsumer>
            </div>
          </div>
        </CardContent>
        <CardFooter className="flex justify-end space-x-2">
          {taskId && (
            <AlertDialog>
              <AlertDialogTrigger asChild>
                <Button variant="destructive">
                  <Trash2 className="mr-2 h-4 w-4" />
                  Delete
                </Button>
              </AlertDialogTrigger>
              <AlertDialogContent>
                <AlertDialogHeader>
                  <AlertDialogTitle>Are you sure?</AlertDialogTitle>
                  <AlertDialogDescription>
                    This action cannot be undone. This will permanently delete
                    the task.
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
          )}
          <Button onClick={saveTask}>Save</Button>
        </CardFooter>
      </Card>
    </div>
  );
}
