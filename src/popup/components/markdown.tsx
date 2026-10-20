import ReactMarkdown, { type Components } from "react-markdown";

/** Links are left out: a click would navigate the popup itself, and no opener plugin is installed. */
const ALLOWED = ["p", "strong", "em", "ul", "ol", "li", "code", "pre"];

const COMPONENTS: Components = {
  p: ({ children }) => <p className="mb-2 last:mb-0">{children}</p>,
  strong: ({ children }) => <strong className="font-semibold">{children}</strong>,
  ul: ({ children }) => <ul className="mb-2 list-disc pl-5 last:mb-0">{children}</ul>,
  ol: ({ children }) => <ol className="mb-2 list-decimal pl-5 last:mb-0">{children}</ol>,
  li: ({ children }) => <li className="mb-0.5">{children}</li>,
  pre: ({ children }) => <pre className="mb-2 overflow-x-auto rounded-md bg-hover p-2 last:mb-0 [&_code]:bg-transparent [&_code]:p-0">{children}</pre>,
  code: ({ children }) => <code className="rounded-sm bg-hover px-1 font-mono text-[0.9em]">{children}</code>,
};

export function Markdown({ text }: { text: string }) {
  return (
    <div className="leading-relaxed wrap-break-word select-text">
      <ReactMarkdown allowedElements={ALLOWED} unwrapDisallowed components={COMPONENTS}>
        {text}
      </ReactMarkdown>
    </div>
  );
}
