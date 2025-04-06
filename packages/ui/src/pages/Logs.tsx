import { format } from "date-fns";

import type { Schemas } from "@sauropod-io/client";

import api from "@/api";
import { ErrorBadge, Level, LevelBadge } from "@/components/badge";

function LogElement({
  log,
  index,
}: {
  log: Schemas["LogMessage"];
  index: number;
}) {
  const { message, ...rest } = log.fields;
  return (
    <div key={index} className="py-1 border-b last:border-0">
      <div className="flex items-start">
        <span className="text-muted-foreground mr-2">
          {log.timestampS != undefined
            ? format(new Date(log.timestampS * 1000), "yyyy-MM-dd HH:mm:ss")
            : ""}
        </span>
        <div className="min-w-16 mr-2">
          <LevelBadge level={log.level as Level}>
            {log.level.toUpperCase()}
          </LevelBadge>
        </div>
        <span>{message ? `${message} ` : ""}</span>
      </div>
      {Object.keys(rest).length !== 0 && (
        <div className="ml-[180px] text-xs text-muted-foreground">
          <span className="mr-2">{JSON.stringify(rest)}</span>
        </div>
      )}
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
        <LogElement log={log} index={index} key={index} />
      ))}
    </>
  );
}

export default function Logs() {
  return (
    <main className="container mx-auto border rounded-b-lg overflow-hidden overflow-y-auto">
      <div className="bg-muted p-4 font-mono text-sm">
        <LogMessageRows />
      </div>
    </main>
  );
}
