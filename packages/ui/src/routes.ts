export const INDEX = "/";
export const LOGS = "/logs";
export const RUN_HISTORY = "/run-history";

export const TASK_PREFIX = "/task/";

/** Get the route for a particular task. */
export function taskRoute(id: string | number) {
  return `${TASK_PREFIX}${id}`;
}
