import { JSX } from "react";

import Editor, { OnChange, OnValidate } from "@monaco-editor/react";

const EDITOR_CLASS =
  "min-h-[10em] resize-none border-input placeholder:text-muted-foreground focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:bg-input/30 field-sizing-content rounded-md border bg-transparent px-3 py-2 text-base shadow-xs transition-[color,box-shadow] outline-none focus-visible:ring-[3px] disabled:cursor-not-allowed disabled:opacity-50 md:text-md";

export interface PromptEditorProps {
  value: string;
  onChange?: OnChange;
  onValidate?: OnValidate;
}

/** Text editor for model prompts. */
export default function PromptEditor({
  value,
  onChange,
  onValidate,
}: PromptEditorProps): JSX.Element {
  return (
    <Editor
      defaultLanguage="markdown"
      value={value}
      className={EDITOR_CLASS}
      onChange={onChange}
      onValidate={onValidate}
      path="prompt.md"
      options={{
        lineNumbers: "off",
        minimap: { enabled: false },
        scrollBeyondLastLine: false,
        wordWrap: "on",
        wrappingIndent: "same",
        wrappingStrategy: "advanced",
        roundedSelection: true,
        copyWithSyntaxHighlighting: false,
        renderLineHighlight: "none",
        selectionHighlight: false,
        suggest: {
          showWords: false,
        },
        occurrencesHighlight: "off",
      }}
    />
  );
}
