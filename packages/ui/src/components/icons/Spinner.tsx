import { cn } from "@/lib/utils";
import { LoaderCircle } from "lucide-react";

/** A loading spinner icon. */
export default function Spinner({ className }: { className: string }) {
  return <LoaderCircle className={cn("animate-spin", className)} />;
}
