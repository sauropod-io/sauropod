import { format } from "date-fns";

import type { Schemas } from "@sauropod-io/client";

import api from "@/api";
import PageHeader from "@/components/PageHeader";
import { ErrorBadge, Level, LevelBadge } from "@/components/badge";

function LogElement({ log }: { log: Schemas["LogMessage"] }) {
  const { message, ...rest } = log.fields;
  return (
    <div className="py-1 border-b last:border-0">
      <div className="flex items-start">
        <div className="min-w-42 text-muted-foreground">
          {log.timestampMs != undefined
            ? format(new Date(log.timestampMs), "yyyy-MM-dd HH:mm:ss")
            : ""}
        </div>
        <div className="min-w-16 mr-2">
          <LevelBadge level={log.level as Level}>
            {log.level.toUpperCase()}
          </LevelBadge>
        </div>
        <div className="flex flex-col w-[calc(100%-15rem)]">
          <div className="overflow-auto truncate">
            {message ? `${message} ` : ""}
          </div>
          {Object.keys(rest).length !== 0 && (
            <div className="overflow-auto truncate text-xs text-muted-foreground">
              {JSON.stringify(rest)}
            </div>
          )}
        </div>
      </div>
    </div>
  );
}

function LogMessageRows() {
  const { data, isLoading, error } = api.useQuery(
    "get",
    "/api/observability/logs",
  );
  if (error != null) {
    return (
      <div>
        <ErrorBadge>Error</ErrorBadge> Could not load system logs:
        {` ${error.error}`}
      </div>
    );
  }

  if (isLoading) {
    return <div>Loading...</div>;
  }

  return (
    <>
      {data!.map((log, index) => (
        <LogElement log={log} key={index} />
      ))}
    </>
  );
}

export default function Logs() {
  return (
    <main className="container mx-auto p-6">
      <PageHeader pageName="Logs" />
      <div className="bg-muted p-4 font-mono text-sm">
        <LogMessageRows />
      </div>
    </main>
  );
}
