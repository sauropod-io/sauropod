import { Handle, HandleProps, useNodeConnections } from "@xyflow/react";
import { forwardRef } from "react";

import "@/components/nodes/BaseHandle.css";
import { cn } from "@/lib/utils";

export type BaseHandleProps = HandleProps;

export const BaseHandle = forwardRef<HTMLDivElement, BaseHandleProps>(
  ({ className, children, isConnectable, id, type, ...props }, ref) => {
    const connections = useNodeConnections({
      handleType: type,
    });

    // Default isConnectable to true if not provided
    if (isConnectable === undefined) {
      if (type == "target" && id !== undefined) {
        // Only allow 1 connection to target handles by default
        isConnectable = !connections.some((x) => x.targetHandle === id);
      } else {
        isConnectable = true;
      }
    }

    return (
      <Handle
        ref={ref}
        {...props}
        className={cn(
          "h-[11px] w-[11px] rounded-full border border-slate-300 bg-slate-100 transition dark:border-secondary dark:bg-secondary",
          className,
        )}
        isConnectable={isConnectable}
        type={type}
        id={id}
        {...props}
      >
        {children}
      </Handle>
    );
  },
);

BaseHandle.displayName = "BaseHandle";
