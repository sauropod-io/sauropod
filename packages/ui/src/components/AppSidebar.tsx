import { GitHubLogoIcon } from "@radix-ui/react-icons";
import { ChartNetwork, Globe, Logs, PencilRuler } from "lucide-react";
import { Link, useLocation } from "react-router";

import api from "@/api";
import SauropodIcon from "@/assets/icon.svg?url";
import Sauropod from "@/assets/sauropod.svg?url";
import {
  Sidebar as ShadcnSidebar,
  SidebarContent,
  SidebarFooter,
  SidebarGroup,
  SidebarGroupContent,
  SidebarGroupLabel,
  SidebarHeader,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
  SidebarRail,
  useSidebar,
} from "@/components/ui/sidebar";
import { LOGS, RUN_HISTORY, taskRoute } from "@/routes";

export default function AppSidebar() {
  const location = useLocation().pathname;
  const { open: sidebarOpen } = useSidebar();
  const { data: version } = api.useQuery("get", `/api/version`);

  return (
    <ShadcnSidebar className="border-green" collapsible="icon">
      <SidebarHeader className="items-start">
        <img
          src={sidebarOpen ? Sauropod : SauropodIcon}
          alt="Sauropod logo"
          className="mx-auto h-8"
        />
      </SidebarHeader>
      <SidebarContent className="group-data-[collapsible=icon]:!gap-0 group-data-[collapsible=icon]:mt-1">
        <SidebarGroup>
          <SidebarGroupLabel>Tasks</SidebarGroupLabel>
          <SidebarGroupContent>
            <SidebarMenu>
              <SidebarMenuItem>
                <SidebarMenuButton
                  isActive={location.startsWith("/task")}
                  asChild
                >
                  <Link to={taskRoute("")}>
                    <PencilRuler className="h-6 w-6" />
                    <span className="group-data-[collapsible=icon]:hidden">
                      Tasks
                    </span>
                  </Link>
                </SidebarMenuButton>
                <SidebarMenuButton isActive={location === RUN_HISTORY} asChild>
                  <Link to={RUN_HISTORY}>
                    <ChartNetwork className="h-6 w-6" />
                    <span className="group-data-[collapsible=icon]:hidden">
                      Runs
                    </span>
                  </Link>
                </SidebarMenuButton>
              </SidebarMenuItem>
            </SidebarMenu>
          </SidebarGroupContent>
        </SidebarGroup>

        <SidebarGroup className="mt-auto">
          <SidebarGroupLabel>System</SidebarGroupLabel>
          <SidebarGroupContent>
            <SidebarMenu>
              <SidebarMenuItem>
                <SidebarMenuButton isActive={location === LOGS} asChild>
                  <Link to={LOGS}>
                    <Logs className="h-6 w-6" />
                    <span className="group-data-[collapsible=icon]:hidden">
                      Logs
                    </span>
                  </Link>
                </SidebarMenuButton>
              </SidebarMenuItem>
              <SidebarMenuItem className="">
                <SidebarMenuButton isActive={false} asChild>
                  <a
                    href="https://github.com/sauropod-io/sauropod"
                    target="_blank"
                    rel="noopener noreferrer"
                  >
                    <GitHubLogoIcon className="h-6 w-6" />
                    <span className="group-data-[collapsible=icon]:hidden">
                      Source Code
                    </span>
                  </a>
                </SidebarMenuButton>
              </SidebarMenuItem>
              <SidebarMenuItem>
                <SidebarMenuButton isActive={false} asChild>
                  <a href="https://sauropod.io/" target="_blank" rel="noopener">
                    <Globe className="h-6 w-6" />
                    <span className="group-data-[collapsible=icon]:hidden">
                      Sauropod v{version || ""}
                    </span>
                  </a>
                </SidebarMenuButton>
              </SidebarMenuItem>
            </SidebarMenu>
          </SidebarGroupContent>
        </SidebarGroup>
      </SidebarContent>
      <SidebarFooter />
      <SidebarRail />
    </ShadcnSidebar>
  );
}
