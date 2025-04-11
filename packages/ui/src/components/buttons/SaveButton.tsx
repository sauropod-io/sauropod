import { Save } from "lucide-react";

import IconButton from "@/components/buttons/IconButton";
import { type ButtonProps } from "@/components/ui/button";

export default function SaveButton(props: ButtonProps) {
  return <IconButton Icon={Save} text="Save" {...props} />;
}
