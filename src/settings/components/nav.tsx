import type { LucideIcon } from "lucide-react";
import { Tabs } from "radix-ui";

export interface NavPage {
  id: string;
  label: string;
  icon: LucideIcon;
}

export function Nav({ pages }: { pages: NavPage[] }) {
  return (
    <Tabs.List aria-label="Settings" className="flex w-44 shrink-0 flex-col gap-0.5 px-2 pt-1">
      {pages.map(({ id, label, icon: Icon }) => (
        <Tabs.Trigger
          key={id}
          value={id}
          className="focus-ring flex h-8 items-center gap-2.5 rounded-md px-2.5 text-sm text-muted transition-colors hover:text-ink data-[state=active]:bg-hover data-[state=active]:text-ink data-[state=active]:[&_svg]:text-accent"
        >
          <Icon className="size-4" strokeWidth={1.75} />
          {label}
        </Tabs.Trigger>
      ))}
    </Tabs.List>
  );
}
