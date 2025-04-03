import { useQueryClient } from "@tanstack/react-query";

import api from "@/api";

/** Mutation to delete a workflow. */
export function useDeleteWorkflow() {
  const queryClient = useQueryClient();
  return api.useMutation("delete", "/api/workflow/{id}", {
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["get", "/api/workflow"] });
    },
  });
}

/** Mutation to create a workflow. */
export function useCreateWorkflow() {
  const queryClient = useQueryClient();
  return api.useMutation("post", "/api/workflow", {
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["get", "/api/workflow"] });
    },
  });
}

/** Mutation to update a workflow. */
export function useUpdateWorkflow() {
  const queryClient = useQueryClient();
  return api.useMutation("post", "/api/workflow/{id}", {
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: ["get", "/api/workflow/{id}"],
      });
      queryClient.invalidateQueries({
        queryKey: ["get", "/api/workflow/{id}/inputSchema"],
      });
    },
  });
}
