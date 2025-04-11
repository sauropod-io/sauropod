import { Play } from "lucide-react";

import IconButton from "@/components/buttons/IconButton";
import { type ButtonProps } from "@/components/ui/button";

export default function RunButton({
  variant = "default",
  ...props
}: ButtonProps) {
  return <IconButton Icon={Play} variant={variant} text="Run" {...props} />;
}
