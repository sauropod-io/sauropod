import { Waypoints } from "lucide-react";

import { workflowRoute, WORKFLOW_PREFIX } from "@/routes";
import api from "@/api";
import SidebarLinkGroup from "./SidebarLinkGroup";

export default function SidebarWorkflowGroup() {
  const { data, isFetching, isLoading, error } = api.useQuery(
    "get",
    "/api/workflow",
  );
  return (
    <SidebarLinkGroup
      items={data}
      error={error}
      isFetching={isFetching}
      isLoading={isLoading}
      label="Workflows"
      labelIcon={Waypoints}
      linkPrefix={WORKFLOW_PREFIX}
      linkRoute={workflowRoute}
      addActionTitle="Add Workflow"
    />
  );
}
