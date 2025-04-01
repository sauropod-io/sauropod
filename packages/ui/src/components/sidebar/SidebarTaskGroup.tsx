import { PencilRuler } from "lucide-react";

import { taskRoute, TASK_PREFIX } from "@/routes";
import api from "@/api";
import SidebarLinkGroup from "./SidebarLinkGroup";

export default function SidebarTaskGroup() {
  const { data, isFetching, isLoading, error } = api.useQuery(
    "get",
    "/api/task",
  );
  return (
    <SidebarLinkGroup
      items={data}
      error={error}
      isFetching={isFetching}
      isLoading={isLoading}
      label="Tasks"
      labelIcon={PencilRuler}
      linkPrefix={TASK_PREFIX}
      linkRoute={taskRoute}
      addActionTitle="Add Task"
    />
  );
}
