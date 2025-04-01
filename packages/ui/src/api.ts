import createReactQueryClient from "openapi-react-query";

import createClient from "@sauropod-io/client";

export const apiClient = createClient({
  baseUrl: `${document.location.origin}`,
});

/**
 * react-query wrapper around the API.
 */
export default createReactQueryClient(apiClient);
