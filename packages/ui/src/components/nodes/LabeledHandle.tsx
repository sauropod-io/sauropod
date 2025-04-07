import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from "../ui/tooltip";
import { HandleProps } from "@xyflow/react";
import { HTMLAttributes, forwardRef } from "react";

import { BaseHandle } from "@/components/nodes/BaseHandle";
import { cn } from "@/lib/utils";

const flexDirections = {
  top: "flex-col",
  right: "justify-end",
  bottom: "flex-col-reverse justify-end",
  left: "flex-row",
};

export const LabeledHandle = forwardRef<
  HTMLDivElement,
  HandleProps &
    HTMLAttributes<HTMLDivElement> & {
      title: string;
      handleClassName?: string;
      labelClassName?: string;
      description?: string;
    }
>(
  (
    {
      className,
      labelClassName,
      handleClassName,
      title,
      position,
      description,
      ...props
    },
    ref,
  ) => (
    <div
      ref={ref}
      title={title}
      className={cn(
        "relative flex items-center align-middle",
        flexDirections[position],
        className,
      )}
    >
      <BaseHandle position={position} className={handleClassName} {...props} />
      <TooltipProvider>
        <Tooltip>
          <TooltipTrigger asChild>
            <label
              className={cn(
                "inline px-3 font-mono text-sm text-foreground align-middle",
                labelClassName,
              )}
            >
              {title}
            </label>
          </TooltipTrigger>
          <TooltipContent>
            <p>{description || title}</p>
          </TooltipContent>
        </Tooltip>
      </TooltipProvider>
    </div>
  ),
);

LabeledHandle.displayName = "LabeledHandle";
