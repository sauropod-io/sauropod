import type { Edge, Node } from "@xyflow/react";

import { Schemas } from "@sauropod-io/client";

import { IONodeData } from "@/components/nodes/IONode";
import type { TaskNodeData } from "@/components/nodes/TaskNode";

/** The node type used in the workflow graph. */
export type GraphNodeData = IONodeData | TaskNodeData;

/** The node type used in the workflow graph. */
export type GraphNode = Node<GraphNodeData>;

/** The horizontal spacing between nodes when automatically laying them out. */
export const NODE_X_SPACING = 200;

/** Convert from the graph representation of a workflow to its backend representation. */
export function graphToWorkflow(
  name: string,
  nodes: GraphNode[],
  edges: Edge[],
): Schemas["Workflow"] {
  const actions: Schemas["Workflow"]["actions"] = {};
  const connections: Schemas["Workflow"]["connections"] = [];

  // Extract task nodes and build tasks map
  nodes
    .filter((node): node is Node<TaskNodeData> => node.type === TASK_NODE_TYPE)
    .forEach((node) => {
      // Here we use the node ID (not to be confused with the task ID) as the key in the tasks map
      actions[node.id] = { taskId: node.data.taskId };
    });

  // Process edges to build connections
  edges.forEach((edge) => {
    const sourceNode = nodes.find((node) => node.id === edge.source);
    const targetNode = nodes.find((node) => node.id === edge.target);
    if (!sourceNode) {
      throw new Error(
        `Edge ${edge.id} (${edge.source} -> ${edge.target}) has invalid source node`,
      );
    }
    if (!targetNode) {
      throw new Error(
        `Edge ${edge.id} (${edge.source} -> ${edge.target}) has invalid target node`,
      );
    }

    if (sourceNode.type === INPUT_NODE_TYPE) {
      // This is a parameter connection (input parameter -> task)
      const inputName = edge.sourceHandle;
      if (!inputName) {
        throw new Error(
          `Edge ${edge.id} (${edge.source} -> ${edge.target}) has invalid input handle`,
        );
      }

      connections.push({
        parameter: inputName,
        to: `${targetNode.id}.${edge.targetHandle!}`,
      });
    } else if (
      sourceNode.type === TASK_NODE_TYPE &&
      targetNode.type !== OUTPUT_NODE_TYPE
    ) {
      const sourceHandle = edge.sourceHandle;
      const targetHandle = edge.targetHandle;
      if (!sourceHandle || !targetHandle) {
        throw new Error(
          `Edge ${edge.id} (${edge.source} -> ${edge.target}) has invalid source or target handle`,
        );
      }

      connections.push({
        from: `${sourceNode.id}.${sourceHandle}`,
        to: `${targetNode.id}.${targetHandle}`,
      });
    } else if (
      sourceNode.type === TASK_NODE_TYPE &&
      targetNode.type === OUTPUT_NODE_TYPE
    ) {
      // This is an output connection (task -> output parameter)
      const outputName = edge.targetHandle;
      if (!outputName) {
        throw new Error(
          `Edge ${edge.id} (${edge.source} -> ${edge.target}) has invalid output handle`,
        );
      }

      connections.push({
        from: `${sourceNode.id}.${edge.sourceHandle}`,
        output: outputName,
      });
    }
  });

  return {
    name,
    actions,
    connections,
  };
}

