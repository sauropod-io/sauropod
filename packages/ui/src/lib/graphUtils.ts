import type { Edge, Node } from "@xyflow/react";

import { Schemas } from "@sauropod-io/client";

import type { InputNodeData } from "@/components/nodes/InputNode";
import type { TaskNodeData } from "@/components/nodes/TaskNode";

/** The node type used in the workflow graph. */
export type GraphNode = Node<InputNodeData | TaskNodeData>;

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
      const inputName = (sourceNode.data as InputNodeData).name;

      connections.push({
        parameter: inputName,
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

  console.log(connections);
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
    nodes.push({
      id: nodeId,
      type: TASK_NODE_TYPE,
      position: { x: 250, y: 100 * nodes.length }, // Simple positioning logic
      data: { taskId: action.taskId } as TaskNodeData,
    });
    createdNodes[nodeId] = true;
  });

  // Process connections to create input nodes and edges
  workflow.connections.forEach((connection) => {
    // Handle input parameter connections
    if (connection.parameter) {
      const inputNodeId = `input-${connection.parameter}`;

      // Create input node if not already created
      if (!createdNodes[inputNodeId]) {
        nodes.push({
          id: inputNodeId,
          type: INPUT_NODE_TYPE,
          position: { x: 50, y: 100 * nodes.length }, // Position to the left
          data: { name: connection.parameter } as InputNodeData,
        });
        createdNodes[inputNodeId] = true;
      }

      // Create edges from input node to destination tasks
      connection.to.forEach((target) => {
        const [targetNodeId, targetPort] = target.split(".");
        edges.push({
          id: `${inputNodeId}-${targetNodeId}`,
          source: inputNodeId,
          target: targetNodeId,
          targetHandleId: targetPort,
        });
      });
    }

    // Handle task-to-task or task-to-output connections
    if (connection.from) {
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
          sourceHandleId: sourcePort,
          targetHandleId: targetPort,
        });
      });
    }
  });

  return { nodes, edges };
}
export const INPUT_NODE_TYPE = "input-node";
export const OUTPUT_NODE_TYPE = "output-node";
export const TASK_NODE_TYPE = "task-node";
