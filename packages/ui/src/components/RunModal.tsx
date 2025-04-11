/* eslint-disable @typescript-eslint/no-explicit-any */
import { Play } from "lucide-react";
import { JSX, useEffect, useState } from "react";

import { Schemas } from "@sauropod-io/client";

import api, { apiClient } from "@/api";
import ExampleCodeBlock from "@/components/ExampleCodeBlock";
import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Skeleton } from "@/components/ui/skeleton";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import {
  makeCurlSample,
  makePythonPydanticSample,
  makePythonSample as makePythonRequestSample,
  makeRustSample,
  makeTypeScriptSample,
} from "@/lib/codeSamples";
import { JsonSchemaBase, makeExampleObject } from "@/lib/jsonSchema";

interface ModalInputsProps {
  schema?: Record<string, any>;
  isLoading?: boolean;
  onRun: (parameters: Record<string, any>) => void;
  isRunning: boolean;
}

function ModalInputs({
  schema,
  isLoading,
  isRunning,
  onRun,
}: ModalInputsProps) {
  const [inputValues, setInputValues] = useState<Record<string, any>>({});

  const handleInputChange = (key: string, value: any) => {
    setInputValues((prev) => ({
      ...prev,
      [key]: value,
    }));
  };

  let fields: JSX.Element[] = [];
  if (isLoading) {
    fields = [
      <Skeleton className="h-4 w-full" />,
      <Skeleton className="h-4 w-full" />,
    ];
  } else if (schema?.properties) {
    fields = Object.entries(schema!.properties).map(
      ([key, prop]: [string, any]) => (
        <div key={key} className="grid grid-cols-4 items-center gap-4">
          <Label htmlFor={key} className="text-right">
            {key}{" "}
            {schema!.required?.includes(key) && (
              <span className="text-red-500">*</span>
            )}
          </Label>
          <Input
            id={key}
            className="col-span-3"
            value={inputValues[key] || ""}
            onChange={(e) => handleInputChange(key, e.target.value)}
            placeholder={prop.description || ""}
          />
        </div>
      ),
    );
  }

  return (
    <>
      <div className="grid gap-4 py-4">{fields}</div>
      <DialogFooter>
        <Button onClick={() => onRun(inputValues)} disabled={isRunning}>
          <Play className="h-4 w-4" />
          {isRunning ? "Running..." : "Run"}
        </Button>
      </DialogFooter>
    </>
  );
}

interface RunModalBaseProps {
  name: string;
  open: boolean;
  onOpenChange: (open: boolean) => void;
}

interface RunWorkflowModalProps extends RunModalBaseProps {
  workflowId: string;
}

interface RunTaskModalProps extends RunModalBaseProps {
  taskId: string;
}

interface RunModalProps extends RunModalBaseProps {
  schema?: Schemas["InputAndOutputSchema"];
  schemaLoading?: boolean;
  schemaError?: any;
  runUrl: string;
  onOpenChange: (open: boolean) => void;
  callRun: (parameters: Record<string, any>) => Promise<any>;
}

