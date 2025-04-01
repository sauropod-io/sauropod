export const INDEX = "/";
export const LOGS = "/logs";
export const SETTINGS = "/settings";

export const TASK_PREFIX = "/task/";
export const WORKFLOW_PREFIX = "/workflow/";

/** Get the route for a particular task. */
export function taskRoute(id: string | number) {
  return `${TASK_PREFIX}${id}`;
}

/** Get the route for a particular task. */
export function workflowRoute(id: string | number) {
  return `${WORKFLOW_PREFIX}${id}`;
}
