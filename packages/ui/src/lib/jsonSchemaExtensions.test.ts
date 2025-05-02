import { describe, expect, it } from "vitest";

import {
  JsonSchemaArray,
  JsonSchemaBase,
  JsonSchemaObject,
  JsonSchemaString,
} from "./jsonSchema";
import { IMAGE_REGEX, makeExampleObject } from "./jsonSchemaExtensions";

describe("makeExampleObject", () => {
  it("should return the first example for a string schema with examples", () => {
    const schema: JsonSchemaBase = {
      type: "string",
      examples: ["example text", "other example"],
    };
    expect(makeExampleObject(schema)).toEqual("example text");
  });

  it('should return "text" for a string schema without examples', () => {
    const schema: JsonSchemaBase = { type: "string" };
    expect(makeExampleObject(schema)).toEqual("text");
  });

  it('should return "text" for a string schema without examples', () => {
    const schema: JsonSchemaString = { type: "string", pattern: IMAGE_REGEX };
    expect(makeExampleObject(schema)).toEqual("data:image/png;base64,<data>");
  });

  it('should return "text" for a string schema without examples', () => {
    const schema: JsonSchemaBase = { type: "string" };
    expect(makeExampleObject(schema)).toEqual("text");
  });

  it("should return the first example for a number schema with examples", () => {
    const schema: JsonSchemaBase = {
      type: "number",
      examples: [42, 3.14],
    };
    expect(makeExampleObject(schema)).toEqual(42);
  });

  it("should return 123 for a number schema without examples", () => {
    const schema: JsonSchemaBase = { type: "number" };
    expect(makeExampleObject(schema)).toEqual(123);
  });

  it("should return 123 for an integer schema without examples", () => {
    const schema: JsonSchemaBase = { type: "integer" };
    expect(makeExampleObject(schema)).toEqual(123);
  });

  it("should return the first example for a boolean schema with examples", () => {
    const schema: JsonSchemaBase = {
      type: "boolean",
      examples: [false, true],
    };
    expect(makeExampleObject(schema)).toEqual(false);
  });

  it("should return true for a boolean schema without examples", () => {
    const schema: JsonSchemaBase = { type: "boolean" };
    expect(makeExampleObject(schema)).toEqual(true);
  });

  it("should create an array with one item for an array schema", () => {
    const schema: JsonSchemaArray = {
      type: "array",
      items: { type: "string" },
    };
    expect(makeExampleObject(schema)).toEqual(["text"]);
  });

  it("should create an object with all properties for an object schema", () => {
    const schema: JsonSchemaObject = {
      type: "object",
      properties: {
        name: { type: "string" },
        age: { type: "number" },
        isActive: { type: "boolean" },
      },
    };
    expect(makeExampleObject(schema)).toEqual({
      name: "text",
      age: 123,
      isActive: true,
    });
  });

  it("should handle nested objects", () => {
    const schema: JsonSchemaObject = {
      type: "object",
      properties: {
        user: {
          type: "object",
          properties: {
            name: { type: "string" },
            details: {
              type: "object",
              properties: {
                age: { type: "number" },
              },
            } as JsonSchemaObject,
          },
        } as JsonSchemaObject,
      },
    };
    expect(makeExampleObject(schema)).toEqual({
      user: {
        name: "text",
        details: {
          age: 123,
        },
      },
    });
  });

  it("should handle arrays of objects", () => {
    const schema: JsonSchemaArray = {
      type: "array",
      items: {
        type: "object",
        properties: {
          name: { type: "string" },
        },
      } as JsonSchemaObject,
    };
    expect(makeExampleObject(schema)).toEqual([{ name: "text" }]);
  });
});
