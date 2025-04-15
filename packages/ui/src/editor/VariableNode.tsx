import { addClassNamesToElement } from "@lexical/utils";
import {
  $applyNodeReplacement,
  type EditorConfig,
  type LexicalNode,
  type SerializedTextNode,
  TextNode,
} from "lexical";

/** A variable like `${foo}`. */
export class VariableNode extends TextNode {
  static getType(): string {
    return "variable";
  }

  constructor(text: string, key?: string) {
    super(text, key);

    if (!text.startsWith("${") || !text.endsWith("}")) {
      throw new Error(
        "VariableNode text must start with `${` and end with `}`",
      );
    }
  }

  static clone(node: VariableNode): VariableNode {
    return new VariableNode(node.__text, node.__key);
  }

  static importJSON(serializedNode: SerializedTextNode): VariableNode {
    return $createVariableNode(serializedNode.text).updateFromJSON(
      serializedNode,
    );
  }

  createDOM(config: EditorConfig): HTMLElement {
    const element = super.createDOM(config);
    addClassNamesToElement(element, "variable", config.theme.variable);
    element.spellcheck = false;
    return element;
  }

  /** Get the name of the variable.
   *
   * For example, if the variable is `${foo}`, this will return `foo`.
   */
  getVariableName(): string {
    return this.getTextContent().slice(2, -1);
  }

  isTextEntity(): true {
    return true;
  }

  isUnmergeable(): boolean {
    return true;
  }

  canInsertTextBefore(): boolean {
    return false;
  }

  canInsertTextAfter(): boolean {
    return false;
  }
}

export function $createVariableNode(text: string): VariableNode {
  return $applyNodeReplacement(new VariableNode(text));
}

export function $isVariableNode(
  node: LexicalNode | null | undefined,
): node is VariableNode {
  return node instanceof VariableNode;
}
