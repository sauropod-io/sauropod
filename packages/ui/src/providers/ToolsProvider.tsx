import { createContext } from "react";

import { Schemas } from "@sauropod-io/client";

import api from "@/api";

const ToolContext = createContext<Schemas["ToolDefinition"][]>([]);

export default function ToolProvider({
  children,
}: {
  children: React.ReactNode;
}) {
  const { data } = api.useQuery("get", "/api/tools");

  return (
    <ToolContext.Provider value={data || []}>{children}</ToolContext.Provider>
  );
}

/** Consumer for the tool list. */
export const ToolConsumer = ToolContext.Consumer;
