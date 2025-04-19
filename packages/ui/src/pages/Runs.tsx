import { format, formatDuration, intervalToDuration } from "date-fns";
import { JSX } from "react";

import type { Schemas } from "@sauropod-io/client";

import api from "@/api";
import PageHeader from "@/components/PageHeader";
import { ErrorBadge } from "@/components/badge";

type Step = Schemas["Step"];

type StepStatus = "error" | "completed" | "running";

const statusToColor: { [K in StepStatus]: string } = {
  error: "#e44d42",
  completed: "#73bf69",
  running: "#5794f2",
};

/** Get the status of a step. */
function getStatus(step: Step): StepStatus {
  if (step.error) {
    return "error";
  } else if (step.outputs) {
    return "completed";
  }
  return "running";
}

/** Tree representation of steps. */
interface EnhancedStep extends Step {
  children: EnhancedStep[];
}

/** Component that displays a trace of the steps a task executed. */
function TraceViewer({ steps }: { steps: Step[] }) {
  if (!steps || steps.length === 0) {
    return <div className="text-gray-500 py-2">No steps to display</div>;
  }

  const getStartTime = (step: Step) => {
    return step.startTimeMs ? new Date(step.startTimeMs).getTime() : 0;
  };

  const getEndTime = (step: Step) => {
    return step.endTimeMs
      ? new Date(step.endTimeMs).getTime()
      : getStartTime(step) + 1; // Default to startTime + 1ms if no end time
  };

  // Calculate time range
  let startTime = Math.min(...steps.map(getStartTime).filter(Boolean));
  let endTime = Math.max(...steps.map(getEndTime).filter(Boolean));

  // Handle edge cases
  if (!startTime || !endTime || startTime === endTime) {
    startTime = startTime || 0;
    endTime = endTime || startTime + 1000;
  }

  const totalDuration = endTime - startTime;
  const svgWidth = 800;
  const barHeight = 20;
  const barGap = 4;
  const padding = 25;
  const leftPadding = 12;
  const timeScale = (time: number) => {
    return (
      leftPadding +
      ((time - startTime) / totalDuration) * (svgWidth - padding - 25)
    );
  };

  const stepMap = new Map<number, EnhancedStep>();
  steps.forEach((step) => {
    stepMap.set(step.stepId, { ...step, children: [] } as EnhancedStep);
  });

  const rootSteps: EnhancedStep[] = steps
    .filter((x) => x.parentStepId === null)
    .map((x) => stepMap.get(x.stepId)!);

  steps
    .filter((x) => x.parentStepId !== null)
    .forEach((step) => {
      const parent = stepMap.get(step.parentStepId!);
      parent!.children.push(stepMap.get(step.stepId)!);
    });

  let level = 0;
  function* renderSteps(step: EnhancedStep): Generator<JSX.Element> {
    const stepName =
      "toolId" in step.stepAction ? step.stepAction.toolId : step.taskName!;
    const stepStart = getStartTime(step);
    const stepEnd = getEndTime(step);
    const yPos = level * (barHeight + barGap) + 25; // Start after time markers
    const xStart = timeScale(stepStart);
    const status = getStatus(step);
    const isRunning = status === "running";
    const width = isRunning
      ? svgWidth - xStart
      : Math.max(timeScale(stepEnd) - xStart, 3); // Ensure minimum visibility
    const duration = formatDuration(
      intervalToDuration({ start: stepStart, end: stepEnd }),
    );

    level++;

    yield (
      <g key={step.stepId}>
        {isRunning && (
          <defs>
            <linearGradient
              id={`loading-gradient-${step.stepId}`}
              x1="0%"
              y1="0%"
              x2="100%"
              y2="0%"
            >
              <stop
                offset="0%"
                stopColor={statusToColor.running}
                stopOpacity="0.6"
              />
              <stop
                offset="20%"
                stopColor={statusToColor.running}
                stopOpacity="1"
              />
              <stop
                offset="80%"
                stopColor={statusToColor.running}
                stopOpacity="0.6"
              />
              <stop
                offset="100%"
                stopColor={statusToColor.running}
                stopOpacity="0.6"
              />
              <animate
                attributeName="x1"
                from="-100%"
                to="100%"
                dur="1.5s"
                repeatCount="indefinite"
              />
              <animate
                attributeName="x2"
                from="0%"
                to="200%"
                dur="1.5s"
                repeatCount="indefinite"
              />
            </linearGradient>
          </defs>
        )}
        <rect
          x={xStart}
          y={yPos}
          width={width}
          height={barHeight}
          rx={3}
          fill={
            isRunning
              ? `url(#loading-gradient-${step.stepId})`
              : statusToColor[status]
          }
          opacity={0.8}
          stroke="#333"
          strokeWidth="0.5"
        >
          <title>
            {stepName}: {isRunning ? "In Progress" : duration}
          </title>
        </rect>
        <text
          x={xStart + 4}
          y={yPos + barHeight / 2 + 4}
          fontSize="12"
          fill="#fff"
          className="select-none"
          style={{ textShadow: "0px 0px 2px rgba(0,0,0,0.7)" }}
        >
          {stepName}
        </text>
        <text
          x={xStart + width + 4}
          y={yPos + barHeight / 2 + 4}
          fontSize="10"
          fill="#666"
          className="select-none"
        >
          {duration}
        </text>
      </g>
    );

    for (const child of step.children) {
      for (const element of renderSteps(child)) {
        yield element;
      }
    }
  }

  const elements = [];
  for (const root of rootSteps) {
    for (const element of renderSteps(root)) {
      elements.push(element);
    }
  }

  const svgHeight = Math.max(elements.length * (barHeight + barGap) + 30, 100);
  return (
    <svg
      width={svgWidth}
      height={svgHeight}
      className="self-center max-w-[100%] font-sans mt-4 overflow-x-auto"
    >
      {/* Time axis */}
      <line
        x1={leftPadding}
        y1={15}
        x2={svgWidth}
        y2={15}
        stroke="#ddd"
        strokeWidth="1"
      />

      {/* Time markers */}
      {[0, 0.25, 0.5, 0.75, 1].map((percentage) => {
        const timeMarker = startTime + totalDuration * percentage;
        return (
          <g key={`marker-${percentage}`}>
            <line
              x1={timeScale(timeMarker)}
              y1={12}
              x2={timeScale(timeMarker)}
              y2={18}
              stroke="#ddd"
              strokeWidth="1"
            />
            <text
              x={timeScale(timeMarker)}
              y={10}
              textAnchor="middle"
              fontSize="9"
              fill="#666"
              className="select-none"
            >
              {format(new Date(timeMarker), "HH:mm:ss")}
            </text>
          </g>
        );
      })}

      {elements}
    </svg>
  );
}

