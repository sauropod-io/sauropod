/* eslint-disable @typescript-eslint/no-explicit-any */

/** Make a comment describing an output object. */
function makeOutput(commentPrefix: string, output: any) {
  return JSON.stringify(output, null, 4)
    .split("\n")
    .map((line: string) => {
      return `${commentPrefix} ${line}`;
    })
    .join("\n");
}

/** Make a Python sample. */
export function makePythonSample(endpoint: string, input: any, output: any) {
  return `
import json

import requests

result = requests.post(
  "${endpoint}",
  json=${JSON.stringify(input, null, 4)},
)
print(json.dumps(result.json(), indent=4))
# Example output:
${makeOutput("#", output)}
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

/** Make a TypeScript sample using fetch. */
export function makeTypeScriptSample(
  endpoint: string,
  input: any,
  output: any,
) {
  return `
const response = await fetch(
  "${endpoint}",
  {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(${JSON.stringify(input, null, 4)})
  },
);

const data = await response.json();
console.log(JSON.stringify(data, null, 4));
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
