import { useNavigate } from "react-router";

import { Schemas } from "@sauropod-io/client";

import api from "@/api";
import {
  Card,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Skeleton } from "@/components/ui/skeleton";
import { workflowRoute } from "@/routes";

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
  const { data, isLoading } = api.useQuery("get", `/api/workflow/{id}`, {
    params: { path: { id: `${item.id}` } },
  });
  const navigate = useNavigate();

  if (isLoading) {
    return <SkeletonCard />;
  }

  return (
    <Card onClick={() => navigate(workflowRoute(item.id))}>
      <CardHeader>
        <CardTitle>{data?.name}</CardTitle>
        <CardDescription>TODO add a description</CardDescription>
      </CardHeader>
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