/** Modal used to fill out parameters when invoking tasks and workflows. */
export function RunModal({
  name,
  open,
  schema,
  schemaLoading,
  schemaError,
  runUrl,
  callRun,
  onOpenChange,
}: RunModalProps) {
  const [result, setResult] = useState<Record<string, any> | null>(null);
  const [isRunning, setIsRunning] = useState(false);
  const [runError, setRunError] = useState<string | null>(null);

  // Reset state when modal closes
  useEffect(() => {
    if (!open) {
      setResult(null);
    }
  }, [open]);

  const exampleInput = schema?.inputSchema
    ? makeExampleObject(schema.inputSchema as JsonSchemaBase)
    : {};
  const exampleOutput = schema?.outputSchema
    ? makeExampleObject(schema.outputSchema as JsonSchemaBase)
    : {};

  const runWorkflow = async (parameters: Record<string, any>) => {
    setIsRunning(true);
    setRunError(null);

    try {
      const response = await callRun(parameters);
      if (response.error) {
        setRunError(`Error: ${response.error.error}`);
      } else {
        setResult(response.data!.result as any);
      }
    } catch (err: any) {
      setRunError(`Error: ${err.message}`);
    } finally {
      setIsRunning(false);
    }
  };

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="max-w-3xl max-h-[90vh] overflow-y-auto">
        <DialogHeader>
          <DialogTitle>Run {name}</DialogTitle>
          <DialogDescription></DialogDescription>
        </DialogHeader>

        {!schemaError ? (
          <Tabs defaultValue="ui">
            <TabsList className="grid grid-cols-5">
              <TabsTrigger value="ui">UI</TabsTrigger>
              <TabsTrigger value="curl">cURL</TabsTrigger>
              <TabsTrigger value="python">Python</TabsTrigger>
              <TabsTrigger value="typescript">TypeScript</TabsTrigger>
              <TabsTrigger value="rust">Rust</TabsTrigger>
            </TabsList>

            <TabsContent value="ui">
              <ModalInputs
                schema={schema?.inputSchema}
                isLoading={schemaLoading}
                isRunning={isRunning}
                onRun={runWorkflow}
              />
            </TabsContent>

            <TabsContent value="curl">
              <ExampleCodeBlock
                language="bash"
                examples={{
                  curl: makeCurlSample(runUrl, exampleInput, exampleOutput),
                }}
              />
            </TabsContent>

            <TabsContent value="python">
              <ExampleCodeBlock
                language="python"
                examples={{
                  request: makePythonRequestSample(
                    runUrl,
                    exampleInput,
                    exampleOutput,
                  ),
                  pydantic: schema
                    ? makePythonPydanticSample(
                        runUrl,
                        schema,
                        exampleInput,
                        exampleOutput,
                      )
                    : "",
                }}
              />
            </TabsContent>

            <TabsContent value="typescript">
              <ExampleCodeBlock
                language="typescript"
                examples={{
                  fetch: schema
                    ? makeTypeScriptSample(
                        runUrl,
                        schema,
                        exampleInput,
                        exampleOutput,
                      )
                    : "",
                }}
              />
            </TabsContent>

            <TabsContent value="rust">
              <ExampleCodeBlock
                language="typescript"
                examples={{
                  reqwest: makeRustSample(runUrl, exampleInput, exampleOutput),
                }}
              />
            </TabsContent>
          </Tabs>
        ) : (
          <div className="p-4 my-2 bg-red-50 text-red-600 rounded-md">
            {schemaError.error}
          </div>
        )}

        {runError && (
          <div className="p-4 my-2 bg-red-50 text-red-600 rounded-md">
            {runError}
          </div>
        )}

        {result !== null && (
          <p className="rounded-md bg-gray-100 p-4">
            {JSON.stringify(result, null, 4)}
          </p>
        )}
      </DialogContent>
    </Dialog>
  );
}

/** Modal used to fill out parameters when invoking workflows. */
export function WorkflowRunModal({
  workflowId,
  name,
  open,
  onOpenChange,
}: RunWorkflowModalProps) {
  const {
    data: schema,
    isLoading: schemaLoading,
    error: schemaError,
  } = api.useQuery("get", `/api/workflow/{id}/schema`, {
    params: { path: { id: `${workflowId}` } },
  });
  const runUrl = `${window.location.origin}/api/workflow/${workflowId}/run`;
  const callRun = (parameters: Record<string, any>) =>
    apiClient.POST("/api/workflow/{id}/run", {
      params: { path: { id: workflowId } },
      body: parameters,
    });
  return (
    <RunModal
      name={name}
      open={open}
      onOpenChange={onOpenChange}
      runUrl={runUrl}
      schema={schema}
      schemaLoading={schemaLoading}
      schemaError={schemaError}
      callRun={callRun}
    />
  );
}

/** Modal used to fill out parameters when invoking tasks. */
export function TaskRunModal({
  taskId,
  name,
  open,
  onOpenChange,
}: RunTaskModalProps) {
  const {
    data: schema,
    isLoading: schemaLoading,
    error: schemaError,
  } = api.useQuery("get", `/api/task/{id}/schema`, {
    params: { path: { id: `${taskId}` } },
  });
  const runUrl = `${window.location.origin}/api/task/${taskId}/run`;
  const callRun = (parameters: Record<string, any>) =>
    apiClient.POST("/api/task/{id}/run", {
      params: { path: { id: taskId } },
      body: parameters,
    });

  return (
    <RunModal
      name={name}
      open={open}
      onOpenChange={onOpenChange}
      runUrl={runUrl}
      schema={schema}
      schemaLoading={schemaLoading}
      schemaError={schemaError}
      callRun={callRun}
    />
  );
}
