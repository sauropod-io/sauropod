export type FieldType = "string" | "number" | "boolean" | "object" | "array";

/** A subset of a JSON schema object. */
export type JsonSchemaBase = {
  type: FieldType;
  description?: string;
};

export type JsonSchemaObject = JsonSchemaBase & {
  properties: Record<string, JsonSchemaBase>;
  required?: string[];
  additionalProperties?: boolean;
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

/** Get an iterator over the properties of a JSON schema object. */
export function jsonSchemaObjectProperties(
  schema: JsonSchemaBase,
): [string, JsonSchemaBase][] {
  const result = [];
  for (const [key, value] of Object.entries(
    assertIsJsonSchemaObject(schema).properties,
  )) {
    result.push([key, value] as [string, JsonSchemaBase]);
  }
  return result;
}
