import CustomEdge from "@/components/nodes/CustomEdge";
import InputNode from "@/components/nodes/InputNode";
import OutputNode from "@/components/nodes/OutputNode";
import TaskNode from "@/components/nodes/TaskNode";
import {
  INPUT_NODE_TYPE,
  OUTPUT_NODE_TYPE,
  TASK_NODE_TYPE,
} from "@/lib/workflowGraph";

/** Custom node type mapping. */
export const NODE_TYPES = {
  [INPUT_NODE_TYPE]: InputNode,
  [TASK_NODE_TYPE]: TaskNode,
  [OUTPUT_NODE_TYPE]: OutputNode,
};

/** Custom edge type mapping. */
export const EDGE_TYPES = {
  edge: CustomEdge,
};
