import type { ForkRequestVm } from "../view-models";

// Transcribed from the approved design's "Fork Request lifecycle — seven
// states, one panel" (section p06). All seven fixtures are rendered together
// in the dev scenario gallery, matching the design doc's own review layout;
// in production only one is ever live for a given Fork Request.

export const lifecycleFixtures: readonly ForkRequestVm[] = [
  {
    requestId: "req_7d1",
    phase: "recovering",
    sourceTraceId: "tr_4c81e0",
    recovering: {
      runtimeName: "rt_support",
      profileName: "support-local",
      attempt: 1,
      maxAttempts: 3,
      elapsedSeconds: 6,
    },
  },
  {
    requestId: "req_7d1",
    phase: "preparing",
    sourceTraceId: "tr_4c81e0",
    preparing: {
      checkpointId: "cut_0f2",
      checklist: [
        { label: "Availability revalidated", done: true },
        { label: "Intervention accepted by runtime", done: true },
        { label: "Restoring execution state…", done: false },
      ],
    },
  },
  {
    requestId: "req_7d1",
    phase: "uncertain",
    sourceTraceId: "tr_4c81e0",
    uncertain: { checkpointId: "cut_0f2" },
  },
  {
    requestId: "req_7d1",
    phase: "rejected",
    sourceTraceId: "tr_4c81e0",
    rejected: {
      reason:
        "checkpoint cut_0f2 no longer restorable — process state discarded at restart",
    },
  },
  {
    requestId: "req_7d1",
    phase: "cancelled",
    sourceTraceId: "tr_4c81e0",
    cancelled: { byUser: true },
  },
  {
    requestId: "req_7d1",
    phase: "accepted",
    sourceTraceId: "tr_4c81e0",
    accepted: { childTraceId: "tr_b30f", mode: "sandboxed" },
  },
  {
    requestId: "req_7d1",
    phase: "accepted",
    sourceTraceId: "tr_4c81e0",
    accepted: {
      childTraceId: "tr_b30f",
      mode: "sandboxed",
      childFailure: {
        operationLabel: "Model Call · draft.compose",
        reason: "FAILED — provider rejected system_prompt length",
      },
    },
  },
];
