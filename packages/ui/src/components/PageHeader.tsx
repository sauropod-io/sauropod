import { SidebarTrigger } from "@/components/ui/sidebar";

interface PageHeaderProps {
  pageName: string;
  children?: React.ReactNode;
}
export default function PageHeader({ pageName, children }: PageHeaderProps) {
  return (
    <div className="flex items-center justify-between mb-6">
      <div className="items-center flex flex-row">
        <SidebarTrigger className="mr-2" />
        <h1 className="text-2xl font-bold">{pageName}</h1>
      </div>
      <div>{children}</div>
    </div>
  );
}
