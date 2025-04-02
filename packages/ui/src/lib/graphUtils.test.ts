import { describe, expect, it } from "vitest";

import type { Schemas } from "@sauropod-io/client";

import { INPUT_NODE_TYPE, TASK_NODE_TYPE, workflowToGraph } from "./graphUtils";

describe("workflowToGraph", () => {
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

    expect(result.nodes).toEqual([
      {
        id: "task1",
        type: TASK_NODE_TYPE,
        position: { x: 250, y: 0 },
        data: { taskId: 1 },
      },
      {
        id: "task2",
        type: TASK_NODE_TYPE,
        position: { x: 250, y: 100 },
        data: { taskId: 2 },
      },
    ]);
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
          to: ["task1.input"],
        },
      ],
    };

    const result = workflowToGraph(workflow);

    expect(result.nodes).toEqual([
      {
        id: "task1",
        type: TASK_NODE_TYPE,
        position: { x: 250, y: 0 },
        data: { taskId: 1 },
      },
      {
        id: "input-inputParam",
        type: INPUT_NODE_TYPE,
        position: { x: 50, y: 100 },
        data: { name: "inputParam" },
      },
    ]);
    expect(result.edges).toEqual([
      {
        id: "input-inputParam-task1",
        source: "input-inputParam",
        target: "task1",
        targetHandleId: "input",
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
          to: ["task2.123"],
        },
      ],
    };

    const result = workflowToGraph(workflow);

    expect(result.nodes).toEqual([
      {
        id: "task1",
        type: TASK_NODE_TYPE,
        position: { x: 250, y: 0 },
        data: { taskId: 1 },
      },
      {
        id: "task2",
        type: TASK_NODE_TYPE,
        position: { x: 250, y: 100 },
        data: { taskId: 2 },
      },
    ]);
    expect(result.edges).toEqual([
      {
        id: "task1-abc-task2-123",
        source: "task1",
        target: "task2",
        sourceHandleId: "abc",
        targetHandleId: "123",
      },
    ]);
  });

  it("should handle throw for invalid task IDs", () => {
    const workflow: Schemas["Workflow"] = {
      name: "Test Workflow",
      actions: {
        task1: { taskId: 1 },
      },
      connections: [
        {
          from: "task1.output",
          to: ["output1"],
        },
      ],
    };

    expect(() => workflowToGraph(workflow)).toThrowError(
      /Target node output1 not found in actions/,
    );
  });
});
