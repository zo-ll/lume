# lume

A local-first TUI dashboard for watching AI agents in real time.

## What it is

Lume is a single-binary Go program that runs on your machine, observes the AI agents you use (Claude Code, Codex, pi, opencode, anything via API), captures what they're doing — tool calls, prompts, responses, costs — and shows it all in a live terminal UI.

You run your agent in one terminal pane, `lume` in another (or in a tmux split), and you can see exactly what the agent is doing right now, kill it if it's gone off the rails, and review what it did after the fact.

## Why

Black-box agents are hard to debug. When Claude Code goes off the rails 8 steps in, you have no idea why. Existing observability tools (LangSmith, Langfuse, Phoenix) are cloud-hosted, framework-locked, and aimed at production monitoring. Lume is local-first, agent-agnostic, and aimed at *understanding* a single run.

## What it is not

Not an orchestrator. Not a planner. Not a coordinator. Lume watches and shows. It does not tell agents what to do.

(We considered expanding into orchestration. We didn't. See [Decisions log](#decisions-log).)

## Boundaries

### In scope (v1)

- **Process wrapper watcher** — spawns an agent as a subprocess, captures its output
- **API proxy watcher** — sits between the agent and the LLM API, logs every request/response. The universal adapter — works for any OpenAI-compatible or Anthropic-compatible agent
- **Hook receiver watcher** — accepts POST events from agents with native hook support (Claude Code, etc.)
- **SQLite event store** — single file, on disk, no cloud
- **TUI dashboard** — list of active runs, detail view per run, live updates
- **Kill switch** — terminate a stuck agent from the TUI
- **CLI commands** — `lume list`, `lume show <id>`, `lume kill <id>` for scripting
- **Single binary** — no runtime, no dependencies, no install

### Out of scope (explicitly)

- Task planning / dispatching
- Multi-agent coordination (agents don't talk to each other)
- Scheduling / cron
- Cloud / hosted version
- Auth / multi-user
- Eval / scoring
- Modifying agent behavior (observation only, with termination)
- Cross-machine distributed

### Deferred (v2, not v1)

- Replay / time-travel debugging (modify a tool's response, re-run from that point)
- Search across runs
- Cost budgets / alerts
- Diff two runs
- Export / share runs

## Architecture

```
┌─────────────────────────────────────────────────┐
│ lume (single Go process)                        │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐       │
│  │ Watcher  │  │ Watcher  │  │ Watcher  │ ...   │
│  │ process  │  │ proxy    │  │ hook     │       │
│  └────┬─────┘  └────┬─────┘  └────┬─────┘       │
│       │             │             │             │
│       └─────────────┼─────────────┘             │
│                     ▼                           │
│              ┌────────────┐                     │
│              │ Event Bus  │                     │
│              └─────┬──────┘                     │
│                    ▼                            │
│              ┌────────────┐                     │
│              │   Store    │ (SQLite)            │
│              └─────┬──────┘                     │
│                    ▼                            │
│              ┌────────────┐                     │
│              │    TUI     │ (Bubble Tea)        │
│              └────────────┘                     │
└─────────────────────────────────────────────────┘
```

## Stack

- **Language:** Go — purpose-built for this kind of tool, single static binary, fast iteration, great stdlib
- **TUI:** Bubble Tea + bubbles + lipgloss + glamour (charmbracelet stack)
- **Storage:** SQLite via `modernc.org/sqlite` (pure Go, no cgo)
- **That's it.** No web framework, no orchestrator library, nothing else.

## File structure

```
lume/
├── go.mod
├── go.sum
├── README.md
├── plan.md
├── cmd/
│   └── lume/
│       └── main.go              # CLI entry, subcommands
├── internal/
│   ├── model/
│   │   └── event.go             # Event type
│   ├── store/
│   │   └── store.go             # SQLite wrapper
│   ├── watcher/
│   │   ├── watcher.go           # Watcher interface
│   │   ├── process.go           # Process wrapper
│   │   ├── proxy.go             # API proxy
│   │   └── hook.go              # Hook receiver
│   ├── eventbus/
│   │   └── bus.go               # In-process event bus
│   └── tui/
│       ├── tui.go               # Bubble Tea root model
│       ├── views.go             # List + detail views
│       └── styles.go            # Lipgloss styles
├── examples/
│   ├── claude-code/
│   │   └── settings.json        # Example hook config
│   └── README.md                # How to wire up each agent
└── LICENSE
```

## Watchers

### Process watcher
Spawns an agent as a child process, captures stdout/stderr. Works for any CLI agent.

```bash
lume watch --process -- claude-code ...
```

### API proxy watcher
Runs an HTTP server on a local port, logs every request, forwards to the real LLM API. The agent is configured to use `http://localhost:PORT/v1` instead of the real API.

```bash
lume watch --proxy --port 8080 --target https://api.anthropic.com
ANTHROPIC_BASE_URL=http://localhost:8080 claude-code ...
```

### Hook watcher
Receives POST events from agents with native hook support. Cleanest data, but per-agent integration.

```bash
lume watch --hook --port 8081
# Then configure the agent's hooks to POST to http://localhost:8081/event
```

## TUI

- **List view** (default): all active runs, with status, model, runtime, cost
- **Detail view**: per-run, shows recent tool calls, current context, output stream
- **Live updates** via Bubble Tea's command system
- **Keybindings**: `j/k` to navigate, `enter` to drill in, `K` to kill, `q` to quit

## CLI

```bash
lume                            # open TUI
lume list                       # list runs (like docker ps)
lume show <id>                  # show details of a run
lume kill <id>                  # kill a running agent
lume watch --process -- <cmd>   # start a process watcher
lume watch --proxy --port X     # start an API proxy watcher
lume watch --hook --port Y      # start a hook receiver watcher
```

## Decisions log

The story of how we got here.

### Origin

The project started as an exploration of LLM context management — caching, memory systems (MemGPT), dynamic tool calls, semantic tool retrieval. During a "what should I build?" brainstorm, we landed on a tool-call inspector: a debugger for agents.

That grew: "what if the inspector worked for any agent, not just yours?" — a universal watcher. Then: "could this expand into an orchestrator?" We said no, correctly. Then: the universal watcher with a TUI dashboard.

### Naming

We considered:

- `tally` (counts what agents do)
- `periscope` (vivid metaphor)
- `nyx` (Greek goddess of night) — taken on PyPI
- `iris` (Greek messenger goddess)
- `lume` (Italian: light/lamp — pronounced "loom" by English speakers)

We chose `lume`. The accidental English pronunciation (loom) is a happy coincidence — to loom means to watch/menace. Light that watches. 4 letters, easy to type, available as a brand (we'll use `lume-agent` on PyPI if we ever ship there).

### Stack

- Started with Python, then Bun, then considered Rust/C/Go/Clojure/Odin/Nim/Crystal/Elixir/Zig/V/Gleam/Janet
- Chose Go: purpose-built for this kind of tool, single static binary, fast iteration, great stdlib
- TUI (not web): lives next to the agent in a tmux split, no port, no browser
- Bubble Tea (not hand-rolled): one "real" dep is fine, the time savings are worth it
- Considered "only SQLite as a dependency" — would have meant hand-writing the TUI. Worth the trade for Bubble Tea.

### Boundaries

The "could this be an orchestrator" question was the most important scope decision. We said no. The watcher is the foundation. Orchestration is a different project, and a much bigger one. The architecture is designed such that orchestration could be added later, but the watcher ships first.

### Why not a daemon + web dashboard?

We considered it. A TUI is the right call because:
- Lives in the same terminal as the agent (tmux split, side-by-side)
- No port to remember
- No browser to open
- Works over SSH
- Single process, no separation between UI and data layer

## Open questions

- How to handle agent identity? (process PID? assigned UUID? user-supplied name?)
- Schema for the SQLite store? (events table, runs table, costs table)
- How to render the "context window" view? (full prompt, diff view, just the new tokens?)
- Process watcher: stdout only, or PTY for TUI agents?
- API proxy: should it normalize across OpenAI/Anthropic formats?
- When to consider v2 (replay) — is the "modify tool response and re-run" feature actually worth the complexity?

## Roadmap

- **v1:** three watcher types, TUI, SQLite, kill switch, CLI commands
- **v2:** replay, search, cost budgets, diff
- **v3 (maybe):** orchestration layer, if there's a real use case

## License

TBD. MIT seems right for a tool like this.
