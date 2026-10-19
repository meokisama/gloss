import type { ReactNode } from "react";

export function Group({ title, children }: { title?: string; children: ReactNode }) {
  return (
    <section className="flex flex-col gap-2">
      {title && <h2 className="px-1 text-xs font-medium text-muted">{title}</h2>}
      <div className="divide-y divide-line rounded-lg border border-line bg-field">{children}</div>
    </section>
  );
}
