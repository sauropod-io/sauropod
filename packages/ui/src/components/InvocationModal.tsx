/* eslint-disable @typescript-eslint/no-explicit-any */
import { JSX, useEffect, useState } from "react";

import api, { apiClient } from "@/api";
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
      <div className="grid gap-4">{fields}</div>
      <DialogFooter>
        <Button onClick={() => onRun(inputValues)} disabled={isRunning}>
          {isRunning ? "Running..." : "Run Workflow"}
        </Button>
      </DialogFooter>
    </>
  );
}

interface InvocationModalProps {
  workflowId?: string;
  workflowName: string;
  open: boolean;
  onOpenChange: (open: boolean) => void;
}

/** Modal used to fill out parameters when invoking tasks and workflows. */
export function InvocationModal({
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

  const runWorkflow = async (parameters: Record<string, any>) => {
    if (!workflowId) return;

    setIsRunning(true);
    setError(null);

    try {
      const response = await apiClient.POST("/api/workflow/{id}/invoke", {
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
          <DialogDescription>
            {workflowResult
              ? "Workflow execution results"
              : "Enter input parameters to run this workflow"}
          </DialogDescription>
        </DialogHeader>

        {!schemaError ? (
          <ModalInputs
            schema={schema?.inputSchema}
            isLoading={schemaLoading}
            isRunning={isRunning}
            onRun={runWorkflow}
          />
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
