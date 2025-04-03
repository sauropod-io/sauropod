import { Waypoints } from "lucide-react";

import api from "@/api";
import { WORKFLOW_PREFIX, workflowRoute } from "@/routes";

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
