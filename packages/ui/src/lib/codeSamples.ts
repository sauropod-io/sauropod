/* eslint-disable @typescript-eslint/no-explicit-any */
import { type Schemas } from "@sauropod-io/client";

import { JsonSchemaBase, jsonSchemaObjectProperties } from "./jsonSchema";

/** Make a comment describing an output object. */
function makeOutput(commentPrefix: string, output: any) {
  return JSON.stringify(output, null, 2)
    .split("\n")
    .map((line: string) => {
      return `${commentPrefix} ${line}`;
    })
    .join("\n");
}

/** Add a prefix to each line of a string. */
function prefixWith(prefix: string, str: string): string {
  return str
    .split("\n")
    .map((line) => `${prefix}${line}`)
    .join("\n");
}

/** Make a basic Python sample. */
export function makePythonSample(endpoint: string, input: any, output: any) {
  return `
import json

import requests

result = requests.post(
    "${endpoint}",
    json=${prefixWith("    ", JSON.stringify(input, null, 4)).trim()},
)
print(json.dumps(result.json(), indent=2))
# Example output:
${makeOutput("#", output)}
`.trim();
}

/** Get the Pydantic fields for a schema. */
function schemaToPydanticFields(
  schema:
    | Schemas["InputAndOutputSchema"]["inputSchema"]
    | Schemas["InputAndOutputSchema"]["outputSchema"],
): string {
  return jsonSchemaObjectProperties(schema as JsonSchemaBase)
    .map(([key, value]) => {
      let type;
      switch (value.type) {
        case "string":
          type = "str";
          break;
        case "integer":
          type = "int";
          break;
        case "number":
          type = "float";
          break;
        case "boolean":
          type = "bool";
          break;
        case "array":
          type = "list";
          break;
        default:
          type = value.type;
      }

      const description = value.description ? `  #: ${value.description}` : "";
      return `${description}  ${key}: ${type}`;
    })
    .join("\n");
}

/** Get arguments to construct Pydantic object. */
function schemaToPydanticArguments(example: any, prefix: string = ""): string {
  return Object.entries(example)
    .map(
      ([key, value]) =>
        `\n${prefix}  ${key}=${JSON.stringify(value, null, 4)},`,
    )
    .join("");
}

/** Make a Python sample that uses Pydantic. */
export function makePythonPydanticSample(
  endpoint: string,
  schemas: Schemas["InputAndOutputSchema"],
  input: any,
  output: any,
) {
  return `
import pydantic
import requests

class Input(pydantic.BaseModel):
  """Input to the API."""
${schemaToPydanticFields(schemas.inputSchema)}

class Output(pydantic.BaseModel):
  """Output of the API."""
${schemaToPydanticFields(schemas.outputSchema)}

result = requests.post(
  "${endpoint}",
  json=Input(${schemaToPydanticArguments(input, "  ")}
  ).model_dump(),
)
output = Output.model_validate(result.json())
print(output)
# Example output:${schemaToPydanticArguments(output, "# ")}
`.trim();
}

/** Make a Curl sample. */
export function makeCurlSample(endpoint: string, input: any, output: any) {
  return `
curl -X POST \\
  "${endpoint}" \\
  -H "Content-Type: application/json" \\
  -d '${JSON.stringify(input).replace(/'/g, "\\'")}'
# Example output:
${makeOutput("#", output)}
`.trim();
}

/** Get the Pydantic fields for a schema. */
function schemaToTypeScriptFields(
  schema:
    | Schemas["InputAndOutputSchema"]["inputSchema"]
    | Schemas["InputAndOutputSchema"]["outputSchema"],
): string {
  return jsonSchemaObjectProperties(schema as JsonSchemaBase)
    .map(([key, value]) => {
      const description = value.description
        ? `  /** ${value.description} */`
        : "";
      return `${description}  ${key}: ${value.type};`;
    })
    .join("\n");
}

/** Make a TypeScript sample using fetch. */
export function makeTypeScriptSample(
  endpoint: string,
  schema: Schemas["InputAndOutputSchema"],
  input: any,
  output: any,
) {
  return `
/** Input to the API. */
interface Input {
${schemaToTypeScriptFields(schema.inputSchema)}
}

/** Output of the API. */
interface Output {
${schemaToTypeScriptFields(schema.outputSchema)}
}

const input: Input = ${JSON.stringify(input, null, 2)};
const response = await fetch(
  "${endpoint}",
  {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(input)
  },
);

const data = await response.json() as Output;
console.log(JSON.stringify(data, null, 2));
// Example output:
${makeOutput("//", output)}
`.trim();
}

/** Make a Rust sample using reqwest. */
export function makeRustSample(endpoint: string, input: any, output: any) {
  return `
let body = serde_json::json!(${JSON.stringify(input, null, 4)});

let client = reqwest::Client::new();
let response = client
    .post("${endpoint}")
    .json(&body)
    .send()
    .await?;

let data: Value = response.json().await?;
println!("{}", serde_json::to_string_pretty(&data)?);
// Example output:
${makeOutput("//", output)}
`.trim();
}
