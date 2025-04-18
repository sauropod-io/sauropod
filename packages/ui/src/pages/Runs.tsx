import type { Schemas } from "@sauropod-io/client";

import api from "@/api";
import PageHeader from "@/components/PageHeader";
import { ErrorBadge } from "@/components/badge";

function RunElement({ taskRun: taskRun }: { taskRun: Schemas["TaskRunInfo"] }) {
  return (
    <div className="py-1 border-b last:border-0">{JSON.stringify(taskRun)}</div>
  );
}

function TaskRunRows() {
  const { data, isLoading, error } = api.useQuery("get", "/api/task/run");
  if (error != null) {
    return (
      <div>
        <ErrorBadge>Error</ErrorBadge> Could not load rask runs:
        {` ${error}`}
      </div>
    );
  }

  if (isLoading) {
    return <div>Loading...</div>;
  }

  return (
    <>
      {data!.map((taskRun, index) => (
        <RunElement taskRun={taskRun} key={index} />
      ))}
    </>
  );
}

export default function Logs() {
  return (
    <main className="container mx-auto p-6">
      <PageHeader pageName="Task Runs" />
      <div className="bg-muted p-4 font-mono text-sm">
        <TaskRunRows />
      </div>
    </main>
  );
}
