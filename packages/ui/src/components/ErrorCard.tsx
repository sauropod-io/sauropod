import { Logs } from "lucide-react";
import { Link } from "react-router";

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
  error: Error;
}

export default function ErrorCard({ message, error }: ErrorCardProps) {
  return (
    <main className="container mx-auto p-6 max-w-lg">
      <Card className="mb-6">
        <CardHeader>
          <CardTitle>{message}</CardTitle>
        </CardHeader>
        <CardContent>
          <p className="text-red-500">{error.message}</p>
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
    </main>
  );
}
