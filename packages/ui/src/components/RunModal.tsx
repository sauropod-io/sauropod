/* eslint-disable @typescript-eslint/no-explicit-any */
import { Play } from "lucide-react";
import { JSX, useEffect, useState } from "react";

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
  makePythonSample,
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

/** Get the URL to make the API call to invoke a run. */
function getRunUrl(workflowId: string) {
  return `${window.location.origin}/api/workflow/${workflowId}/run`;
}

interface InvocationModalProps {
  workflowId?: string;
  workflowName: string;
  open: boolean;
  onOpenChange: (open: boolean) => void;
}

/** Modal used to fill out parameters when invoking tasks and workflows. */
export function RunModal({
  workflowId,
  workflowName,
  open,
  onOpenChange,
}: InvocationModalProps) {
  const {
    data: schema,
    isLoading: schemaLoading,
    error: schemaError,
  } = api.useQuery("get", `/api/workflow/{id}/schema`, {
    params: { path: { id: `${workflowId}` } },
  });
  const [workflowResult, setWorkflowResult] = useState<Record<
    string,
    any
  > | null>(null);
  const [isRunning, setIsRunning] = useState(false);
  const [workflowError, setError] = useState<string | null>(null);

  // Reset state when modal closes
  useEffect(() => {
    if (!open) {
      setWorkflowResult(null);
    }
  }, [open]);

  const runUrl = getRunUrl(workflowId!);
  const exampleInput = schema?.inputSchema
    ? makeExampleObject(schema.inputSchema as JsonSchemaBase)
    : {};
  const exampleOutput = schema?.outputSchema
    ? makeExampleObject(schema.outputSchema as JsonSchemaBase)
    : {};

  const runWorkflow = async (parameters: Record<string, any>) => {
    if (!workflowId) return;

    setIsRunning(true);
    setError(null);

    try {
      const response = await apiClient.POST("/api/workflow/{id}/run", {
        params: { path: { id: workflowId } },
        body: parameters,
      });
      if (response.error) {
        setError(`Error: ${response.error.error}`);
      } else {
        setWorkflowResult(response.data!.result as any);
      }
    } catch (err: any) {
      setError(`Error invoking workflow: ${err.message}`);
    } finally {
      setIsRunning(false);
    }
  };

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="max-w-3xl max-h-[90vh] overflow-y-auto">
        <DialogHeader>
          <DialogTitle>Run {workflowName}</DialogTitle>
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
              <ExampleCodeBlock language="bash">
                {makeCurlSample(runUrl, exampleInput, exampleOutput)}
              </ExampleCodeBlock>
            </TabsContent>

            <TabsContent value="python">
              <ExampleCodeBlock language="python">
                {makePythonSample(runUrl, exampleInput, exampleOutput)}
              </ExampleCodeBlock>
            </TabsContent>

            <TabsContent value="typescript">
              <ExampleCodeBlock language="typescript">
                {makeTypeScriptSample(runUrl, exampleInput, exampleOutput)}
              </ExampleCodeBlock>
            </TabsContent>

            <TabsContent value="rust">
              <ExampleCodeBlock language="rust">
                {makeRustSample(runUrl, exampleInput, exampleOutput)}
              </ExampleCodeBlock>
            </TabsContent>
          </Tabs>
        ) : (
          <div className="p-4 my-2 bg-red-50 text-red-600 rounded-md">
            {schemaError.error}
          </div>
        )}

        {workflowError && (
          <div className="p-4 my-2 bg-red-50 text-red-600 rounded-md">
            {workflowError}
          </div>
        )}

        {workflowResult !== null && (
          <p className="rounded-md bg-gray-100 p-4">
            {JSON.stringify(workflowResult, null, 4)}
          </p>
        )}
      </DialogContent>
    </Dialog>
  );
}
