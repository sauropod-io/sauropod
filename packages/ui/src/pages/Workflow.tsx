import { useParams } from "react-router";

import WorkflowEditor from "@/components/WorkflowEditor";

export default function Workflow() {
  const match = useParams();
  return <WorkflowEditor workflowId={match.id} />;
}
