import _createClient, { Client, ClientOptions } from "openapi-fetch";
import type { MediaType } from "openapi-typescript-helpers";

import type { components, paths } from "./openapi";

export type Schemas = components["schemas"];

export default function createClient<Media extends MediaType = MediaType>(
  clientOptions?: ClientOptions,
): Client<paths, Media> {
  return _createClient<paths>(clientOptions);
}
