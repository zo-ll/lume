# Issue tracker: Local Markdown

Issues and specs for this repository live as Markdown files in `.scratch/`.

## Conventions

- One effort per directory: `.scratch/<effort-slug>/`.
- A specification, when present, is `.scratch/<effort-slug>/spec.md`.
- Issues are one file per ticket at `.scratch/<effort-slug>/issues/<NN>-<slug>.md`, numbered from `01`.
- Conversation history and resolution notes are appended under `## Comments` or `## Answer` rather than replacing the original question.

## Wayfinding operations

- **Map**: `.scratch/<effort>/map.md` contains the destination, notes, decisions index, fog, and scope boundary.
- **Child ticket**: `.scratch/<effort>/issues/NN-<slug>.md` contains one decision question. `Type:` records `research`, `prototype`, `grilling`, or `task`; `Status:` records `open`, `claimed`, or `resolved`.
- **Blocking**: `Blocked by: NN, NN` near the top of a ticket. A ticket is unblocked when every listed file is resolved.
- **Frontier**: open, unblocked, unclaimed child tickets, ordered by ticket number.
- **Claim**: change `Status: open` to `Status: claimed` before beginning work.
- **Resolve**: append the resolution under `## Answer`, change the status to `resolved`, and append a linked one-line gist to the map's `Decisions so far` section.
