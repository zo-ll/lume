import type { CommandVm, HoldHintVm, PaletteVm } from "../view-models";

// Transcribed from the approved design's command palette (section p03).

export const paletteFixture: PaletteVm = {
  scopeLabel: "From op_44 · Model Call · draft.compose",
  groups: [
    {
      commands: [
        {
          id: "create-fork-request",
          label: "Create Fork Request from this checkpoint",
          shortcut: "⌘⏎",
          badge: "AVAILABLE",
        },
        {
          id: "compare-source-branch",
          label: "Compare this trace with its source branch",
          shortcut: "⌘D",
        },
        {
          id: "go-to-delegated-trace",
          label: "Go to delegated trace tr_9f2b",
          shortcut: "⌘]",
        },
        {
          id: "reveal-protected-inputs",
          label: "Reveal protected inputs",
          disabledReason: "Protected Store is locked",
        },
      ],
    },
    {
      label: "Elsewhere",
      commands: [
        {
          id: "fork-requests",
          label: "Fork Requests · 1 preparing",
          shortcut: "⌘2",
        },
        {
          id: "runtime-trust",
          label: "Runtime Trust and Recovery Profiles",
          shortcut: "⌘3",
        },
        {
          id: "diagnostic-export",
          label: "Diagnostic Export…",
          shortcut: "⌘⇧E",
        },
      ],
    },
  ],
};

export const holdHintFixture: HoldHintVm = {
  eyebrow: "Checkpoint",
  chips: [
    { key: "⏎", label: "Fork" },
    { key: "D", label: "Compare" },
    { key: "]", label: "Linked" },
    { key: "U", label: "Unlock store", disabled: true },
  ],
};

export const allCommands: readonly CommandVm[] = paletteFixture.groups.flatMap(
  (g) => g.commands,
);
