import type { OperationDetailVm, TraceWorkspaceVm } from "../view-models";

// Small but real fixtures for the two traces linked from tr_4c81e0's
// lineage strip, so pointer clicks and the [ ] keyboard shortcuts have an
// observable effect: independently owned traces are swapped in, never
// nested inside tr_4c81e0's own outline (design section p07).

const detail = (
  title: string,
  operationId: string,
  lifecycle: string,
): OperationDetailVm => ({
  eyebrow: "Operation selected",
  title,
  operationId,
  causalPosition: "Included in the current trace",
  lifecycle,
  runtime: "rt_support · connected",
  completeness: "Complete",
  trust: "Trusted observation source",
  protectedStore: "Locked · protected values remain masked",
  badges: [],
  permittedActions: ["Open command palette", "Compare linked traces"],
});

/** tr_1a09 — the source trace tr_4c81e0 was forked from. Already ended. */
export const sourceTraceFixture: TraceWorkspaceVm = {
  header: {
    traceId: "tr_1a09",
    agentName: "support-triage-agent",
    completeness: "complete",
    completenessLabel: "COMPLETE",
    outcome: "Resolved",
    trust: "Trusted",
    sessionLive: false,
    runtimeCount: 1,
    runtimeName: "rt_support",
    storeLocked: true,
  },
  links: [
    { traceId: "tr_1a09", current: true },
    { traceId: "tr_4c81e0", relation: "fork", current: false },
  ],
  selectedId: "op_18",
  outline: {
    scaleSeconds: 9,
    operationCount: 34,
    agentStepCount: 3,
    checkpointCount: 1,
    rows: [
      {
        id: "step_a1",
        depth: 0,
        kind: "Agent Step",
        label: "Classify inbound ticket",
        completeness: "complete",
        hasChildren: true,
        expanded: true,
        duration: {
          startFraction: 0,
          extentFraction: 0.3,
          seconds: 0,
          style: "aggregate",
        },
      },
      {
        id: "op_18",
        depth: 1,
        kind: "Model Call",
        label: "classify.v3",
        producer: { kind: "model", label: "model" },
        completeness: "complete",
        duration: {
          startFraction: 0,
          extentFraction: 0.14,
          seconds: 1.61,
          style: "normal",
        },
      },
      {
        id: "op_39",
        depth: 1,
        kind: "Model Call",
        label: "draft.compose",
        producer: { kind: "model", label: "model" },
        completeness: "complete",
        checkpointAvailability: "available",
        badges: ["FORK CHECKPOINT"],
        checkpoint: {
          id: "cut_0f2",
          operationCount: 43,
          availability: "available",
          evidenceAge: "4s old",
        },
        duration: {
          startFraction: 0.6,
          extentFraction: 0.2,
          seconds: 2.9,
          style: "normal",
        },
      },
    ],
  },
  detailsById: {
    step_a1: detail(
      "Agent Step · Classify inbound ticket",
      "step_a1",
      "Succeeded",
    ),
    op_18: detail("Model Call · classify.v3", "op_18", "Succeeded"),
    op_39: detail("Model Call · draft.compose", "op_39", "Succeeded"),
  },
};

/** tr_9f2b — the trace op_45 delegates to. Untrusted runtime, unverified. */
export const delegatedTraceFixture: TraceWorkspaceVm = {
  header: {
    traceId: "tr_9f2b",
    agentName: "doc-retrieval-agent",
    completeness: "unverified",
    completenessLabel: "UNVERIFIED",
    outcome: "Unverified",
    trust: "Untrusted",
    sessionLive: true,
    runtimeCount: 1,
    runtimeName: "rt_docs",
    storeLocked: false,
  },
  links: [
    { traceId: "tr_4c81e0", relation: undefined, current: false },
    { traceId: "tr_9f2b", relation: "delegates", current: true },
  ],
  selectedId: "op_71",
  outline: {
    scaleSeconds: 4,
    operationCount: 6,
    agentStepCount: 1,
    checkpointCount: 0,
    rows: [
      {
        id: "op_70",
        depth: 0,
        kind: "Tool Call",
        label: "docs.search",
        producer: { kind: "tool", label: "tool" },
        completeness: "unverified",
        duration: {
          startFraction: 0,
          extentFraction: 0.4,
          seconds: 1.4,
          style: "normal",
        },
      },
      {
        id: "op_71",
        depth: 0,
        kind: "Tool Call",
        label: "docs.fetch",
        producer: { kind: "tool", label: "tool" },
        completeness: "unverified",
        duration: {
          startFraction: 0.4,
          extentFraction: 0.3,
          seconds: 1.1,
          style: "normal",
        },
      },
    ],
  },
  detailsById: {
    op_70: detail("Tool Call · docs.search", "op_70", "Unverified"),
    op_71: {
      ...detail("Tool Call · docs.fetch", "op_71", "Unverified"),
      badges: ["UNTRUSTED RUNTIME"],
    },
  },
};

export const traceRegistry: Record<string, TraceWorkspaceVm> = {
  tr_1a09: sourceTraceFixture,
  tr_9f2b: delegatedTraceFixture,
};
