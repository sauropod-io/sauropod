import type { Edge, Node } from "@xyflow/react";

import { Schemas } from "@sauropod-io/client";

import { IONodeData } from "@/components/nodes/IONode";
import type { TaskNodeData } from "@/components/nodes/TaskNode";

/** The node type used in the workflow graph. */
export type GraphNode = Node<IONodeData | TaskNodeData>;

/** Convert from the graph representation of a workflow to its backend representation. */
export function graphToWorkflow(
  name: string,
  nodes: GraphNode[],
  edges: Edge[],
): Schemas["Workflow"] {
  const actions: Schemas["Workflow"]["actions"] = {};

  // Extract task nodes and build tasks map
  nodes
    .filter((node) => node.type === TASK_NODE_TYPE)
    .forEach((node) => {
      const { taskId } = node.data as TaskNodeData;
      // Use the node ID as the key in the tasks map
      const taskKey = node.id;

      actions[taskKey] = { taskId: taskId };
    });

  // Create connections array
  const connections: Schemas["Workflow"]["connections"] = [];

  // Process edges to build connections
  edges.forEach((edge) => {
    const sourceNode = nodes.find((node) => node.id === edge.source);
    const targetNode = nodes.find((node) => node.id === edge.target);
    if (!sourceNode || !targetNode) return;

    if (sourceNode.type === INPUT_NODE_TYPE) {
      // This is a parameter connection (input parameter -> task)
      const inputName = edge.sourceHandle;

      connections.push({
        parameter: inputName!,
        // TODO
        to: [`${targetNode.id}.input`], // Assuming 'input' as the parameter name
      });
    } else if (
      sourceNode.type === TASK_NODE_TYPE &&
      targetNode.type !== OUTPUT_NODE_TYPE
    ) {
      // This is a task-to-task connection
      // TODO fix connection names
      connections.push({
        from: `${sourceNode.id}.output`, // Assuming 'output' as the output field
        to: [`${targetNode.id}.input`], // Assuming 'input' as the input field
      });
    } else if (
      sourceNode.type === TASK_NODE_TYPE &&
      targetNode.type === OUTPUT_NODE_TYPE
    ) {
      // TODO
    }
  });

  return {
    name,
    actions,
    connections,
  };
}

/** Convert from the backend representation of a workflow to its graph representation. */
export function workflowToGraph(workflow: Schemas["Workflow"]): {
  nodes: GraphNode[];
  edges: Edge[];
} {
  const nodes: GraphNode[] = [];
  const edges: Edge[] = [];

  // Map to track created nodes
  const createdNodes: Record<string, boolean> = {};

  // Create task nodes from actions
  Object.entries(workflow.actions).forEach(([nodeId, action]) => {
    if (!("taskId" in action)) {
      throw new Error(
        `Action ${nodeId} does not have a taskId - currently we don't support other types of actions`,
      );
    }

    nodes.push({
      id: nodeId,
      type: TASK_NODE_TYPE,
      position: { x: 250, y: 100 * nodes.length }, // Simple positioning logic
      data: { taskId: action.taskId } as TaskNodeData,
    });
    createdNodes[nodeId] = true;
  });

  const inputs = workflow.connections
    .filter((x): x is { parameter: string; to: string[] } => "parameter" in x)
    .map((x) => x.parameter);
  if (inputs.length > 0) {
    nodes.push({
      id: INPUT_NODE_ID,
      type: INPUT_NODE_TYPE,
      position: { x: 0, y: 100 },
      data: { names: inputs } as IONodeData,
    });
  }

  // Process connections to create input nodes and edges
  workflow.connections.forEach((connection) => {
    // Handle input parameter connections
    if ("parameter" in connection) {
      const inputNodeId = `input-${connection.parameter}`;

      // Create edges from input node to destination tasks
      connection.to.forEach((target) => {
        const [targetNodeId, targetPort] = target.split(".");
        edges.push({
          id: `${inputNodeId}-${targetNodeId}`,
          source: inputNodeId,
          target: targetNodeId,
          targetHandle: targetPort,
        });
      });
    }

    // Handle task-to-task or task-to-output connections
    if ("from" in connection) {
      const [sourceNodeId, sourcePort] = connection.from.split(".");

      connection.to.forEach((target) => {
        const [targetNodeId, targetPort] = target.split(".");

        if (!workflow.actions[targetNodeId]) {
          throw new Error(`Target node ${targetNodeId} not found in actions`);
        }

        // This is a task-to-task connection
        edges.push({
          id: `${sourceNodeId}-${sourcePort}-${targetNodeId}-${targetPort}`,
          source: sourceNodeId,
          target: targetNodeId,
          sourceHandle: sourcePort,
          targetHandle: targetPort,
        });
      });
    }
  });

  return { nodes, edges };
}

/** The ID of the input node. */
export const INPUT_NODE_ID = "input";
/** The ID of the output node. */
export const OUTPUT_NODE_ID = "output";

export const INPUT_NODE_TYPE = "input-node";
export const OUTPUT_NODE_TYPE = "output-node";
export const TASK_NODE_TYPE = "task-node";
