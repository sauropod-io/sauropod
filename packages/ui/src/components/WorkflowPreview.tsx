import { Background, ReactFlow, ReactFlowProvider } from "@xyflow/react";
import "@xyflow/react/dist/base.css";

import { Schemas } from "@sauropod-io/client";

import { EDGE_TYPES, NODE_TYPES } from "@/components/nodes/CustomNodes";
import { workflowToGraph } from "@/lib/workflowGraph";

/** Show a workflow graph. */
export default function WorkflowPreview({
  workflow,
  height = "150px",
}: {
  workflow: Schemas["Workflow"];
  height?: string;
}) {
  const { nodes, edges } = workflowToGraph(workflow);

  return (
    <div style={{ height }} className="w-full">
      <ReactFlowProvider>
        <ReactFlow
          nodes={nodes}
          edges={edges}
          nodeTypes={NODE_TYPES}
          edgeTypes={EDGE_TYPES}
          defaultEdgeOptions={{
            type: "edge",
            animated: false,
          }}
          onInit={(reactFlow) => {
            reactFlow.fitView({ padding: 0.0 });
          }}
          fitView
          nodesConnectable={false}
          nodesDraggable={false}
          elementsSelectable={false}
          zoomOnScroll={false}
          panOnScroll={false}
          panOnDrag={false}
          proOptions={{
            // Hide the attribution in the preview because it looks a little odd in the card display
            hideAttribution: true,
          }}
        >
          <Background gap={24} size={2} />
        </ReactFlow>
      </ReactFlowProvider>
    </div>
  );
}
