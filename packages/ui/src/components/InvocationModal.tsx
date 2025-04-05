/* eslint-disable @typescript-eslint/no-explicit-any */
import { useEffect, useState } from "react";

import { apiClient } from "@/api";
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
import { Textarea } from "@/components/ui/textarea";

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
  const [inputSchema, setInputSchema] = useState<Record<string, any> | null>(
    null,
  );
  const [inputValues, setInputValues] = useState<Record<string, any>>({});
  const [workflowResult, setWorkflowResult] = useState<Record<
    string,
    any
  > | null>(null);
  const [isRunning, setIsRunning] = useState(false);
  const [error, setError] = useState<string | null>(null);

  // Fetch input schema when modal opens
  useEffect(() => {
    const fetchSchema = async () => {
      if (!open || !workflowId) return;

      setIsRunning(true);
      setError(null);
      setWorkflowResult(null);

      try {
        const response = await apiClient.GET("/api/workflow/{id}/inputSchema", {
          params: { path: { id: workflowId } },
        });
        setInputSchema(response.data!);

        // Initialize input values with defaults
        const initialValues: Record<string, any> = {};
        if (response.data?.properties) {
          Object.entries(response.data.properties).forEach(
            ([key, prop]: [string, any]) => {
              initialValues[key] = prop.default || "";
            },
          );
        }
        setInputValues(initialValues);
      } catch (err) {
        setError(
          `Failed to fetch input schema: ${JSON.stringify(err, null, 4)}`,
        );
      } finally {
        setIsRunning(false);
      }
    };

    fetchSchema();
  }, [workflowId, open]);

  // Reset state when modal closes
  useEffect(() => {
    if (!open) {
      setWorkflowResult(null);
    }
  }, [open]);

  const handleInputChange = (key: string, value: any) => {
    setInputValues((prev) => ({
      ...prev,
      [key]: value,
    }));
  };

  const runWorkflow = async () => {
    if (!workflowId) return;

    setIsRunning(true);
    setError(null);

    try {
      const response = await apiClient.POST("/api/workflow/{id}/invoke", {
        params: { path: { id: workflowId } },
        body: inputValues,
      });
      setWorkflowResult(response.data!);
    } catch (err: any) {
      setError(`Failed to run workflow: ${err.message}`);
    } finally {
      setIsRunning(false);
    }
  };
  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="max-w-3xl max-h-[90vh] overflow-y-auto">
        <DialogHeader>
          <DialogTitle>Run Workflow: {workflowName}</DialogTitle>
          <DialogDescription>
            {workflowResult
              ? "Workflow execution results"
              : "Enter input parameters to run this workflow"}
          </DialogDescription>
        </DialogHeader>

        {error && (
          <div className="p-4 my-2 bg-red-50 text-red-600 rounded-md">
            {error}
          </div>
        )}

        {!workflowResult ? (
          <div className="grid gap-4 py-4">
            {inputSchema?.properties &&
              Object.entries(inputSchema.properties).map(
                ([key, prop]: [string, any]) => (
                  <div
                    key={key}
                    className="grid grid-cols-4 items-center gap-4"
                  >
                    <Label htmlFor={key} className="text-right">
                      {key}{" "}
                      {inputSchema.required?.includes(key) && (
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
              )}
          </div>
        ) : (
          <div className="py-4">
            <Label>Result:</Label>
            <Textarea
              className="font-mono mt-2"
              rows={15}
              readOnly
              value={JSON.stringify(workflowResult, null, 2)}
            />
          </div>
        )}

        <DialogFooter>
          {!workflowResult ? (
            <Button onClick={runWorkflow} disabled={isRunning}>
              {isRunning ? "Running..." : "Run Workflow"}
            </Button>
          ) : (
            <Button onClick={() => setWorkflowResult(null)}>Run Again</Button>
          )}
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
