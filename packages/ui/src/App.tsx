import { Suspense, lazy } from "react";
import { Navigate, Route, Routes } from "react-router";

import AppSidebar from "@/components/AppSidebar";
import {
  SidebarInset,
  SidebarProvider,
  SidebarTrigger,
} from "@/components/ui/sidebar";
import { INDEX, LOGS, taskRoute } from "@/routes";

const Logs = lazy(() => import("@/pages/Logs"));
const Task = lazy(() => import("@/pages/Task"));
const TaskList = lazy(() => import("@/pages/TaskList"));

/** Suspense wrapper. */
function SupportLoading({ children }: { children: React.ReactNode }) {
  return <Suspense fallback={<div>Loading page...</div>}>{children}</Suspense>;
}

function PageContent() {
  return (
    <Routes>
      <Route path="*" element={<div>Not found</div>} />
      <Route path={INDEX} element={<Navigate to={taskRoute("")} />} />
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
        path={LOGS}
        element={
          <SupportLoading>
            <Logs />
          </SupportLoading>
        }
      />
    </Routes>
  );
}

function App() {
  return (
    <div className="flex h-screen w-full">
      <SidebarProvider defaultOpen={true}>
        <AppSidebar />
        <SidebarInset className="bg-slate-50 flex-1 overflow-auto">
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
