import { Logs } from "lucide-react";
import { Link } from "react-router";

import type { Schemas } from "@sauropod-io/client";

import {
  Card,
  CardContent,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { LOGS } from "@/routes";

interface ErrorCardProps {
  message: string;
  error: Schemas["Error"];
  className?: string;
  buttons?: React.ReactNode;
}

export default function ErrorCard({
  message,
  className,
  error,
  buttons,
}: ErrorCardProps) {
  return (
    <Card className={className}>
      <CardHeader>
        <CardTitle>{message}</CardTitle>
      </CardHeader>
      <CardContent>
        <p className="text-red-500">{error.error}</p>
        See the&nbsp;
        <Link to={LOGS}>
          <Logs className="mr-1 inline" />
          Logs
        </Link>
        &nbsp;for more info
      </CardContent>
      <CardFooter>{buttons}</CardFooter>
    </Card>
  );
}
