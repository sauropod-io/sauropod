import { useLexicalComposerContext } from "@lexical/react/LexicalComposerContext";
import { useLexicalTextEntity } from "@lexical/react/useLexicalTextEntity";
import { TextNode } from "lexical";
import { JSX, useCallback, useEffect } from "react";

import { $createVariableNode, VariableNode } from "@/editor/VariableNode";

/** The regex used to match variable strings. */
const VARIABLE_REGEX = /(\$\{[a-zA-Z_][a-zA-Z0-9_]*\})/;

/** Plugin to detect and segment out variables in the editor text. */
export function VariablePlugin(): JSX.Element | null {
  const [editor] = useLexicalComposerContext();
  useEffect(() => {
    if (!editor.hasNodes([VariableNode])) {
      throw new Error("VariablePlugin: VariableNode not registered on editor");
    }
  }, [editor]);

  const $createVariableNodeCallback = useCallback(
    (textNode: TextNode): VariableNode => {
      return $createVariableNode(textNode.getTextContent());
    },
    [],
  );

  const getVariableMatch = useCallback((text: string) => {
    const matchArr = VARIABLE_REGEX.exec(text);
    if (matchArr === null) {
      return null;
    }

    const startOffset = matchArr.index;
    const endOffset = matchArr.index + matchArr[1].length;
    return {
      end: endOffset,
      start: startOffset,
    };
  }, []);

  useLexicalTextEntity(
    getVariableMatch,
    VariableNode,
    $createVariableNodeCallback,
  );
  return null;
}
