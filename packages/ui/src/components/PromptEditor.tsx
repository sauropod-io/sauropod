import { VariablePlugin } from "../editor/VariablePlugin";
import { AutoFocusPlugin } from "@lexical/react/LexicalAutoFocusPlugin";
import {
  InitialConfigType,
  LexicalComposer,
} from "@lexical/react/LexicalComposer";
import { useLexicalComposerContext } from "@lexical/react/LexicalComposerContext";
import { ContentEditable } from "@lexical/react/LexicalContentEditable";
import { LexicalErrorBoundary } from "@lexical/react/LexicalErrorBoundary";
import { HistoryPlugin } from "@lexical/react/LexicalHistoryPlugin";
import { PlainTextPlugin } from "@lexical/react/LexicalPlainTextPlugin";
import { TabIndentationPlugin } from "@lexical/react/LexicalTabIndentationPlugin";
import {
  $createParagraphNode,
  $createTextNode,
  $getCaretRange,
  $getChildCaret,
  $getRoot,
  $getSelection,
  $getSiblingCaret,
  $isElementNode,
  $isLineBreakNode,
  $isTextNode,
  EditorThemeClasses,
  ElementNode,
  LineBreakNode,
} from "lexical";
import { JSX, useEffect } from "react";

import "@/components/PromptEditor.css";
import { $isVariableNode, VariableNode } from "@/editor/VariableNode";

/** Log errors. */
function onError(error: Error): void {
  console.error(error);
}

export interface PromptContent {
  text: string;
  variables: string[];
}

export interface PromptEditorProps {
  initialValue: string;
  editable?: boolean;
  onChange?: (text: PromptContent) => void;
}

const editorClasses =
  "prompt-editor w-full h-full placeholder:text-muted-foreground selection:bg-primary selection:text-primary-foreground dark:bg-input/30 border-input rounded-md border bg-transparent px-3 py-1 text-base shadow-xs md:text-sm focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px] aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive";

function InputOutputPlugin({
  onChange,
  initialValue,
}: {
  onChange: (markdown: PromptContent) => void;
  initialValue: string;
}) {
  const [editor] = useLexicalComposerContext();

  useEffect(() => {
    return editor.registerUpdateListener(() => {
      editor.read(() => {
        onChange({
          text: $convertToString(),
          variables: $getVariables(),
        });
      });
    });
  }, [editor, onChange]);

  useEffect(() => {
    editor.update(() => {
      $convertFromString(initialValue);
    });
  }, [editor, initialValue]);

  if (!onChange) {
    return null;
  }
  return null;
}

/** Populate `node` with `text`. */
function $convertFromString(text: string, node?: ElementNode) {
  const lines = text.split("\n");
  const root = node || $getRoot();
  root.clear();
  const paragraph = $createParagraphNode();
  root.append(paragraph);

  for (let index = 0; index < lines.length; index++) {
    paragraph.append($createTextNode(lines[index]));

    if (index < lines.length - 1) {
      paragraph.append(new LineBreakNode());
    }
  }

  if ($getSelection() !== null) {
    root.selectStart();
  }
}

/** Convert the contents of `node` to a string. */
function $convertToString(node?: ElementNode) {
  const root = node || $getRoot();
  let result = "";

  const startCaret = $getChildCaret(root, "next");
  const endCaret = $getSiblingCaret(root, "next");

  for (const caret of $getCaretRange(startCaret, endCaret)) {
    const { origin } = caret;

    // We only want to process text content so skip element nodes
    if ($isElementNode(origin)) {
      continue;
    }

    if ($isTextNode(origin)) {
      result += origin.getTextContent();
    } else if ($isLineBreakNode(origin)) {
      result += "\n";
    }
  }

  return result;
}

/** Get the variables used in a node. */
function $getVariables(node?: ElementNode): string[] {
  const root = node || $getRoot();
  const variableSet = new Set<string>();

  const startCaret = $getChildCaret(root, "next");
  const endCaret = $getSiblingCaret(root, "next");

  for (const caret of $getCaretRange(startCaret, endCaret)) {
    const { origin } = caret;
    if ($isVariableNode(origin)) {
      variableSet.add(origin.getVariableName());
    }
  }

  const result = [];
  for (const variable of variableSet) {
    result.push(variable);
  }

  return result;
}

const theme: EditorThemeClasses = {};

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
    editorState: () => $convertFromString(initialValue),
    editable,
    nodes: [VariableNode],
  };

  function emitChange(content: PromptContent) {
    if (!onChange) {
      return;
    }

    onChange(content);
  }

  return (
    <LexicalComposer initialConfig={initialConfig}>
      <TabIndentationPlugin />
      <PlainTextPlugin
        contentEditable={
          <ContentEditable
            className={editorClasses}
            aria-placeholder="Enter your task description."
            placeholder={<></>}
          />
        }
        ErrorBoundary={LexicalErrorBoundary}
      />
      <InputOutputPlugin initialValue={initialValue} onChange={emitChange} />
      <HistoryPlugin />
      <AutoFocusPlugin />
      <VariablePlugin />
    </LexicalComposer>
  );
}
