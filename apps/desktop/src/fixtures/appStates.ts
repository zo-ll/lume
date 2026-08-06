import type { DegradedGapVm } from "../view-models";

// Verbatim copy from the approved design's empty/loading/degraded/offline/
// error states (section p08).

export const emptyStateCopy = {
  title: "No Execution Traces yet",
  body: "The Lume Service is running and listening on localhost:7701. Point an Instrumented Agent Runtime at it and its traces appear here as they arrive.",
};

export const loadingStateCopy = {
  caption: "Reading Local History · 12,400 Operations indexed",
  note: "no spinner — row count rises as it loads",
};

export const degradedGapFixture: DegradedGapVm = {
  fromOperationId: "op_112",
  toOperationId: "op_144",
  operationCount: 31,
  seconds: 4.2,
};

export const degradedStateCopy = {
  title: "This trace has a gap",
  body: "The runtime acknowledged dropping 31 Operations between op_112 and op_144 under backpressure. The trace is marked incomplete, not repaired, and causal claims across the gap are withheld rather than inferred.",
};

export const offlineStateCopy = {
  title: "Local History is fully investigable",
  body: "No Runtime Session is open. Everything already received can be read, searched, and exported. Forking cannot proceed until the owning runtime is reachable and revalidates the checkpoint.",
  capabilities: [
    { available: true, label: "Investigation, comparison, export — available" },
    {
      available: false,
      label:
        "Fork, unlock-and-restore, live mode — unavailable, reason shown in place",
    },
  ],
};

export const errorStateCopy = {
  title: "Cannot reach the Lume Service",
  body: "The desktop window is a client of a separate per-user service. This window has nothing to show, but trace capture may still be running — or the service may have stopped.",
  technicalLine: "socket /run/user/1000/lume.sock · connection refused",
};
