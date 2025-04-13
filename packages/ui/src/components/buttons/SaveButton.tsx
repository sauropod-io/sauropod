import { Check, Save } from "lucide-react";

import IconButton from "@/components/buttons/IconButton";
import { type ButtonProps } from "@/components/ui/button";

export default function SaveButton({
  showSuccess,
  ...props
}: ButtonProps & { showSuccess?: boolean }) {
  return (
    <IconButton Icon={showSuccess ? Check : Save} text="Save" {...props} />
  );
}
