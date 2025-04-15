import { Check, ChevronsUpDown } from "lucide-react";
import { useState } from "react";

import { Button } from "@/components/ui/button";
import {
  Command,
  CommandEmpty,
  CommandGroup,
  CommandInput,
  CommandItem,
  CommandList,
} from "@/components/ui/command";
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from "@/components/ui/popover";
import { cn } from "@/lib/utils";
import { ToolConsumer } from "@/providers/ToolsProvider";

interface ToolSelectorProps {
  selectedTools: string[];
  className?: string;
  onToolSelected: (toolId: string) => void;
}

/** Selector for tools. */
export default function ToolSelector({
  selectedTools,
  className,
  onToolSelected,
}: ToolSelectorProps) {
  const [open, setOpen] = useState(false);

  return (
    <ToolConsumer>
      {(tools) => {
        // Group tools by provider
        const toolsByProvider: Record<string, typeof tools> = {};
        tools.forEach((tool) => {
          if (!toolsByProvider[tool.provider]) {
            toolsByProvider[tool.provider] = [];
          }
          toolsByProvider[tool.provider].push(tool);
        });

        return (
          <Popover open={open} onOpenChange={setOpen}>
            <PopoverTrigger asChild>
              <Button
                variant="outline"
                role="combobox"
                aria-expanded={open}
                className={cn(className, "w-full justify-between")}
              >
                {selectedTools.length === 0
                  ? "Allowed tools..."
                  : `${selectedTools.length} tool${selectedTools.length === 1 ? "" : "s"} allowed`}
                <ChevronsUpDown className="ml-2 h-4 w-4 shrink-0 opacity-50" />
              </Button>
            </PopoverTrigger>
            <PopoverContent className="w-[300px] p-0">
              <Command>
                <CommandInput placeholder="Search tools..." />
                <CommandList>
                  <CommandEmpty>No tools found.</CommandEmpty>
                  {Object.entries(toolsByProvider).map(
                    ([provider, providerTools]) => (
                      <CommandGroup key={provider} heading={provider}>
                        {providerTools.map((tool) => {
                          const isSelected = selectedTools.includes(tool.id);
                          return (
                            <CommandItem
                              key={tool.id}
                              value={tool.id}
                              onSelect={() => onToolSelected(tool.id)}
                            >
                              <Check
                                className={cn(
                                  "mr-2 h-4 w-4",
                                  isSelected ? "opacity-100" : "opacity-0",
                                )}
                              />
                              {tool.name}
                            </CommandItem>
                          );
                        })}
                      </CommandGroup>
                    ),
                  )}
                </CommandList>
              </Command>
            </PopoverContent>
          </Popover>
        );
      }}
    </ToolConsumer>
  );
}
