import { Plus } from "lucide-react";
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
import { Skeleton } from "@/components/ui/skeleton";

export interface TaskSelectorProps {
  autoFocus?: boolean;
  onSelect: (taskId: number) => void;
}

export default function TaskSelector({
  autoFocus,
  onSelect,
}: TaskSelectorProps) {
  const { data: availableTasks, isLoading: isLoadingTasks } = api.useQuery(
    "get",
    "/api/task",
  );

  const [searchQuery, setSearchQuery] = useState("");

  return (
    <Command className="rounded-lg border shadow-md max-h-96">
      <CommandInput
        placeholder="Search tasks..."
        value={searchQuery}
        onValueChange={setSearchQuery}
        autoFocus={autoFocus}
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
                  task.name.toLowerCase().includes(searchQuery.toLowerCase()),
                )
                .map((task) => (
                  <CommandItem key={task.id} onSelect={() => onSelect(task.id)}>
                    {task.name}
                    <Button
                      size="sm"
                      variant="ghost"
                      className="ml-auto"
                      onClick={(e) => {
                        e.stopPropagation();
                        onSelect(task.id);
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
  );
}
