import { CodeNode } from "@lexical/code";
import { LinkNode } from "@lexical/link";
import { ListItemNode, ListNode } from "@lexical/list";
import {
  $convertFromMarkdownString,
  $convertToMarkdownString,
  TRANSFORMERS,
} from "@lexical/markdown";
import { AutoFocusPlugin } from "@lexical/react/LexicalAutoFocusPlugin";
import {
  InitialConfigType,
  LexicalComposer,
} from "@lexical/react/LexicalComposer";
import { useLexicalComposerContext } from "@lexical/react/LexicalComposerContext";
import { ContentEditable } from "@lexical/react/LexicalContentEditable";
import { LexicalErrorBoundary } from "@lexical/react/LexicalErrorBoundary";
import { HistoryPlugin } from "@lexical/react/LexicalHistoryPlugin";
import { MarkdownShortcutPlugin } from "@lexical/react/LexicalMarkdownShortcutPlugin";
import { RichTextPlugin } from "@lexical/react/LexicalRichTextPlugin";
import { HeadingNode, QuoteNode } from "@lexical/rich-text";
import { EditorThemeClasses } from "lexical";
import { JSX, useEffect } from "react";

import "@/components/PromptEditor.css";

const theme: EditorThemeClasses = {
  ltr: "ltr",
  rtl: "rtl",
  paragraph: "editor-paragraph",
  quote: "editor-quote",
  heading: {
    h1: "editor-heading-h1",
    h2: "editor-heading-h2",
    h3: "editor-heading-h3",
    h4: "editor-heading-h4",
    h5: "editor-heading-h5",
    h6: "editor-heading-h6",
  },
  list: {
    nested: {
      listitem: "editor-nested-listitem",
    },
    ol: "editor-list-ol",
    ul: "editor-list-ul",
    listitem: "editor-listItem",
    listitemChecked: "editor-listItemChecked",
    listitemUnchecked: "editor-listItemUnchecked",
  },
  hashtag: "editor-hashtag",
  image: "editor-image",
  link: "editor-link",
  text: {
    bold: "editor-textBold",
    code: "editor-textCode",
    italic: "editor-textItalic",
    strikethrough: "editor-textStrikethrough",
    subscript: "editor-textSubscript",
    superscript: "editor-textSuperscript",
    underline: "editor-textUnderline",
    underlineStrikethrough: "editor-textUnderlineStrikethrough",
  },
  code: "editor-code",
  codeHighlight: {
    atrule: "editor-tokenAttr",
    attr: "editor-tokenAttr",
    boolean: "editor-tokenProperty",
    builtin: "editor-tokenSelector",
    cdata: "editor-tokenComment",
    char: "editor-tokenSelector",
    class: "editor-tokenFunction",
    "class-name": "editor-tokenFunction",
    comment: "editor-tokenComment",
    constant: "editor-tokenProperty",
    deleted: "editor-tokenProperty",
    doctype: "editor-tokenComment",
    entity: "editor-tokenOperator",
    function: "editor-tokenFunction",
    important: "editor-tokenVariable",
    inserted: "editor-tokenSelector",
    keyword: "editor-tokenAttr",
    namespace: "editor-tokenVariable",
    number: "editor-tokenProperty",
    operator: "editor-tokenOperator",
    prolog: "editor-tokenComment",
    property: "editor-tokenProperty",
    punctuation: "editor-tokenPunctuation",
    regex: "editor-tokenVariable",
    selector: "editor-tokenSelector",
    string: "editor-tokenSelector",
    symbol: "editor-tokenProperty",
    tag: "editor-tokenProperty",
    url: "editor-tokenOperator",
    variable: "editor-tokenVariable",
  },
};

/** Log errors. */
function onError(error: Error): void {
  console.error(error);
}

export interface PromptEditorProps {
  initialValue: string;
  editable?: boolean;
  onChange?: (text: string) => void;
}

const editorClasses =
  "w-full h-full placeholder:text-muted-foreground selection:bg-primary selection:text-primary-foreground dark:bg-input/30 border-input rounded-md border bg-transparent px-3 py-1 text-base shadow-xs md:text-sm focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px] aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive";

function InputOutputPlugin({
  onChange,
  initialValue,
}: {
  onChange: (markdown: string) => void;
  initialValue: string;
}) {
  const [editor] = useLexicalComposerContext();

  useEffect(() => {
    return editor.registerUpdateListener(() => {
      editor.read(() => {
        const markdown = $convertToMarkdownString(TRANSFORMERS);
        onChange(markdown);
      });
    });
  }, [editor, onChange]);

  useEffect(() => {
    editor.update(() => {
      $convertFromMarkdownString(initialValue, TRANSFORMERS);
    });
  }, [editor, initialValue]);

  if (!onChange) {
    return null;
  }
  return null;
}

/** Text editor for model prompts. */
export default function PromptEditor({
  initialValue,
  onChange,
  editable,
}: PromptEditorProps): JSX.Element {
  const initialConfig: InitialConfigType = {
    namespace: "PromptEditor",
    theme,
    onError,
    editorState: () => $convertFromMarkdownString(initialValue, TRANSFORMERS),
    editable,
    nodes: [ListNode, ListItemNode, QuoteNode, CodeNode, HeadingNode, LinkNode],
  };

  function emitChange(markdown: string) {
    if (!onChange) {
      return;
    }

    onChange(markdown);
  }

  return (
    <LexicalComposer initialConfig={initialConfig}>
      <RichTextPlugin
        contentEditable={
          <ContentEditable
            className={editorClasses}
            aria-placeholder="Enter your task description."
            placeholder={<div>Enter your task description.</div>}
          />
        }
        ErrorBoundary={LexicalErrorBoundary}
      />
      <MarkdownShortcutPlugin transformers={TRANSFORMERS} />
      <InputOutputPlugin initialValue={initialValue} onChange={emitChange} />
      <HistoryPlugin />
      <AutoFocusPlugin />
    </LexicalComposer>
  );
}
