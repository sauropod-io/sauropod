import { createContext } from "react";

import { JsonSchemaBase } from "@/lib/jsonSchema";

const InputsContext = createContext<{ [name: string]: JsonSchemaBase }>({});

export default function InputVariablesProvider({
  children,
  value,
}: {
  children: React.ReactNode;
  value: { [name: string]: JsonSchemaBase };
}) {
  return (
    <InputsContext.Provider value={value}>{children}</InputsContext.Provider>
  );
}

/** Consumer for the input variables. */
export const InputVariablesConsumer = InputsContext.Consumer;