/** Lay the workflow graph nodes out based on topological sorting, from left to right */
export function topologicalLayout(nodes: GraphNode[], edges: Edge[]): void {
  // Position nodes by level
  const ySpacing = 100; // Vertical spacing between nodes in same level

  // The in-degree (number of incoming edges) for each node
  const inDegree: Record<string, number> = {};
  // A map of outgoing edges for each node
  const outgoing: Record<string, string[]> = {};

  nodes.forEach((node) => {
    inDegree[node.id] = 0;
    outgoing[node.id] = [];
  });
  edges.forEach((edge) => {
    inDegree[edge.target] = (inDegree[edge.target] || 0) + 1;
    outgoing[edge.source].push(edge.target);
  });

  // Find nodes with no incoming edges (sources)
  const sources: string[] = Object.entries(inDegree)
    .filter(([, degree]) => degree === 0)
    .map(([nodeId]) => nodeId);

  // Perform topological sort
  const sortedNodes: string[] = [];
  const queue = [...sources];

  while (queue.length > 0) {
    const nodeId = queue.shift()!;
    sortedNodes.push(nodeId);

    outgoing[nodeId].forEach((targetId) => {
      inDegree[targetId]--;
      if (inDegree[targetId] === 0) {
        queue.push(targetId);
      }
    });
  }

  // Calculate level for each node
  const levels: Record<string, number> = {};

  // Calculate levels for all other nodes
  sortedNodes.forEach((nodeId) => {
    // Find maximum level of predecessors
    let maxLevel = -1;
    edges.forEach((edge) => {
      if (edge.target === nodeId && levels[edge.source] !== undefined) {
        maxLevel = Math.max(maxLevel, levels[edge.source]);
      }
    });

    levels[nodeId] = maxLevel + 1;
  });

  // Group nodes by level
  const nodesByLevel: string[][] = [];
  for (const [nodeId, level] of Object.entries(levels)) {
    if (!nodesByLevel[level]) {
      nodesByLevel[level] = [];
    }
    nodesByLevel[level].push(nodeId);
  }

  for (const [level, levelNodes] of nodesByLevel.entries()) {
    const totalHeight = (levelNodes.length - 1) * ySpacing;

    levelNodes.forEach((nodeId, index) => {
      const node = nodes.find((n) => n.id === nodeId);
      if (!node) {
        throw new Error(`Node with ID ${nodeId} not found`);
      }
      node.position = {
        x: level * NODE_X_SPACING,
        y: index * ySpacing - totalHeight / 2,
      };
    });
  }
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
  Object.entries(workflow.actions || {}).forEach(([nodeId, action]) => {
    if (!("taskId" in action)) {
      throw new Error(
        `Action ${nodeId} does not have a taskId - other types of actions aren't implemented yet`,
      );
    }

    nodes.push({
      id: nodeId,
      type: TASK_NODE_TYPE,
      position: { x: 0, y: 0 },
      data: { taskId: action.taskId } as TaskNodeData,
    });
    createdNodes[nodeId] = true;
  });

  const inputs = workflow.connections
    .filter((x): x is { parameter: string; to: string } => "parameter" in x)
    .map((x) => x.parameter);
  const outputs = workflow.connections
    .filter((x): x is { output: string; from: string } => "output" in x)
    .map((x) => x.output);
  if (inputs.length > 0) {
    nodes.push({
      id: INPUT_NODE_ID,
      type: INPUT_NODE_TYPE,
      position: { x: 0, y: 0 },
      data: { names: inputs } as IONodeData,
    });
  }
  if (outputs.length > 0) {
    nodes.push({
      id: OUTPUT_NODE_ID,
      type: OUTPUT_NODE_TYPE,
      position: { x: 0, y: 0 },
      data: { names: outputs } as IONodeData,
    });
  }

  // Process connections to create input nodes and edges
  workflow.connections.forEach((connection) => {
    // Handle input parameter connections
    if ("parameter" in connection) {
      // Create edges from input node to destination tasks
      const [targetNodeId, targetPort] = connection.to.split(".");
      edges.push({
        id: `${INPUT_NODE_ID}-${connection.parameter}-${targetNodeId}-${targetPort}`,
        source: INPUT_NODE_ID,
        sourceHandle: connection.parameter,
        target: targetNodeId,
        targetHandle: targetPort,
      });
    }

    // Handle task to output connections
    else if ("output" in connection) {
      const outputNodeId = `output-${connection.output}`;

      // Create edges from source tasks to output node
      const [sourceNodeId, sourcePort] = connection.from.split(".");
      edges.push({
        id: `${sourceNodeId}-${sourcePort}-${outputNodeId}`,
        source: sourceNodeId,
        target: OUTPUT_NODE_ID,
        sourceHandle: sourcePort,
        targetHandle: connection.output,
      });
    }

    // Handle task to task connections
    else if ("to" in connection) {
      const [sourceNodeId, sourcePort] = connection.from.split(".");
      const [targetNodeId, targetPort] = connection.to.split(".");

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
    }
  });

  // Apply the topological layout to position nodes
  topologicalLayout(nodes, edges);

  return { nodes, edges };
}

/** The ID of the input node. */
export const INPUT_NODE_ID = "input";
/** The ID of the output node. */
export const OUTPUT_NODE_ID = "output";

export const INPUT_NODE_TYPE = "input-node";
export const OUTPUT_NODE_TYPE = "output-node";
export const TASK_NODE_TYPE = "task-node";
