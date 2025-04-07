import { PrismLight as SyntaxHighlighter } from "react-syntax-highlighter";
import bash from "react-syntax-highlighter/dist/esm/languages/prism/bash";
import python from "react-syntax-highlighter/dist/esm/languages/prism/python";
import rust from "react-syntax-highlighter/dist/esm/languages/prism/rust";
import typescript from "react-syntax-highlighter/dist/esm/languages/prism/typescript";
import style from "react-syntax-highlighter/dist/esm/styles/prism/ghcolors";

export type SupportedLanguage = "python" | "typescript" | "rust" | "bash";
export interface CodeBlock {
  children?: string | string[];
  language: SupportedLanguage;
}

/** A code block with syntax highlighting.  */
export default function CodeBlock({ children, language }: CodeBlock) {
  return (
    <SyntaxHighlighter language={language} style={style}>
      {children as string}
    </SyntaxHighlighter>
  );
}

SyntaxHighlighter.registerLanguage("typescript", typescript);
SyntaxHighlighter.registerLanguage("python", python);
SyntaxHighlighter.registerLanguage("rust", rust);
SyntaxHighlighter.registerLanguage("bash", bash);
