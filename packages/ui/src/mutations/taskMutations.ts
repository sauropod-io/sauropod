import { useQueryClient } from "@tanstack/react-query";

import api from "@/api";

/** Mutation to delete a task. */
export function useDeleteTask() {
  const queryClient = useQueryClient();
  return api.useMutation("delete", "/api/task/{id}", {
    onSuccess: () => {
      queryClient.refetchQueries({ queryKey: ["get", "/api/task"] });
      queryClient.refetchQueries({ queryKey: ["get", "/api/tools"] });
    },
  });
}

/** Mutation to create a task. */
export function useCreateTask() {
  const queryClient = useQueryClient();
  return api.useMutation("post", "/api/task", {
    onSuccess: () => {
      queryClient.refetchQueries({ queryKey: ["get", "/api/task"] });
      queryClient.refetchQueries({ queryKey: ["get", "/api/tools"] });
    },
  });
}

/** Mutation to update a task. */
export function useUpdateTask() {
  const queryClient = useQueryClient();
  return api.useMutation("post", "/api/task/{id}", {
    onSuccess: () => {
      queryClient.refetchQueries({ queryKey: ["get", "/api/task/{id}"] });
      queryClient.refetchQueries({
        queryKey: ["get", "/api/task/{id}/schema"],
      });
      queryClient.refetchQueries({ queryKey: ["get", "/api/tools"] });
    },
  });
}
