// Named scenes for the dev-only scenario switcher. Every entry here is
// reachable in review even though several have no natural trigger yet
// (no Rust backend exists to fail a connection, drop Operations, etc.).
// Not product chrome — see DevScenarioSwitcher.vue.

export type SceneId =
  | "investigation"
  | "state-degraded"
  | "fork-intervention"
  | "fork-blocked"
  | "fork-invalid"
  | "fork-mode"
  | "fork-live-confirm"
  | "lifecycle-gallery"
  | "comparison"
  | "state-empty"
  | "state-loading"
  | "state-offline"
  | "state-error";

export interface SceneOption {
  id: SceneId;
  label: string;
}

export interface SceneGroup {
  label: string;
  scenes: readonly SceneOption[];
}

export const sceneGroups: readonly SceneGroup[] = [
  {
    label: "Investigation (p01-p02)",
    scenes: [
      { id: "investigation", label: "Wide workspace" },
      { id: "state-degraded", label: "Degraded (evidence gap)" },
    ],
  },
  {
    // Stage 1 (Checkpoint) is not its own panel: it is the checkpoint row
    // you already clicked "Create Fork Request" from in the investigation
    // workspace, and the stepper shows it pre-completed for that reason.
    // Stages 2-4 below are the panels this implementation actually has.
    label:
      "Fork creation (p04-p05) — stage 1 is the checkpoint row, not a separate screen",
    scenes: [
      { id: "fork-intervention", label: "Stage 2 · Intervention editing" },
      { id: "fork-blocked", label: "Stage 2 · Blocked (no-op)" },
      { id: "fork-invalid", label: "Stage 2 · Invalid (constraint)" },
      { id: "fork-mode", label: "Stage 3 · Execution mode" },
      { id: "fork-live-confirm", label: "Stage 4 · Live confirmation" },
    ],
  },
  {
    label: "Fork Request lifecycle (p06)",
    scenes: [{ id: "lifecycle-gallery", label: "All 7 states" }],
  },
  {
    label: "Linked navigation (p07)",
    scenes: [{ id: "comparison", label: "Branch comparison" }],
  },
  {
    label: "Empty / loading / error (p08)",
    scenes: [
      { id: "state-empty", label: "Empty · first run" },
      { id: "state-loading", label: "Loading · outline skeleton" },
      { id: "state-offline", label: "Offline · no runtime" },
      { id: "state-error", label: "Error · service unreachable" },
    ],
  },
];
