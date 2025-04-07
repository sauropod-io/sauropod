/** A subset of a JSON schema object. */
export type JsonSchemaBase = {
  type: "string" | "number" | "boolean" | "object" | "array";
  description?: string;
};

export type JsonSchemaObject = JsonSchemaBase & {
  properties: Record<string, JsonSchemaBase>;
  required?: string[];
};

/** Check whether a JSON schema object is an object schema. */
export function isJsonSchemaObject(
  schema: JsonSchemaBase,
): schema is JsonSchemaObject {
  return schema.type === "object";
}

/** Check whether a JSON schema object is an object schema.
 * @throws Error if the schema is not an object schema.
 */
export function assertIsJsonSchemaObject(
  schema: JsonSchemaBase,
): JsonSchemaObject {
  if (!isJsonSchemaObject(schema)) {
    throw new Error(
      `Schema is not an object schema: ${JSON.stringify(schema)}`,
    );
  }
  return schema;
}
