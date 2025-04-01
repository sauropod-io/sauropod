import { useQueryClient } from "@tanstack/react-query";

import api from "@/api";

/** Mutation to delete a task. */
export function useDeleteTask() {
  const queryClient = useQueryClient();
  return api.useMutation("delete", "/api/task/{id}", {
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["get", "/api/task"] });
    },
  });
}

/** Mutation to create a task. */
export function useCreateTask() {
  const queryClient = useQueryClient();
  return api.useMutation("post", "/api/task", {
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["get", "/api/task"] });
    },
  });
}

/** Mutation to update a task. */
export function useUpdateTask() {
  const queryClient = useQueryClient();
  return api.useMutation("post", "/api/task/{id}", {
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["get", "/api/task/{id}"] });
      queryClient.invalidateQueries({
        queryKey: ["get", "/api/task/{id}/inputSchema"],
      });
    },
  });
}
