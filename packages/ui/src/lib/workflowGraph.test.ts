import { describe, expect, it } from "vitest";

import type { Schemas } from "@sauropod-io/client";

import {
  INPUT_NODE_ID,
  INPUT_NODE_TYPE,
  OUTPUT_NODE_ID,
  OUTPUT_NODE_TYPE,
  TASK_NODE_TYPE,
  workflowToGraph,
} from "./workflowGraph";

/**
 * Check that a node has a position value.
 */
// eslint-disable-next-line @typescript-eslint/no-explicit-any
function hasValidPosition(node: any) {
  expect(node.position).toBeDefined();
  expect(typeof node.position.x).toBe("number");
  expect(typeof node.position.y).toBe("number");
  return true;
}

describe("workflowToGraph", () => {
  it("should convert empty workflows to empty graphs", () => {
    const workflow: Schemas["Workflow"] = {
      name: "Empty Workflow",
      actions: {},
      connections: [],
    };

    const { nodes, edges } = workflowToGraph(workflow);
    expect(nodes).toEqual([]);
    expect(edges).toEqual([]);
  });

  it("should convert a workflow with task nodes to graph representation", () => {
    const workflow: Schemas["Workflow"] = {
      name: "Test Workflow",
      actions: {
        task1: { taskId: 1 },
        task2: { taskId: 2 },
      },
      connections: [],
    };

    const result = workflowToGraph(workflow);

    expect(result.nodes).toHaveLength(2);
    expect(result.nodes[0]).toMatchObject({
      id: "task1",
      type: TASK_NODE_TYPE,
      data: { taskId: 1 },
    });
    expect(result.nodes[1]).toMatchObject({
      id: "task2",
      type: TASK_NODE_TYPE,
      data: { taskId: 2 },
    });
    result.nodes.forEach((node) => hasValidPosition(node));
    expect(result.edges).toEqual([]);
  });

  it("should handle input parameter connections", () => {
    const workflow: Schemas["Workflow"] = {
      name: "Test Workflow",
      actions: {
        task1: { taskId: 1 },
      },
      connections: [
        {
          parameter: "inputParam",
          to: "task1.input",
        },
      ],
    };

    const result = workflowToGraph(workflow);

    expect(result.nodes).toHaveLength(2);
    expect(result.nodes[0]).toMatchObject({
      id: "task1",
      type: TASK_NODE_TYPE,
      data: { taskId: 1 },
    });
    expect(result.nodes[1]).toMatchObject({
      id: INPUT_NODE_ID,
      type: INPUT_NODE_TYPE,
      data: { names: ["inputParam"] },
    });
    result.nodes.forEach((node) => hasValidPosition(node));
    expect(result.edges).toEqual([
      {
        id: "input-inputParam-task1-input",
        source: "input",
        sourceHandle: "inputParam",
        target: "task1",
        targetHandle: "input",
      },
    ]);
  });

  it("should handle output parameter connections", () => {
    const workflow: Schemas["Workflow"] = {
      name: "Test Workflow",
      actions: {
        task1: { taskId: 1 },
      },
      connections: [
        {
          from: "task1.a",
          output: "output1",
        },
        {
          from: "task1.b",
          output: "output2",
        },
      ],
    };

    const result = workflowToGraph(workflow);

    expect(result.nodes).toHaveLength(2);
    expect(result.nodes[0]).toMatchObject({
      id: "task1",
      type: TASK_NODE_TYPE,
      data: { taskId: 1 },
    });
    expect(result.nodes[1]).toMatchObject({
      id: OUTPUT_NODE_ID,
      type: OUTPUT_NODE_TYPE,
      data: { names: ["output1", "output2"] },
    });
    result.nodes.forEach((node) => hasValidPosition(node));
    expect(result.edges).toEqual([
      {
        id: "task1-a-output-output1",
        source: "task1",
        sourceHandle: "a",
        target: OUTPUT_NODE_ID,
        targetHandle: "output1",
      },
      {
        id: "task1-b-output-output2",
        source: "task1",
        sourceHandle: "b",
        target: OUTPUT_NODE_ID,
        targetHandle: "output2",
      },
    ]);
  });

  it("should handle task-to-task connections", () => {
    const workflow: Schemas["Workflow"] = {
      name: "Test Workflow",
      actions: {
        task1: { taskId: 1 },
        task2: { taskId: 2 },
      },
      connections: [
        {
          from: "task1.abc",
          to: "task2.123",
        },
      ],
    };

    const result = workflowToGraph(workflow);

    expect(result.nodes).toHaveLength(2);
    expect(result.nodes[0]).toMatchObject({
      id: "task1",
      type: TASK_NODE_TYPE,
      data: { taskId: 1 },
    });
    expect(result.nodes[1]).toMatchObject({
      id: "task2",
      type: TASK_NODE_TYPE,
      data: { taskId: 2 },
    });
    result.nodes.forEach((node) => hasValidPosition(node));
    expect(result.edges).toEqual([
      {
        id: "task1-abc-task2-123",
        source: "task1",
        target: "task2",
        sourceHandle: "abc",
        targetHandle: "123",
      },
    ]);
  });

  it("should throw for invalid task IDs", () => {
    const workflow: Schemas["Workflow"] = {
      name: "Test Workflow",
      actions: {
        task1: { taskId: 1 },
      },
      connections: [
        {
          from: "task1.output",
          to: "output1",
        },
      ],
    };

    expect(() => workflowToGraph(workflow)).toThrowError(
      /Target node output1 not found in actions/,
    );
  });

  it("should convert a workflow with multiple connections", () => {
    const workflow: Schemas["Workflow"] = {
      name: "Flow",
      actions: {
        abc: { taskId: 1 },
        "123": { taskId: 2 },
      },
      connections: [
        { parameter: "url", to: "abc.url" },
        { from: "abc.value", to: "123.content" },
        { from: "123.value", output: "summary" },
      ],
    };

    const result = workflowToGraph(workflow);

    // Verify nodes
    expect(result.nodes).toHaveLength(4); // 2 task nodes + input node + output node

    // Verify task nodes
    const taskNode1 = result.nodes.find((node) => node.id === "abc");
    const taskNode2 = result.nodes.find((node) => node.id === "123");
    expect(taskNode1).toMatchObject({
      id: "abc",
      type: TASK_NODE_TYPE,
      data: { taskId: 1 },
    });
    expect(taskNode2).toMatchObject({
      id: "123",
      type: TASK_NODE_TYPE,
      data: { taskId: 2 },
    });

    // Verify input and output nodes
    const inputNode = result.nodes.find((node) => node.id === INPUT_NODE_ID);
    const outputNode = result.nodes.find((node) => node.id === OUTPUT_NODE_ID);
    expect(inputNode).toMatchObject({
      id: INPUT_NODE_ID,
      type: INPUT_NODE_TYPE,
      data: { names: ["url"] },
    });
    expect(outputNode).toMatchObject({
      id: OUTPUT_NODE_ID,
      type: OUTPUT_NODE_TYPE,
      data: { names: ["summary"] },
    });

    // Verify all nodes have valid positions
    result.nodes.forEach((node) => hasValidPosition(node));

    // Verify edges
    expect(result.edges).toHaveLength(3);
    expect(result.edges).toEqual([
      {
        id: "input-url-abc-url",
        source: "input",
        sourceHandle: "url",
        target: "abc",
        targetHandle: "url",
      },
      {
        id: "abc-value-123-content",
        source: "abc",
        sourceHandle: "value",
        target: "123",
        targetHandle: "content",
      },
      {
        id: "123-value-output-summary",
        source: "123",
        sourceHandle: "value",
        target: "output",
        targetHandle: "summary",
      },
    ]);
    expect(result.edges).toContainEqual({
      id: "abc-value-123-content",
      source: "abc",
      target: "123",
      sourceHandle: "value",
      targetHandle: "content",
    });
    expect(result.edges).toContainEqual({
      id: "123-value-output-summary",
      source: "123",
      sourceHandle: "value",
      target: OUTPUT_NODE_ID,
      targetHandle: "summary",
    });
  });
});
