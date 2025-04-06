import { Logs, Play, Trash2 } from "lucide-react";
import { Link, useNavigate } from "react-router";

import type { Schemas } from "@sauropod-io/client";

import api from "@/api";
import WorkflowPreview from "@/components/WorkflowPreview";
import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Skeleton } from "@/components/ui/skeleton";
import { useDeleteWorkflow } from "@/mutations/workflowMutations";
import { LOGS, workflowRoute } from "@/routes";

function SkeletonCard() {
  return (
    <Card>
      <CardHeader>
        <CardTitle>
          <Skeleton className="h-4 w-[200px]" />
        </CardTitle>
        <CardDescription>
          <Skeleton className="h-4 w-[200px]" />
        </CardDescription>
      </CardHeader>
    </Card>
  );
}

function WorkflowCard({ item }: { item: Schemas["ObjectInfo"] }) {
  const { data, isLoading, error } = api.useQuery("get", `/api/workflow/{id}`, {
    params: { path: { id: `${item.id}` } },
  });
  const navigate = useNavigate();

  const deleteTask = useDeleteWorkflow();
  const handleDelete = async (e: React.MouseEvent) => {
    e.stopPropagation(); // Prevent navigation when clicking delete button
    deleteTask.mutate({
      params: { path: { id: `${item.id}` } },
    });
  };

  const handleRun = (e: React.MouseEvent) => {
    e.stopPropagation(); // Prevent default card navigation
    navigate(`${workflowRoute(item.id)}?run=true`);
  };

  if (isLoading || data === undefined) {
    return <SkeletonCard />;
  }

  const deleteButton = (
    <Button
      variant="destructive"
      size="sm"
      className="cursor-pointer"
      onClick={handleDelete}
    >
      <Trash2 className="h-4 w-4 mr-1" />
      Delete
    </Button>
  );

  if (error != null) {
    return (
      <Card>
        <CardHeader>
          <CardTitle>Error loading workflow (ID {item.id})</CardTitle>
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
        <CardFooter className="flex justify-end gap-2 pt-0">
          {deleteButton}
        </CardFooter>
      </Card>
    );
  }

  return (
    <Card>
      <CardHeader
        className="flex-grow cursor-pointer"
        onClick={() => navigate(workflowRoute(item.id))}
      >
        <CardTitle>{data?.name}</CardTitle>
      </CardHeader>
      <CardContent
        className="cursor-pointer pt-0"
        onClick={() => navigate(workflowRoute(item.id))}
      >
        <WorkflowPreview workflow={data} />
      </CardContent>
      <CardFooter className="flex justify-end gap-2 pt-0">
        <Button variant="default" size="sm" onClick={handleRun}>
          <Play className="h-4 w-4 mr-1" />
          Run
        </Button>
        {deleteButton}
      </CardFooter>
    </Card>
  );
}

export default function WorkflowList() {
  const { data, isLoading } = api.useQuery("get", "/api/workflow");

  if (isLoading || data === undefined) {
    return (
      <main className="container mx-auto p-6">
        <h1 className="text-2xl font-bold mb-6">Workflows</h1>
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
          {[1, 2, 3].map((i) => (
            <SkeletonCard key={i} />
          ))}
        </div>
      </main>
    );
  }
  return (
    <main className="container mx-auto p-6">
      <h1 className="text-2xl font-bold mb-6">Workflows</h1>
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        {data.map((item) => (
          <WorkflowCard key={item.id} item={item} />
        ))}
      </div>
    </main>
  );
}
