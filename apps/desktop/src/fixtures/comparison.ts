import type { ComparisonVm } from "../view-models";

// Transcribed from the approved design's branch comparison screen
// (section p07).

export const comparisonFixture: ComparisonVm = {
  originalTraceId: "tr_4c81e0",
  forkTraceId: "tr_b30f",
  sharedOperationCount: 43,
  sharedSummaryRows: [
    { label: "Agent Step · Resolve entitlement", note: "12 Operations" },
    { label: "Agent Step · Draft response", note: "up to the cut" },
  ],
  checkpoint: {
    id: "cut_0f2",
    label: "Model Call · draft.compose",
    operationId: "op_44",
    interventionSummary: "system_prompt, temperature",
  },
  original: {
    outcome: "ESCALATED",
    rows: [
      { id: "c1", label: "Model Call · draft.compose", duration: "3.02s" },
      { id: "c2", label: "Delegation · ▸ tr_9f2b", duration: "1.31s" },
      { id: "c3", label: "Model Call · draft.revise", duration: "1.10s" },
      {
        id: "c4",
        label: "Tool Call · entitlement.verify",
        duration: "0.09s",
        badges: ["FAILED"],
      },
      { id: "c5", label: "Tool Call · ticket.escalate", duration: "0.18s" },
    ],
    footnote: "end of trace · 5 Operations after the cut",
  },
  fork: {
    outcome: "RESOLVED",
    badge: "SANDBOXED",
    rows: [
      {
        id: "f1",
        label: "Model Call · draft.compose",
        duration: "2.44s",
        tint: "success",
      },
      { id: "f2", label: "Tool Call · entitlement.verify", duration: "0.11s" },
      { id: "f3", label: "Tool Call · ticket.reply", duration: "0.20s" },
      { id: "f4", label: "receiving…", receiving: true },
    ],
    footnote: "live · 3 Operations so far",
  },
  summary:
    "The fork skipped the failing retry loop and reached a reply. Two fewer Operations, 1.9s faster to first tool call.",
};
