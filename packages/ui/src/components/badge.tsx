import { Badge } from "@/components/ui/badge";
import { cn } from "@/lib/utils";

export type Level = "info" | "warning" | "error" | "default";

/** Get the color class names to use for the badge. */
function getBadgeColor(color: string): string {
  return `bg-${color}-100 text-${color}-800 dark:bg-${color}-900 dark:text-${color}-300`;
}

function getLevelColor(level: Level | null): string {
  switch (level) {
    case "info":
      return getBadgeColor("blue");
    case "warning":
      return getBadgeColor("yellow");
    case "error":
      return getBadgeColor("red");
    default:
      return getBadgeColor("gray");
  }
}

/** A badge with an associated level. */
export function LevelBadge({
  children,
  level,
  className,
}: {
  children: React.ReactNode;
  level: Level | null;
  className?: string;
}) {
  return (
    <Badge variant="default" className={cn(className, getLevelColor(level))}>
      {children}
    </Badge>
  );
}

/** An error badge. */
export function ErrorBadge({
  children,
  className,
}: {
  children: React.ReactNode;
  className?: string;
}) {
  return (
    <LevelBadge level="error" className={className}>
      {children}
    </LevelBadge>
  );
}
