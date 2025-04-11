import { Button, type ButtonProps } from "@/components/ui/button";

interface IconButtonProps extends ButtonProps {
  Icon: React.ElementType;
  text: string;
}

export default function IconButton({ Icon, text, ...props }: IconButtonProps) {
  return (
    <Button variant="outline" {...props}>
      <Icon className="h-4 w-4" />
      <span className="hidden md:inline">{text}</span>
    </Button>
  );
}
