/**
 * Custom extensions for JSON Schema.
 */
import {
  FieldType,
  FieldValue,
  JsonSchemaBase,
  JsonSchemaString,
  isJsonSchemaArray,
  isJsonSchemaObject,
  isJsonSchemaString,
  jsonSchemaObjectProperties,
} from "@/lib/jsonSchema";

/** Internal field types. */
export type InternalFieldType = FieldType | "image" | "audio";

/** Regex used to validate image data. */
export const IMAGE_REGEX = "^data:image/([^;]+);base64,[A-Za-z0-9+/=]+$";

/** Regex used to validate audio data. */
export const AUDIO_REGEX = "^data:audio/([^;]+);base64,[A-Za-z0-9+/=]+$";

/** Convert a field type to more friendly name. */
export function fieldTypeToFriendly(type: InternalFieldType): string {
  switch (type) {
    case "string":
      return "Text";
    case "number":
      return "Number";
    case "boolean":
      return "True/False";
    case "image":
      return "Image";
    case "audio":
      return "Audio";
    default:
      return type;
  }
}

/** Make a schema describing an internal field type. */
export function internalFieldTypeToSchema(
  fieldType: InternalFieldType,
): JsonSchemaBase {
  switch (fieldType) {
    case "audio":
      return {
        type: "string",
        description:
          "Base64 encoded audio string - e.g. `data:audio/mpeg;base64,<data>`",
        pattern: AUDIO_REGEX,
      } as JsonSchemaString;
    case "image":
      return {
        type: "string",
        description:
          "Base64 encoded image string - e.g. `data:image/png;base64,<data>`",
        pattern: IMAGE_REGEX,
      } as JsonSchemaString;
    default:
      return {
        type: fieldType,
      } as JsonSchemaBase;
  }
}

/** Check whether a property is an image. */
export function isJsonSchemaImage(property: JsonSchemaBase): boolean {
  return isJsonSchemaString(property) && property.pattern === IMAGE_REGEX;
}

/** Check whether a property is audio. */
export function isJsonSchemaAudio(property: JsonSchemaBase): boolean {
  return isJsonSchemaString(property) && property.pattern === AUDIO_REGEX;
}

/** Translate a JSON schema to an internal field type. */
export function getFieldType(property: JsonSchemaBase): InternalFieldType {
  if (isJsonSchemaImage(property)) {
    return "image";
  } else if (isJsonSchemaAudio(property)) {
    return "audio";
  } else {
    return property.type as InternalFieldType;
  }
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
  } else if (isJsonSchemaImage(schema)) {
    return "data:image/png;base64,<data>";
  } else if (isJsonSchemaAudio(schema)) {
    return "data:image/mpeg;base64,<data>";
  } else if (schema.type === "string") {
    return "text";
  } else if (schema.type === "number" || schema.type === "integer") {
    return 123;
  } else if (schema.type === "boolean") {
    return true;
  }
  throw new Error(`Unknown schema type: ${schema.type}`);
}
