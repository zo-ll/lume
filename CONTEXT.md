# Lume

Lume's language for describing instrumented AI-agent activity and the evidence used to understand it.

## Language

**Execution Trace**:
A bounded attempt by one agent to achieve an objective, containing its causally related model calls, tool calls, and other operations. Work delegated to another agent belongs to a separate, linked Execution Trace.
_Avoid_: Run, session, event stream

**Trace Link**:
A typed causal relationship between two Execution Traces, such as delegation to a sub-agent or forking from a prior trace.
_Avoid_: Nested run

**Trace Fork**:
A new Execution Trace that resumes from a chosen point in an existing Execution Trace with a deliberate intervention, while retaining its ancestry.
_Avoid_: Replay, rerun, retry

**Instrumented Agent Runtime**:
The cooperating agent system that emits observable activity to Lume and performs a requested Trace Fork by restoring its own execution state and continuing from it.
_Avoid_: Lume runtime, watcher

**Fork Checkpoint**:
A durable point in an Execution Trace that its Instrumented Agent Runtime declares restorable, together with the typed inputs the user may edit before continuing. It remains addressable across runtime restarts.
_Avoid_: Snapshot, arbitrary event

**Intervention**:
The validated set of checkpoint-defined input changes applied when creating a Trace Fork.
_Avoid_: State patch, replay edit

**Fork Execution Mode**:
The Instrumented Agent Runtime's declaration that a Trace Fork will continue in either a sandboxed environment or a live environment with real side effects.
_Avoid_: Safety level

**Sensitive Field**:
A trace or checkpoint field that the Instrumented Agent Runtime marks as requiring protected storage or omission and hidden-by-default presentation.
_Avoid_: Detected secret
