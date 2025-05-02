export type FieldType =
  | "string"
  | "number"
  | "integer"
  | "boolean"
  | "object"
  | "array";
export type FieldValue =
  | string
  | number
  | boolean
  | object
  | FieldValue[]
  | { [key: string]: FieldValue };

/** A subset of a JSON schema object. */
export type JsonSchemaBase = {
  type: FieldType;
  description?: string;
  examples?: FieldValue[];
};

export type JsonSchemaObject = JsonSchemaBase & {
  type: "object";
  properties: Record<string, JsonSchemaBase>;
  required?: string[];
  additionalProperties?: boolean;
};

export type JsonSchemaArray = JsonSchemaBase & {
  type: "array";
  items: JsonSchemaBase;
};

export type JsonSchemaString = JsonSchemaBase & {
  type: "string";
  pattern?: string;
};

/** Check whether a JSON schema object is an object schema. */
export function isJsonSchemaObject(
  schema: JsonSchemaBase,
): schema is JsonSchemaObject {
  return schema.type === "object";
}

/** Check whether a JSON schema object is an array schema. */
export function isJsonSchemaArray(
  schema: JsonSchemaBase,
): schema is JsonSchemaArray {
  return schema.type === "array";
}

/** Check whether a JSON schema object is a string schema. */
export function isJsonSchemaString(
  schema: JsonSchemaBase,
): schema is JsonSchemaString {
  return schema.type === "string";
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
