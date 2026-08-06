# Prototype the Bubble Tea investigation and fork workflow

Parent: [Specify Lume v1 causal debugging and trace forking](../map.md)
Type: `prototype`
Status: `resolved`
Blocked by: 01, 04, 05, 06, 08
Superseded in part by: 12

## Question

What Bubble Tea interaction model makes it comfortable to investigate a causal trace, select a Fork Checkpoint, edit a typed Intervention, confirm its Fork Execution Mode, watch the new trace live, navigate linked traces, and compare the resulting branch with its original in a terminal?

## Prototype

- Branch: `prototype/bubble-tea-fork-workflow`
- Commit: `a6afc93` (`prototype Bubble Tea fork workflow`)
- Immediate worktree: `/tmp/lume-prototype-bubble-tea`

Run:

```bash
go -C /tmp/lume-prototype-bubble-tea run ./prototype/bubble-tea-fork-workflow
```

The throwaway artifact contains a pure workflow state machine behind a Bubble Tea shell. It uses in-memory fixture data and exposes investigation, checkpoint selection, typed changes, sandboxed/live mode choice, live confirmation, simulated request recovery and preparation, atomic acceptance, live child Operations, linked-trace navigation, pre-acceptance rejection or cancellation, and side-by-side branch comparison.

## Answer

> **Supersession:** Ticket 12 retains this prototype's workflow findings but replaces Bubble Tea, terminal layout rules, and the production TUI target with a Rust/Tauri desktop application and Vue interface.

The prototype validates an investigation-first staged workflow as a viable baseline. The user found the interaction useful enough to retain while explicitly deferring final visual design to a later dedicated design pass. The verdict covers workflow structure, not production styling or exact keybindings.

### Validated interaction structure

- Investigation is the home context. The selected trace and Operation stay visible while the user locates a runtime-declared Fork Checkpoint.
- Fork creation is a linear drill-in from that context: checkpoint selection → typed Intervention → Fork Execution Mode → live confirmation when required → request progress → accepted child trace.
- Contextual commands belong at the bottom of the current screen. Users should not need to memorize one global command set for every workflow phase.
- The Fork Request remains visibly distinct from a Trace Fork. Recovery and preparation show no placeholder child trace; Fork Acceptance creates the child and moves focus to it.
- After acceptance, linked navigation switches between independently owned source and child traces without nesting their Operations.
- Live child Operations append in place while request identity, execution mode, trace outcome, and link context remain inspectable.
- Comparison shows shared pre-checkpoint history once and places only the original and forked suffixes side by side.

### Prototype findings

Three persistent panes—trace graph, timeline, and inspector—are useful on a wide terminal but do not remain readable at 80 columns. The validated responsive behavior is:

- wide terminals keep all three panes;
- narrow terminals collapse the graph into a linked-trace breadcrumb and preserve timeline plus inspector; and
- branch comparison uses a dedicated two-column screen rather than trying to fit inside the ordinary inspector.

Showing the complete state machine in the inspector made request-versus-trace boundaries and invalid transitions easy to evaluate. Production presentation may translate these fields into user-facing language, but must preserve the same distinctions and honest absence of a child before acceptance.

The prototype also confirmed that no-op Intervention rejection, separate live confirmation, pre-acceptance cancellation or rejection, atomic acceptance, immediate post-acceptance failure ownership, linked navigation, and shared-prefix comparison can coexist without requiring hidden navigation modes.

### Deliberately deferred

The prototype does not settle color, typography, borders, final pane proportions, production keybindings, dense-trace navigation, actual field editors, accessibility details, or the final visual identity. Those belong to the user's later design pass. The prototype TUI remains throwaway primary-source evidence on its branch; it is not production code to merge into main.
