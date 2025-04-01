import { useParams } from "react-router";

import TaskEditor from "@/components/TaskEditor";

export default function Task() {
  const match = useParams();
  return <TaskEditor taskId={match.id} />;
}
