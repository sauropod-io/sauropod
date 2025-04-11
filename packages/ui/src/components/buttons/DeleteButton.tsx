import { Trash2 } from "lucide-react";

import IconButton from "@/components/buttons/IconButton";
import { type ButtonProps } from "@/components/ui/button";

export default function DeleteButton(props: ButtonProps) {
  return (
    <IconButton Icon={Trash2} text="Delete" variant="destructive" {...props} />
  );
}
