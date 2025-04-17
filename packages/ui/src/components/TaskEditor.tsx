import { useQuery } from "@tanstack/react-query";
import { Parentheses, Plus, Settings2, Trash2 } from "lucide-react";
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
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
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
  FieldType,
  JsonSchemaBase,
  JsonSchemaObject,
  jsonSchemaObjectProperties,
} from "@/lib/jsonSchema";
import {
  useCreateTask,
  useDeleteTask,
  useUpdateTask,
} from "@/mutations/taskMutations";
import InputVariablesProvider from "@/providers/InputsProvider";
import { TASK_PREFIX, taskRoute } from "@/routes";

import IconButton from "./buttons/IconButton";

// Output type options
const VARIABLE_TYPES: FieldType[] = ["string", "number", "boolean"];

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

interface VariableProps {
  name: string;
  value: FieldType;
  onChangeType: (type: FieldType) => void;
  onRemove?: () => void;
}

function Variable({
  name,
  value,
  onRemove,
  onChangeType: changeType,
}: VariableProps) {
  return (
    <div className="flex items-center space-x-2">
      <Label className="w-[200px]">{name}</Label>
      <Select
        value={value}
        onValueChange={(value) => changeType(value as FieldType)}
      >
        <SelectTrigger className="w-[100px]">
          <SelectValue placeholder="Type" />
        </SelectTrigger>
        <SelectContent>
          {VARIABLE_TYPES.map((type) => (
            <SelectItem key={type} value={type}>
              {fieldTypeToFriendly(type)}
            </SelectItem>
          ))}
        </SelectContent>
      </Select>
      {onRemove && (
        <Button variant="ghost" size="icon" onClick={() => onRemove()}>
          <Trash2 className="h-4 w-4" />
        </Button>
      )}
    </div>
  );
}

function OutputConfiguration({
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

    const newField: JsonSchemaBase = {
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
      <Variable
        key={key}
        name={key}
        value={value.type}
        onChangeType={(value) => changeType(key, value as FieldType)}
        onRemove={() => removeOutput(key)}
      />,
    );
  }

  return (
    <div className="space-y-3">
      {disabled || outputFields}

      <div className="flex items-center space-x-2">
        <Input
          value={newInputName}
          onChange={(e) => setNewInputName(e.target.value)}
          placeholder="New output name"
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
  const [inputSchema, setInputSchema] = useState<JsonSchemaObject>({
    type: "object",
    properties: {},
    required: [],
  });
  const [structuredOutput, setStructuredOutput] = useState(false);
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
          template: "",
          inputSchema: {
            type: "object",
            properties: {},
            required: [],
          } as any, // eslint-disable-line @typescript-eslint/no-explicit-any
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
        tools: taskData.availableToolIds || [],
      });

      setPromptText(taskData.template);
      setPromptInitialValue(taskData.template);
      setInputSchema(taskData.inputSchema as JsonSchemaObject);

      if (taskData.outputSchema) {
        setOutputSchema(taskData.outputSchema as JsonSchemaObject);
        setStructuredOutput(true);
      } else {
        setOutputSchema(null);
        setStructuredOutput(false);
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
      template: promptText!,
      outputSchema:
        !structuredOutput ||
        Object.keys(outputSchema?.properties ?? {}).length == 0
          ? null
          : outputSchema,
      availableToolIds: formState.tools,
      inputSchema: inputSchema,
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
        <div className="flex-grow max-h-[50%] min-h-[200px]">
          {promptInitialValue !== null && (
            <InputVariablesProvider value={inputSchema.properties}>
              <PromptEditor
                onChange={(prompt) => {
                  setPromptText(prompt.text);
                  setInputSchema(({ properties, ...rest }) => {
                    const newProperties: { [name: string]: JsonSchemaBase } =
                      {};
                    for (const variable of prompt.variables) {
                      newProperties[variable] = properties[variable] ?? {
                        type: "string",
                      };
                    }
                    return { properties: newProperties, ...rest };
                  });
                }}
                initialValue={promptInitialValue!}
              />
            </InputVariablesProvider>
          )}
        </div>

        <div className="grid grid-cols-1 xl:grid-cols-3 lg:grid-cols-2 gap-4 mt-4">
          {/* Inputs Section */}
          <Card>
            <CardHeader className="pb-2">
              <CardTitle className="flex items-center gap-2 text-base font-medium">
                <Parentheses className="h-4 w-4" />
                Task Input
              </CardTitle>
              <p className="text-sm text-muted-foreground">
                Inputs into the task.
              </p>
            </CardHeader>
            <CardContent>
              {Object.entries(inputSchema.properties).length === 0 && (
                <p className="text-sm text-muted-foreground">
                  Use <code>{"${variableName}"}</code> in your task description
                  above to make an input variable.
                </p>
              )}
              <div className="space-y-3">
                {Object.entries(inputSchema?.properties || {}).map(
                  ([variable, variableType]) => (
                    <Variable
                      key={variable}
                      name={variable}
                      value={variableType.type}
                      onChangeType={(value) => {
                        setInputSchema(({ properties, ...rest }) => ({
                          properties: {
                            ...properties,
                            [variable]: { type: value },
                          },
                          ...rest,
                        }));
                      }}
                    />
                  ),
                )}
              </div>
            </CardContent>
          </Card>

          {/* Outputs Section */}
          <Card>
            <CardHeader className="pb-2">
              <CardTitle className="flex items-center justify-between text-base font-medium">
                <div className="flex items-center gap-2">
                  <Parentheses className="h-4 w-4" />
                  Structured Output
                </div>
                <Switch
                  id="structured-output"
                  checked={structuredOutput}
                  onCheckedChange={(checked) => setStructuredOutput(!!checked)}
                />
              </CardTitle>
              <p className="text-sm text-muted-foreground">
                The structure the task should output.
              </p>
            </CardHeader>
            <CardContent>
              <OutputConfiguration
                onChange={(schema) => {
                  setOutputSchema(schema);
                }}
                disabled={!structuredOutput}
                outputSchema={outputSchema}
              />
            </CardContent>
          </Card>

          {/* Permissions Section */}
          <Card>
            <CardHeader className="pb-2">
              <CardTitle className="flex items-center gap-2 text-base font-medium">
                <Settings2 className="h-4 w-4" />
                Permissions
              </CardTitle>
              <p className="text-sm text-muted-foreground">
                Control which tools this task can access when running.
              </p>
            </CardHeader>
            <CardContent>
              <ToolSelector
                selectedTools={formState.tools}
                onToolSelected={handleToolToggle}
              />
            </CardContent>
          </Card>
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
