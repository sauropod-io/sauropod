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
}

export default function ErrorCard({
  message,
  className,
  error,
}: ErrorCardProps) {
  return (
    <Card className={className}>
      <CardHeader>
        <CardTitle>{message}</CardTitle>
      </CardHeader>
      <CardContent>
        <p className="text-red-500">{error.error}</p>
      </CardContent>
      <CardFooter className="">
        See the&nbsp;
        <Link to={LOGS}>
          <Logs className="mr-1 inline" />
          Logs
        </Link>
        &nbsp;for more info
      </CardFooter>
    </Card>
  );
}
