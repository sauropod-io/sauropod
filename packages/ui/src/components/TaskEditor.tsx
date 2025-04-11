import { useQuery } from "@tanstack/react-query";
import { Play, Plus, Save, Trash2 } from "lucide-react";
import { JSX, useEffect, useState } from "react";
import { useNavigate } from "react-router";

import { type Schemas } from "@sauropod-io/client";

import { apiClient } from "@/api";
import ErrorCard from "@/components/ErrorCard";
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
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { Switch } from "@/components/ui/switch";
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from "@/components/ui/tooltip";
import {
  FieldType,
  JsonSchemaObject,
  jsonSchemaObjectProperties,
} from "@/lib/jsonSchema";
import {
  useCreateTask,
  useDeleteTask,
  useUpdateTask,
} from "@/mutations/taskMutations";
import { ToolConsumer } from "@/providers/ToolsProvider";
import { TASK_PREFIX, taskRoute } from "@/routes";

type ModelSize = "weak" | "strong";
const MODEL_SIZES: ModelSize[] = ["weak", "strong"];

// Output type options
const OUTPUT_TYPES: FieldType[] = ["string", "number", "boolean"];

// Output field interface
type OutputField = JsonSchemaObject["properties"][string];

interface OutputConfigurationProps {
  outputSchema: JsonSchemaObject | null;
  onChange: (outputSchema: JsonSchemaObject) => void;
  disabled?: boolean;
}

/** Convert a field type to more friendly name. */
function fieldTypeToFriendly(type: FieldType): string {
  switch (type) {
    case "string":
      return "Text";
    case "number":
      return "Number";
    case "boolean":
      return "True/False";
    default:
      return type;
  }
}

export function OutputConfiguration({
  disabled,
  outputSchema,
  onChange,
}: OutputConfigurationProps) {
  if (!outputSchema) {
    outputSchema = {
      type: "object",
      properties: {},
      required: [],
      additionalProperties: false,
    };
  }

  const [newInputName, setNewInputName] = useState("");
  const schemaProperties = jsonSchemaObjectProperties(outputSchema);

  const copyOutputSchema = () => {
    return {
      ...outputSchema,
      properties: {
        ...outputSchema.properties,
      },
      required: [...(outputSchema.required || [])],
    };
  };
  const addOutput = () => {
    const inputName = newInputName.trim();
    if (!inputName || schemaProperties.some(([name]) => name === inputName)) {
      return;
    }

    const newField: OutputField = {
      type: "string",
    };
    const newSchema = copyOutputSchema();
    newSchema.properties[inputName] = newField;
    newSchema.required!.push(inputName);
    setNewInputName("");
    onChange(newSchema);
  };

  const removeOutput = (key: string) => {
    const newSchema = copyOutputSchema();
    delete newSchema.properties[key];
    newSchema.required = newSchema.required?.filter((name) => name !== key);
    onChange(newSchema);
  };

  const changeType = (key: string, type: FieldType) => {
    const newSchema = copyOutputSchema();
    newSchema.properties[key]["type"] = type;
    onChange(newSchema);
  };

  const outputFields: JSX.Element[] = [];
  for (const [key, value] of schemaProperties) {
    outputFields.push(
      <div key={key} className="flex items-center space-x-2">
        <Label className="w-[200px]">{key}</Label>
        <Select
          value={value.type}
          onValueChange={(value) => changeType(key, value as FieldType)}
        >
          <SelectTrigger className="w-[100px]">
            <SelectValue placeholder="Type" />
          </SelectTrigger>
          <SelectContent>
            {OUTPUT_TYPES.map((type) => (
              <SelectItem key={type} value={type}>
                {fieldTypeToFriendly(type)}
              </SelectItem>
            ))}
          </SelectContent>
        </Select>
        <Button variant="ghost" size="icon" onClick={() => removeOutput(key)}>
          <Trash2 className="h-4 w-4" />
        </Button>
      </div>,
    );
  }

  return (
    <div className="mt-4 space-y-4">
      {disabled || outputFields}

      <div className="flex items-center space-x-2">
        <Input
          value={newInputName}
          onChange={(e) => setNewInputName(e.target.value)}
          placeholder="New field name"
          className="flex-1 max-w-[200px]"
          disabled={disabled}
        />
        <Button
          variant="outline"
          size="sm"
          disabled={disabled}
          onClick={addOutput}
          className="w-[100px]"
        >
          <Plus className="mr-2 h-4 w-4" />
          Add Field
        </Button>
      </div>
    </div>
  );
}

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
  const [outputSchema, setOutputSchema] = useState<JsonSchemaObject | null>(
    null,
  );
  const [outputAll, setOutputAll] = useState(true);

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

      if (response.error) {
        throw new Error(response.error.error);
      }

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
      if (taskData.action?.invokeLLM?.outputSchema) {
        setOutputSchema(
          taskData.action.invokeLLM.outputSchema as JsonSchemaObject,
        );
        setOutputAll(false);
      } else {
        setOutputSchema(null);
        setOutputAll(true);
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
          outputSchema:
            outputAll || Object.keys(outputSchema?.properties ?? {}).length == 0
              ? null
              : outputSchema,
        },
      },
    };

    if (taskId) {
      try {
        await updateTask.mutateAsync({
          params: {
            path: {
              id: taskId,
            },
          },
          body: taskData,
        });
      } catch (error) {
        alert(`Error updating task: ${JSON.stringify(error, null, 2)}`);
        return;
      }
    } else {
      let newTaskId: number;
      try {
        newTaskId = await createTask.mutateAsync({
          body: taskData,
        });
      } catch (error) {
        alert(`Error creating task: ${JSON.stringify(error, null, 2)}`);
        return;
      }

      await navigate(taskRoute(newTaskId));
    }
  };

  const handleRun = async () => {
    // First save the task
    await saveTask();

    console.log("TODO");
  };

  if (taskDataError) {
    return (
      <ErrorCard
        message="Error loading task data"
        error={{ error: taskDataError.message }}
      />
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
            value={promptText}
          />

          <div className="grid grid-cols-1 lg:grid-cols-2 gap-4">
            {/* Output Configuration Section */}
            <div>
              <Label className="text-base">Output Configuration</Label>
              <div className="mt-2 flex items-center space-x-2">
                <Switch
                  id="output-all"
                  checked={outputAll}
                  onCheckedChange={(checked) => setOutputAll(!!checked)}
                />
                <TooltipProvider>
                  <Tooltip>
                    <TooltipTrigger asChild className="truncate">
                      <Label htmlFor="output-all">Output all content</Label>
                    </TooltipTrigger>
                    <TooltipContent>
                      <p>
                        Directly output what the LLM generates as the "output"
                        key.
                      </p>
                    </TooltipContent>
                  </Tooltip>
                </TooltipProvider>
              </div>
              <OutputConfiguration
                onChange={(schema) => {
                  setOutputSchema(schema);
                }}
                disabled={outputAll}
                outputSchema={outputSchema}
              />
            </div>

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
                        <Switch
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
          </div>
        </CardContent>
        <CardFooter className="flex justify-end space-x-2">
          <Button onClick={handleRun} variant="outline">
            <Play className="h-4 w-4" />
            Run
          </Button>
          {taskId && (
            <AlertDialog>
              <AlertDialogTrigger asChild>
                <Button variant="destructive">
                  <Trash2 className="h-4 w-4" />
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
          <Button onClick={saveTask}>
            <Save />
            Save
          </Button>
        </CardFooter>
      </Card>
    </div>
  );
}
