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
import { JsonSchemaBase } from "@/lib/jsonSchema";
import {
  isJsonSchemaAudio,
  isJsonSchemaImage,
  makeExampleObject,
} from "@/lib/jsonSchemaExtensions";

// Function to convert File to base64
function fileToBase64(file: File): Promise<string> {
  return new Promise((resolve, reject) => {
    const reader = new FileReader();
    reader.readAsDataURL(file);
    reader.onload = () => resolve(reader.result as string);
    reader.onerror = (error) => reject(error);
  });
}

// Common interface for file upload components
interface FileUploadProps {
  id: string;
  value: string;
  onChange: (value: string) => void;
  placeholder?: string;
  accept: string;
}

// Generic file upload component that handles file conversion
function FileUploadBase({
  id,
  value,
  onChange,
  placeholder,
  accept,
  children,
}: FileUploadProps & { children: (value: string) => JSX.Element | null }) {
  const [loading, setLoading] = useState(false);

  const handleFileChange = async (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (!file) return;

    setLoading(true);
    try {
      const base64 = await fileToBase64(file);
      onChange(base64);
    } catch (error) {
      console.error("Error converting file to base64:", error);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="col-span-3">
      <div className="flex gap-2 items-center flex-row">
        <Input
          type="file"
          id={id}
          accept={accept}
          onChange={handleFileChange}
          placeholder={placeholder}
          className="w-full"
        />
        {loading && <span className="text-sm">Loading...</span>}
        {value && !loading && children(value)}
      </div>
    </div>
  );
}

// Image Upload Component
function ImageUploadInput(props: Omit<FileUploadProps, "accept">) {
  return (
    <FileUploadBase {...props} accept="image/*">
      {(value) => (
        <div className="w-12 h-12 border rounded overflow-hidden">
          <img
            src={value}
            alt="Preview"
            className="w-full h-full object-cover"
          />
        </div>
      )}
    </FileUploadBase>
  );
}

// Audio Upload Component
function AudioUploadInput(props: Omit<FileUploadProps, "accept">) {
  return (
    <FileUploadBase {...props} accept="audio/*">
      {(value) => (
        <div className="mt-2 w-full">
          <audio controls src={value} className="w-full">
            Your browser does not support the audio element.
          </audio>
        </div>
      )}
    </FileUploadBase>
  );
}

interface ModalInputsProps {
  schema?: {
    properties?: Record<string, JsonSchemaBase>;
    required?: string[];
  };
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

  const renderField = (key: string, prop: JsonSchemaBase) => {
    const isRequired = schema?.required?.includes(key);

    return (
      <div key={key} className="grid grid-cols-4 items-center gap-4">
        <Label htmlFor={key} className="text-right">
          {key} {isRequired && <span className="text-red-500">*</span>}
        </Label>
        {renderInput(key, prop)}
      </div>
    );
  };

  const renderInput = (key: string, prop: JsonSchemaBase) => {
    if (isJsonSchemaImage(prop)) {
      return (
        <ImageUploadInput
          id={key}
          value={inputValues[key] || ""}
          onChange={(value) => handleInputChange(key, value)}
          placeholder={prop.description}
        />
      );
    }

    if (isJsonSchemaAudio(prop)) {
      return (
        <AudioUploadInput
          id={key}
          value={inputValues[key] || ""}
          onChange={(value) => handleInputChange(key, value)}
          placeholder={prop.description}
        />
      );
    }

    return (
      <Input
        id={key}
        className="col-span-3"
        value={inputValues[key] || ""}
        onChange={(e) => handleInputChange(key, e.target.value)}
        placeholder={prop.description || ""}
      />
    );
  };

  const renderFields = () => {
    if (isLoading) {
      return [
        <Skeleton key="skeleton1" className="h-4 w-full" />,
        <Skeleton key="skeleton2" className="h-4 w-full" />,
      ];
    }

    if (!schema?.properties) {
      return [];
    }

    return Object.entries(schema.properties).map(([key, prop]) =>
      renderField(key, prop as JsonSchemaBase),
    );
  };

  return (
    <>
      <div className="grid gap-4 py-4">{renderFields()}</div>
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
        setResult(response.data! as any);
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
