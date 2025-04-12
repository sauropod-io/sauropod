import { ListChecks, Plus, Trash2 } from "lucide-react";
import { useState } from "react";

import TaskSelector from "@/components/TaskSelector";
import IconButton from "@/components/buttons/IconButton";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import {
  Sheet,
  SheetContent,
  SheetDescription,
  SheetHeader,
  SheetTitle,
  SheetTrigger,
} from "@/components/ui/sheet";

interface WorkflowConfigSheetProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  inputs: string[];
  onAddInput: (inputName: string) => void;
  onRemoveInput: (inputName: string) => void;
  outputs: string[];
  onAddOutput: (outputName: string) => void;
  onRemoveOutput: (outputName: string) => void;
  onAddTask: (taskId: number) => void;
}

export function WorkflowConfigSheet({
  open,
  onOpenChange,
  inputs,
  onAddInput,
  onRemoveInput,
  outputs,
  onAddOutput,
  onRemoveOutput,
  onAddTask,
}: WorkflowConfigSheetProps) {
  const [newInputName, setNewInputName] = useState("");
  const [newOutputName, setNewOutputName] = useState("");

  const handleAddInput = () => {
    if (!newInputName.trim()) return;
    onAddInput(newInputName);
    setNewInputName("");
  };

  const handleAddOutput = () => {
    if (!newOutputName.trim()) return;
    onAddOutput(newOutputName);
    setNewOutputName("");
  };

  return (
    <Sheet open={open} onOpenChange={onOpenChange}>
      <SheetTrigger asChild>
        <IconButton Icon={ListChecks} text="Configuration" variant="outline" />
      </SheetTrigger>
      <SheetContent className="px-4 overflow-y-auto w-[80vw] sm:w-[400px]">
        <SheetHeader>
          <SheetTitle>Workflow Configuration</SheetTitle>

          <SheetDescription>
            Configure the workflow by adding input, outputs, and tasks.
          </SheetDescription>
        </SheetHeader>

        <div className="flex flex-col gap-6">
          {/* Inputs Section */}
          <div>
            <h3 className="text-lg font-medium mb-2">Inputs</h3>
            <div className="flex items-center gap-2 mb-4">
              <Input
                placeholder="New input name"
                value={newInputName}
                onChange={(e) => setNewInputName(e.target.value)}
                onKeyDown={(e) => {
                  if (e.key === "Enter") {
                    handleAddInput();
                  }
                }}
                className="flex-1"
              />
              <Button onClick={handleAddInput} size="sm">
                <Plus className="h-4 w-4 mr-1" /> Add
              </Button>
            </div>

            {inputs.length > 0 ? (
              <div className="space-y-2">
                {inputs.map((input) => (
                  <div
                    key={input}
                    className="flex items-center justify-between bg-muted p-2 rounded-md"
                  >
                    <span>{input}</span>
                    <Button
                      size="sm"
                      variant="ghost"
                      onClick={() => onRemoveInput(input)}
                    >
                      <Trash2 className="h-4 w-4" />
                    </Button>
                  </div>
                ))}
              </div>
            ) : (
              <p className="text-sm text-muted-foreground">No inputs defined</p>
            )}
          </div>

          {/* Outputs Section */}
          <div>
            <h3 className="text-lg font-medium mb-2">Outputs</h3>
            <div className="flex items-center gap-2 mb-4">
              <Input
                placeholder="New output name"
                value={newOutputName}
                onChange={(e) => setNewOutputName(e.target.value)}
                onKeyDown={(e) => {
                  if (e.key === "Enter") {
                    handleAddOutput();
                  }
                }}
                className="flex-1"
              />
              <Button onClick={handleAddOutput} size="sm">
                <Plus className="h-4 w-4 mr-1" /> Add
              </Button>
            </div>

            {outputs.length > 0 ? (
              <div className="space-y-2">
                {outputs.map((output) => (
                  <div
                    key={output}
                    className="flex items-center justify-between bg-muted p-2 rounded-md"
                  >
                    <span>{output}</span>
                    <Button
                      size="sm"
                      variant="ghost"
                      onClick={() => onRemoveOutput(output)}
                    >
                      <Trash2 className="h-4 w-4" />
                    </Button>
                  </div>
                ))}
              </div>
            ) : (
              <p className="text-sm text-muted-foreground">
                No outputs defined
              </p>
            )}
          </div>

          {/* Tasks Section */}
          <div>
            <h3 className="text-lg font-medium mb-2">Tasks</h3>
            <TaskSelector onSelect={onAddTask} />
          </div>
        </div>
      </SheetContent>
    </Sheet>
  );
}
