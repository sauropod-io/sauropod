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
  properties: Record<string, JsonSchemaBase>;
  required?: string[];
  additionalProperties?: boolean;
};

export type JsonSchemaArray = JsonSchemaBase & {
  items: JsonSchemaBase;
};

/** Check whether a JSON schema object is an object schema. */
export function isJsonSchemaObject(
  schema: JsonSchemaBase,
): schema is JsonSchemaObject {
  return schema.type === "object";
}

/** Check whether a JSON schema object is an object schema. */
export function isJsonSchemaArray(
  schema: JsonSchemaBase,
): schema is JsonSchemaArray {
  return schema.type === "array";
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

/** Create an example object from a JSON schema. */
export function makeExampleObject(schema: JsonSchemaBase): FieldValue {
  if (isJsonSchemaObject(schema)) {
    const obj: { [key: string]: FieldValue } = {};
    for (const [key, value] of jsonSchemaObjectProperties(schema)) {
      obj[key] = makeExampleObject(value);
    }
    return obj;
  } else if (isJsonSchemaArray(schema)) {
    return [makeExampleObject(schema.items)];
  } else if (schema.examples && schema.examples.length > 0) {
    return schema.examples[0];
  } else if (schema.type === "string") {
    return "text";
  } else if (schema.type === "number" || schema.type === "integer") {
    return 123;
  } else if (schema.type === "boolean") {
    return true;
  }
  throw new Error(`Unknown schema type: ${schema.type}`);
}
