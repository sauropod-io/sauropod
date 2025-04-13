import { useQuery } from "@tanstack/react-query";
import { Plus, Trash2 } from "lucide-react";
import { JSX, useEffect, useState } from "react";
import { useNavigate, useSearchParams } from "react-router";

import { type Schemas } from "@sauropod-io/client";

import { apiClient } from "@/api";
import { EditorHeader } from "@/components/EditorHeader";
import ErrorCard from "@/components/ErrorCard";
import PromptEditor from "@/components/PromptEditor";
import { TaskRunModal } from "@/components/RunModal";
import ToolSelector from "@/components/ToolSelector";
import { Button } from "@/components/ui/button";
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
import { TASK_PREFIX, taskRoute } from "@/routes";

import IconButton from "./buttons/IconButton";

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
        <IconButton
          variant="outline"
          disabled={disabled}
          onClick={addOutput}
          className="w-[100px]"
          Icon={Plus}
          text="Add field"
        />
      </div>
    </div>
  );
}

export default function TaskEditor({ taskId }: { taskId?: string }) {
  const navigate = useNavigate();
  const deleteTask = useDeleteTask();
  const createTask = useCreateTask();
  const updateTask = useUpdateTask();
  const [searchParams, setSearchParams] = useSearchParams();

  const [formState, setFormState] = useState<{
    name: string;
    tools: string[];
  }>({
    name: "",
    tools: [],
  });
  const [promptInitialValue, setPromptInitialValue] = useState<string | null>(
    taskId === undefined ? "" : null,
  );
  const [promptText, setPromptText] = useState<string>("");
  const [outputSchema, setOutputSchema] = useState<JsonSchemaObject | null>(
    null,
  );
  const [outputAll, setOutputAll] = useState(true);
  const [isRunModalOpen, setIsRunModalOpen] = useState(false);

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
        tools: taskData.action?.invokeLLM?.availableToolIds || [],
      });

      if (taskData.action?.invokeLLM?.template) {
        setPromptText(taskData.action.invokeLLM.template);
        setPromptInitialValue(taskData.action.invokeLLM.template);
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

  // Open the run modal automatically if the URL contains the "run" parameter
  useEffect(() => {
    if (searchParams.has("run") && taskId !== undefined) {
      setIsRunModalOpen(true);
      // Remove the parameter from the URL to prevent reopening on refresh
      searchParams.delete("run");
      setSearchParams(searchParams);
    }
  }, [searchParams, setSearchParams, taskId]);

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
          template: promptText!,
          outputSchema:
            outputAll || Object.keys(outputSchema?.properties ?? {}).length == 0
              ? null
              : outputSchema,
          availableToolIds: formState.tools,
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
    if (taskId === undefined) {
      alert("Please save the task first");
      return;
    }

    // Make sure the task definition is up to date on the server
    await saveTask();

    setIsRunModalOpen(true);
  };

  const handleToolToggle = (toolId: string) => {
    setFormState((prevFormState) => ({
      ...prevFormState,
      tools: prevFormState.tools.includes(toolId)
        ? prevFormState.tools.filter((id) => id !== toolId)
        : [...prevFormState.tools, toolId],
    }));
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
    <div className="flex flex-col flex-grow">
      <EditorHeader
        name={formState.name || ""}
        onNameChange={(newName) =>
          setFormState({ ...formState, name: newName })
        }
        onRun={handleRun}
        onSave={saveTask}
        onDelete={handleDelete}
        disabled={taskId === undefined}
      />

      <div className="p-4 bg-white flex flex-col flex-grow">
        <div className="flex-grow max-h-[50%]">
          {promptInitialValue !== null && (
            <PromptEditor
              onChange={(prompt) => setPromptText(prompt!)}
              initialValue={promptInitialValue!}
            />
          )}
        </div>

        <div className="grid grid-cols-1 lg:grid-cols-2 gap-4 mt-4">
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

          <div>
            <Label className="text-base">Available Tools</Label>
            <ToolSelector
              selectedTools={formState.tools}
              onToolSelected={handleToolToggle}
            />
          </div>
        </div>
      </div>
      {taskId !== undefined && (
        <TaskRunModal
          taskId={taskId}
          name={formState.name}
          open={isRunModalOpen}
          onOpenChange={setIsRunModalOpen}
        />
      )}
    </div>
  );
}
