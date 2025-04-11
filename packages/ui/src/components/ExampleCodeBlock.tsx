import { Check, ChevronDown, Copy } from "lucide-react";
import { Suspense, lazy, useMemo, useState } from "react";

import type { SupportedLanguage } from "@/components/CodeBlock";
import { Button } from "@/components/ui/button";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";

const CodeBlock = lazy(() => import("./CodeBlock"));

export interface ExampleCodeBlockProps {
  language: SupportedLanguage;
  /** A mapping from the library of the example to the content for that library. */
  examples: Record<string, string>;
}

/** A code block used for examples.  */
export default function ExampleCodeBlock({
  language,
  examples,
}: ExampleCodeBlockProps) {
  const [copied, setCopied] = useState(false);

  // Get library names and set default selection
  const libraryNames = useMemo(() => Object.keys(examples), [examples]);
  const [selectedLibrary, setSelectedLibrary] = useState<string>(
    libraryNames.length > 0 ? libraryNames[0] : "",
  );

  // Get the current code to display based on selected library
  const currentCode = selectedLibrary ? examples[selectedLibrary] : "";

  const handleCopy = async () => {
    await navigator.clipboard.writeText(currentCode);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000); // Reset after 2 seconds
  };

  return (
    <div className="relative group">
      <Suspense fallback={<pre>{currentCode}</pre>}>
        <CodeBlock language={language}>{currentCode}</CodeBlock>
      </Suspense>
      <div className="flex justify-end mb-2 gap-2">
        {libraryNames.length > 1 && (
          <DropdownMenu>
            <DropdownMenuTrigger asChild>
              <Button variant="outline" size="sm" className="flex gap-1">
                {selectedLibrary}
                <ChevronDown className="h-4 w-4" />
              </Button>
            </DropdownMenuTrigger>
            <DropdownMenuContent align="end" className="z-50">
              {libraryNames.map((lib) => (
                <DropdownMenuItem
                  key={lib}
                  onClick={() => setSelectedLibrary(lib)}
                >
                  {lib}
                </DropdownMenuItem>
              ))}
            </DropdownMenuContent>
          </DropdownMenu>
        )}
        <Button
          variant="ghost"
          size="sm"
          onClick={handleCopy}
          aria-label="Copy code to clipboard"
        >
          {copied ? (
            <Check className="h-4 w-4" />
          ) : (
            <Copy className="h-4 w-4" />
          )}
        </Button>
      </div>
    </div>
  );
}
