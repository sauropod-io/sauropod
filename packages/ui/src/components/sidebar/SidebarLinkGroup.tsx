import { LucideIcon, PlusCircle } from "lucide-react";
import { Link, useLocation, useNavigate } from "react-router";

import type { Schemas } from "@sauropod-io/client";

import { ErrorBadge } from "@/components/badge";
import Spinner from "@/components/icons/Spinner";
import {
  SidebarGroup,
  SidebarGroupAction,
  SidebarGroupContent,
  SidebarGroupLabel,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
  SidebarMenuSkeleton,
} from "@/components/ui/sidebar";

interface SidebarLinkGroupProps {
  items?: { id: number; name: string }[];
  label: string;
  labelIcon: LucideIcon;
  linkPrefix: string;
  linkRoute: (id: string) => string;
  addActionTitle: string;
  isFetching: boolean;
  isLoading: boolean;
  error: Schemas["Error"] | null;
}

/** A link group. */
export default function SidebarLinkGroup({
  items,
  label,
  labelIcon: LabelIcon,
  linkPrefix,
  linkRoute,
  addActionTitle,
  isFetching,
  isLoading,
  error,
}: SidebarLinkGroupProps) {
  const navigate = useNavigate();
  const location = useLocation().pathname;

  let selectedItemId = null;
  if (location.startsWith(linkPrefix)) {
    selectedItemId = location.substring(linkPrefix.length);
  }

  let menuItems = [];
  if (error != null) {
    menuItems = [
      <SidebarMenuItem>
        <ErrorBadge>Error</ErrorBadge> {error.error}
      </SidebarMenuItem>,
    ];
  } else if (!isLoading) {
    menuItems = items!.map((item) => {
      const id = `${item.id}`;
      return (
        <SidebarMenuItem
          className="group-data-[collapsible=icon]:hidden"
          key={id}
        >
          <SidebarMenuButton isActive={selectedItemId === id} asChild={true}>
            <Link to={linkRoute(id)}>{item.name}</Link>
          </SidebarMenuButton>
        </SidebarMenuItem>
      );
    });
  } else {
    // Add skeleton items when initial data is loading
    for (let i = 0; i < 3; i++) {
      menuItems.push(
        <SidebarMenuItem
          className="group-data-[collapsible=icon]:hidden"
          key={i}
        >
          <SidebarMenuSkeleton />
        </SidebarMenuItem>,
      );
    }
  }

  // For background fetches, show a spinner instead of the label icon
  const Icon = isFetching ? Spinner : LabelIcon;

  return (
    <SidebarGroup className="group-data-[collapsible=icon]:!py-0">
      <SidebarGroupLabel
        hideWhenClosed={false}
        className="group-data-[collapsible=icon]:-p-0 text-foreground"
        asChild
      >
        <Link
          to={linkRoute("")}
          className="flex shrink-0 items-center text-xs [&>svg]:size-4 [&>svg]:shrink-0"
        >
          <Icon className="h-6 w-6" />
          <span className="mx-1 group-data-[collapsible=icon]:hidden">
            {label}
          </span>
        </Link>
      </SidebarGroupLabel>
      <SidebarGroupAction
        title={addActionTitle}
        className="hover:bg-ring"
        onClick={() => navigate(linkRoute("new"))}
      >
        <PlusCircle className="cursor-pointer" />{" "}
        <span className="sr-only">{addActionTitle}</span>
      </SidebarGroupAction>
      <SidebarGroupContent>
        <SidebarMenu>
          <hr className="border-green mx-1 group-data-[collapsible=icon]:hidden" />
          {menuItems}
        </SidebarMenu>
      </SidebarGroupContent>
    </SidebarGroup>
  );
}
