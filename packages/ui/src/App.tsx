import { Suspense, lazy } from "react";
import { Navigate, Route, Routes } from "react-router";

import AppSidebar from "@/components/AppSidebar";
import {
  SidebarInset,
  SidebarProvider,
  SidebarTrigger,
} from "@/components/ui/sidebar";
import { INDEX, LOGS, SETTINGS, taskRoute, workflowRoute } from "@/routes";

const Logs = lazy(() => import("@/pages/Logs"));
const Settings = lazy(() => import("@/pages/Settings"));
const Task = lazy(() => import("@/pages/Task"));
const TaskList = lazy(() => import("@/pages/TaskList"));
const Workflow = lazy(() => import("@/pages/Workflow"));
const WorkflowList = lazy(() => import("@/pages/WorkflowList"));

/** Suspense wrapper. */
function SupportLoading({ children }: { children: React.ReactNode }) {
  return <Suspense fallback={<div>Loading page...</div>}>{children}</Suspense>;
}

function PageContent() {
  return (
    <Routes>
      <Route path="*" element={<div>Not found</div>} />
      <Route path={INDEX} element={<Navigate to={workflowRoute("")} />} />
      <Route
        path={taskRoute("new")}
        element={
          <SupportLoading>
            <Task />
          </SupportLoading>
        }
      />
      <Route
        path={taskRoute(":id")}
        element={
          <SupportLoading>
            <Task />
          </SupportLoading>
        }
      />
      <Route
        path={taskRoute("")}
        element={
          <SupportLoading>
            <TaskList />
          </SupportLoading>
        }
      />
      <Route
        path={workflowRoute("new")}
        element={
          <SupportLoading>
            <Workflow />
          </SupportLoading>
        }
      />
      <Route
        path={workflowRoute(":id")}
        element={
          <SupportLoading>
            <Workflow />
          </SupportLoading>
        }
      />
      <Route
        path={workflowRoute("")}
        element={
          <SupportLoading>
            <WorkflowList />
          </SupportLoading>
        }
      />
      <Route
        path={LOGS}
        element={
          <SupportLoading>
            <Logs />
          </SupportLoading>
        }
      />
      <Route
        path={SETTINGS}
        element={
          <SupportLoading>
            <Settings />
          </SupportLoading>
        }
      />
    </Routes>
  );
}

function App() {
  return (
    <div className="flex h-screen w-full bg-background">
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
