import { ListChecks, Plus, Trash2 } from "lucide-react";
import { useState } from "react";

import api from "@/api";
import { Button } from "@/components/ui/button";
import {
  Command,
  CommandEmpty,
  CommandGroup,
  CommandInput,
  CommandItem,
  CommandList,
} from "@/components/ui/command";
import { Input } from "@/components/ui/input";
import {
  Sheet,
  SheetContent,
  SheetDescription,
  SheetHeader,
  SheetTitle,
  SheetTrigger,
} from "@/components/ui/sheet";
import { Skeleton } from "@/components/ui/skeleton";

interface WorkflowInput {
  id: string;
  name: string;
}

interface WorkflowTask {
  id: number;
  name: string;
}

interface WorkflowConfigSheetProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  inputs: WorkflowInput[];
  onAddInput: (inputName: string) => void;
  onRemoveInput: (inputName: string) => void;
  selectedTasks: WorkflowTask[];
  onAddTask: (taskId: number, taskName: string) => void;
  onRemoveTask: (taskId: number) => void;
}

export function WorkflowConfigSheet({
  open,
  onOpenChange,
  inputs,
  onAddInput,
  onRemoveInput,
  selectedTasks,
  onAddTask,
  onRemoveTask,
}: WorkflowConfigSheetProps) {
  const { data: availableTasks, isLoading: isLoadingTasks } = api.useQuery(
    "get",
    "/api/task",
  );

  const [newInputName, setNewInputName] = useState("");
  const [searchQuery, setSearchQuery] = useState("");

  const handleAddInput = () => {
    if (!newInputName.trim()) return;
    onAddInput(newInputName);
    setNewInputName("");
  };

  return (
    <Sheet open={open} onOpenChange={onOpenChange}>
      <SheetTrigger asChild>
        <Button size="sm" variant="outline">
          <ListChecks className="mr-2 h-4 w-4" />
          Tasks
        </Button>
      </SheetTrigger>
      <SheetContent className="px-4 overflow-y-auto w-[80vw] sm:w-[400px]">
        <SheetHeader>
          <SheetTitle>Workflow Configuration</SheetTitle>

          <SheetDescription>
            Configure the workflow by adding input parameters and tasks.
          </SheetDescription>
        </SheetHeader>
        <div className="flex flex-col gap-6">
          {/* Inputs Section */}
          <div>
            <h3 className="text-lg font-medium mb-2">Input Parameters</h3>
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

          {/* Tasks Section */}
          <div>
            <h3 className="text-lg font-medium mb-2">Tasks</h3>
            <Command className="rounded-lg border shadow-md max-h-96">
              <CommandInput
                placeholder="Search tasks..."
                value={searchQuery}
                onValueChange={setSearchQuery}
              />
              <CommandList>
                {isLoadingTasks ? (
                  <div className="p-2 space-y-3">
                    <Skeleton className="h-8 w-full" />
                    <Skeleton className="h-8 w-full" />
                    <Skeleton className="h-8 w-full" />
                  </div>
                ) : (
                  <>
                    <CommandEmpty>No tasks found.</CommandEmpty>
                    <CommandGroup heading="Available Tasks">
                      {availableTasks!
                        .filter((task) =>
                          task.name
                            .toLowerCase()
                            .includes(searchQuery.toLowerCase()),
                        )
                        .map((task) => (
                          <CommandItem
                            key={task.id}
                            onSelect={() => onAddTask(task.id, task.name)}
                          >
                            {task.name}
                            <Button
                              size="sm"
                              variant="ghost"
                              className="ml-auto"
                              onClick={(e) => {
                                e.stopPropagation();
                                onAddTask(task.id, task.name);
                              }}
                            >
                              <Plus className="h-3 w-3" />
                            </Button>
                          </CommandItem>
                        ))}
                    </CommandGroup>
                  </>
                )}
              </CommandList>
            </Command>
            <div className="mt-4">
              <h4 className="text-sm font-medium mb-2">Selected Tasks</h4>
              {selectedTasks.length > 0 ? (
                <div className="space-y-2">
                  {selectedTasks.map((task) => (
                    <div
                      key={task.id}
                      className="flex items-center justify-between bg-muted p-2 rounded-md"
                    >
                      <span>{task.name}</span>
                      <Button
                        size="sm"
                        variant="ghost"
                        onClick={() => onRemoveTask(task.id)}
                      >
                        <Trash2 className="h-4 w-4" />
                      </Button>
                    </div>
                  ))}
                </div>
              ) : (
                <p className="text-sm text-muted-foreground">No tasks added</p>
              )}
            </div>
          </div>
        </div>
      </SheetContent>
    </Sheet>
  );
}
