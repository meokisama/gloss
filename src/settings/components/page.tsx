import type { ReactNode } from "react";

export function Page({ title, description, children }: { title: string; description: string; children: ReactNode }) {
  return (
    <div className="mx-auto flex max-w-xl flex-col gap-6 px-8 pb-10 pt-7">
      <header>
        <h1 className="text-lg font-semibold tracking-tight">{title}</h1>
        <p className="mt-0.5 text-sm text-muted">{description}</p>
      </header>
      {children}
    </div>
  );
}
