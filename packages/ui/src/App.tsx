import { Navigate, Route, Routes } from "react-router";

import AppSidebar from "@/components/AppSidebar";
import {
  SidebarInset,
  SidebarProvider,
  SidebarTrigger,
} from "@/components/ui/sidebar";
import Logs from "@/pages/Logs";
import Settings from "@/pages/Settings";
import Task from "@/pages/Task";
import TaskList from "@/pages/TaskList";
import Workflow from "@/pages/Workflow";
import WorkflowList from "@/pages/WorkflowList";
import { INDEX, LOGS, SETTINGS, taskRoute, workflowRoute } from "@/routes";

function PageContent() {
  return (
    <Routes>
      <Route path="*" element={<div>Not found</div>} />
      <Route path={INDEX} element={<Navigate to={workflowRoute("")} />} />
      <Route path={taskRoute("new")} element={<Task />} />
      <Route path={taskRoute(":id")} element={<Task />} />
      <Route path={taskRoute("")} element={<TaskList />} />
      <Route path={workflowRoute("new")} element={<Workflow />} />
      <Route path={workflowRoute(":id")} element={<Workflow />} />
      <Route path={workflowRoute("")} element={<WorkflowList />} />
      <Route path={LOGS} element={<Logs />} />
      <Route path={SETTINGS} element={<Settings />} />
    </Routes>
  );
}

function App() {
  return (
    <div className="flex  h-screen w-full bg-background">
      <SidebarProvider defaultOpen={true}>
        <AppSidebar />
        <SidebarInset className="flex-1 overflow-auto">
          <div className="p-2 md:hidden">
            <SidebarTrigger className="mb-2" />
          </div>
          <PageContent />
        </SidebarInset>
      </SidebarProvider>
    </div>
  );
}

export default App;
