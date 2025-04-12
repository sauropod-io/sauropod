import { GitHubLogoIcon } from "@radix-ui/react-icons";
import { Logs } from "lucide-react";
import { Link, useLocation } from "react-router";

import api from "@/api";
import SauropodIcon from "@/assets/icon.svg?url";
import Sauropod from "@/assets/sauropod.svg?url";
import SidebarTaskGroup from "@/components/sidebar/SidebarTaskGroup";
import SidebarWorkflowGroup from "@/components/sidebar/SidebarWorkflowGroup";
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
import { LOGS } from "@/routes";

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
        <SidebarTaskGroup />
        <SidebarWorkflowGroup />
        <SidebarGroup className="mt-auto">
          <SidebarGroupLabel>System</SidebarGroupLabel>
          <SidebarGroupContent>
            <SidebarMenu>
              <SidebarMenuItem>
                <SidebarMenuButton isActive={location === LOGS} asChild>
                  <Link to={LOGS}>
                    <Logs className="h-6 w-6" />
                    <span>Logs</span>
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
                    <span>Source Code</span>
                  </a>
                </SidebarMenuButton>
              </SidebarMenuItem>
              <SidebarMenuItem>
                <SidebarMenuButton isActive={false} asChild>
                  <a href="https://sauropod.io/" target="_blank" rel="noopener">
                    <span>Sauropod v{version || ""}</span>
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
