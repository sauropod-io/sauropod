import { Plus } from "lucide-react";
import { useNavigate } from "react-router";

import { Schemas } from "@sauropod-io/client";

import api from "@/api";
import ErrorCard from "@/components/ErrorCard";
import PageHeader from "@/components/PageHeader";
import DeleteButton from "@/components/buttons/DeleteButton";
import RunButton from "@/components/buttons/RunButton";
import { Button } from "@/components/ui/button";
import {
  Card,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { SidebarTrigger } from "@/components/ui/sidebar";
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

  const handleRun = (e: React.MouseEvent) => {
    e.stopPropagation(); // Prevent default card navigation
    navigate(`${taskRoute(item.id)}?run=true`);
  };

  if (isLoading) {
    return <SkeletonCard />;
  }

  const deleteButton = (
    <DeleteButton variant="destructive" size="sm" onClick={handleDelete} />
  );

  if (error != null) {
    return (
      <ErrorCard
        className="relative group"
        message={`${item.name}`}
        error={error}
        buttons={[deleteButton]}
      />
    );
  }

  return (
    <Card
      onClick={() => navigate(taskRoute(item.id))}
      className="relative group"
    >
      <CardHeader>
        <CardTitle>{data?.name}</CardTitle>
        <CardDescription>{data?.template.substring(0, 64)}</CardDescription>
      </CardHeader>
      <CardFooter className="flex justify-end gap-2 pt-0">
        <RunButton variant="default" size="sm" onClick={handleRun} />
        {deleteButton}
      </CardFooter>
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
          <SidebarTrigger />
          <h1 className="text-2xl font-bold">Tasks</h1>
          <Button
            variant="outline"
            onClick={() => navigate(taskRoute("new"))}
            size="sm"
          >
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
      <PageHeader pageName="Tasks">
        <Button
          variant="outline"
          onClick={() => navigate(taskRoute("new"))}
          size="sm"
        >
          <Plus className="h-4 w-4 mr-1" />
          Create Task
        </Button>
      </PageHeader>
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        {data.length === 0 ? (
          <Card
            onClick={() => navigate(taskRoute("new"))}
            className="cursor-pointer border-dashed"
          >
            <CardHeader className="text-center">
              <CardTitle className="flex items-center justify-center gap-2">
                <Plus className="h-5 w-5" />
                Create your first task
              </CardTitle>
              <CardDescription>Get started by creating a task</CardDescription>
            </CardHeader>
          </Card>
        ) : (
          data.map((item) => <TaskCard key={item.id} item={item} />)
        )}
      </div>
    </main>
  );
}
