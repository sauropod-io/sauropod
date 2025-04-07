import { Check, Copy } from "lucide-react";
import { Suspense, lazy, useState } from "react";

import type { SupportedLanguage } from "@/components/CodeBlock";
import { Button } from "@/components/ui/button";

export interface ExampleCodeBlockProps {
  children?: React.ReactNode;
  language: SupportedLanguage;
}

const CodeBlock = lazy(() => import("./CodeBlock"));

/** A code block used for examples.  */
export default function ExampleCodeBlock({
  children,
  language,
}: ExampleCodeBlockProps) {
  const [copied, setCopied] = useState(false);

  const handleCopy = async () => {
    if (typeof children === "string") {
      await navigator.clipboard.writeText(children);
      setCopied(true);
      setTimeout(() => setCopied(false), 2000); // Reset after 2 seconds
    }
  };

  return (
    <div className="relative group">
      <Suspense fallback={<pre>{children}</pre>}>
        <CodeBlock language={language}>{children as string}</CodeBlock>
      </Suspense>
      <Button
        variant="ghost"
        size="sm"
        className="absolute top-2 right-2 opacity-0 group-hover:opacity-100 transition-opacity"
        onClick={handleCopy}
        aria-label="Copy code to clipboard"
      >
        {copied ? <Check className="h-4 w-4" /> : <Copy className="h-4 w-4" />}
      </Button>
    </div>
  );
}