function RunElement({ taskRun: taskRun }: { taskRun: Schemas["TaskRunInfo"] }) {
  const { data, isLoading, error } = api.useQuery("get", `/api/task/run/{id}`, {
    params: { path: { id: `${taskRun.id}` } },
  });

  if (error != null) {
    return (
      <div>
        Could not load task run {taskRun.id}: {error.error}
      </div>
    );
  }

  if (isLoading) {
    return <div>Loading...</div>;
  }

  const taskRunData = data!;
  const parentStep = taskRunData.steps.filter((x) => x.parentStepId == null);
  const parentStepName = parentStep ? parentStep[0].taskName : `Unknown`;

  const formattedStartTime = taskRun.startTimeMs
    ? format(new Date(taskRun.startTimeMs), "MMM d, yyyy HH:mm:ss")
    : "Unknown time";

  return (
    <div className="flex flex-col py-4 border-b last:border-0">
      <h3 className="font-medium text-lg mb-2">
        {parentStepName} #{taskRun.id}
        <span className="text-muted-foreground text-sm font-normal ml-2">
          {formattedStartTime}
        </span>
      </h3>
      {taskRunData.steps && taskRunData.steps.length > 0 ? (
        <TraceViewer steps={taskRunData.steps} />
      ) : (
        <div className="py-2 text-gray-500">
          No steps available for this task run
        </div>
      )}
    </div>
  );
}

function TaskRunRows() {
  const { data, isLoading, error } = api.useQuery("get", "/api/task/run");
  if (error != null) {
    return (
      <div>
        <ErrorBadge>Error</ErrorBadge> Could not load task runs:
        {` ${error}`}
      </div>
    );
  }

  if (isLoading) {
    return <div>Loading...</div>;
  }

  return (
    <>
      {data!.map((taskRun, index) => (
        <RunElement taskRun={taskRun} key={index} />
      ))}
    </>
  );
}

export default function Logs() {
  return (
    <main className="container mx-auto p-6">
      <PageHeader pageName="Task Runs" />
      <div className="bg-muted p-4 font-mono text-sm">
        <TaskRunRows />
      </div>
    </main>
  );
}
