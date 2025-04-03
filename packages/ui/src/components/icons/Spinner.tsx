import { LoaderCircle } from "lucide-react";

import { cn } from "@/lib/utils";

/** A loading spinner icon. */
export default function Spinner({ className }: { className: string }) {
  return <LoaderCircle className={cn("animate-spin", className)} />;
}
