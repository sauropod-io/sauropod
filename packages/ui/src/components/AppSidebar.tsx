import { GitHubLogoIcon } from "@radix-ui/react-icons";
import { Logs, Settings } from "lucide-react";
import { Link, useLocation } from "react-router";

import SauropodIcon from "@/assets/icon.svg";
import Sauropod from "@/assets/sauropod.svg";
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
import { cn } from "@/lib/utils";
import { LOGS, SETTINGS } from "@/routes";

export default function AppSidebar() {
  const location = useLocation().pathname;
  const { open: sidebarOpen } = useSidebar();

  return (
    <ShadcnSidebar collapsible="icon">
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
                  <Link
                    to={LOGS}
                    className={cn(
                      "text-sm font-medium text-muted-foreground transition-colors hover:text-primary",
                      location === LOGS ? "text-primary" : "",
                    )}
                  >
                    <Logs className="h-6 w-6" />
                    <span>Logs</span>
                  </Link>
                </SidebarMenuButton>
              </SidebarMenuItem>
              <SidebarMenuItem>
                <SidebarMenuButton isActive={location === SETTINGS} asChild>
                  <Link
                    to={SETTINGS}
                    className={cn(
                      "text-sm font-medium text-muted-foreground transition-colors hover:text-primary",
                      location === SETTINGS ? "text-primary" : "",
                    )}
                  >
                    <Settings className="h-6 w-6" />
                    <span>Settings</span>
                  </Link>
                </SidebarMenuButton>
              </SidebarMenuItem>
              <SidebarMenuItem className="text-muted-foreground">
                <SidebarMenuButton isActive={false} asChild>
                  <a
                    href="https://github.com/sauropod-io/sauropod"
                    target="_blank"
                    rel="noopener noreferrer"
                    className="hover:text-primary"
                  >
                    <GitHubLogoIcon className="h-6 w-6" />
                    <span>Source Code</span>
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
