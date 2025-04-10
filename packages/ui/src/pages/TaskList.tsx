// Import X icon for delete button
import { Plus, X } from "lucide-react";
import { useNavigate } from "react-router";

import { Schemas } from "@sauropod-io/client";

import api from "@/api";
import ErrorCard from "@/components/ErrorCard";
import { Button } from "@/components/ui/button";
import {
  Card,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Skeleton } from "@/components/ui/skeleton";
import { useDeleteTask } from "@/mutations/taskMutations";
import { taskRoute } from "@/routes";

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

function TaskCard({ item }: { item: Schemas["ObjectInfo"] }) {
  const { data, isLoading, error } = api.useQuery("get", `/api/task/{id}`, {
    params: { path: { id: `${item.id}` } },
  });
  const navigate = useNavigate();

  const deleteTask = useDeleteTask();
  const handleDelete = async (e: React.MouseEvent) => {
    e.stopPropagation(); // Prevent navigation when clicking delete button
    deleteTask.mutate({
      params: { path: { id: `${item.id}` } },
    });
  };

  if (isLoading) {
    return <SkeletonCard />;
  }

  if (error != null) {
    return (
      <ErrorCard
        className="relative group"
        message={`${item.name}`}
        error={error}
      />
    );
  }

  return (
    <Card
      onClick={() => navigate(taskRoute(item.id))}
      className="relative group"
    >
      <Button
        variant="destructive"
        size="icon"
        className="absolute top-2 right-2 opacity-0 group-hover:opacity-100 transition-opacity"
        onClick={handleDelete}
      >
        <X className="h-4 w-4" />
      </Button>
      <CardHeader>
        <CardTitle>{data?.name}</CardTitle>
        <CardDescription>
          {data?.action.invokeLLM.template.substring(0, 64)}
        </CardDescription>
      </CardHeader>
    </Card>
  );
}

export default function TaskList() {
  const { data, isLoading } = api.useQuery("get", "/api/task");
  const navigate = useNavigate();

  if (isLoading || data === undefined) {
    return (
      <main className="container mx-auto p-6">
        <div className="flex items-center justify-between mb-6">
          <h1 className="text-2xl font-bold">Tasks</h1>
          <Button onClick={() => navigate(taskRoute("new"))} size="sm">
            <Plus className="h-4 w-4 mr-1" />
            Create Task
          </Button>
        </div>
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
      <div className="flex items-center justify-between mb-6">
        <h1 className="text-2xl font-bold">Tasks</h1>
        <Button onClick={() => navigate(taskRoute("new"))} size="sm">
          <Plus className="h-4 w-4 mr-1" />
          Create Task
        </Button>
      </div>
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        {data.map((item) => (
          <TaskCard key={item.id} item={item} />
        ))}
      </div>
    </main>
  );
}
