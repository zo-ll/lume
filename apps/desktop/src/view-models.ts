// Presentation-layer view models, shaped after the approved desktop design's
// own component/view-model inventory (docs/design/lume-desktop-design.html,
// section "Component and view-model inventory"). Every shape here is meant to
// be filled by a future Rust-backed Tauri adapter; Vue never derives
// completeness, validity, sensitivity, or fork lifecycle — it only renders
// what these types are given. Fixtures in `fixtures/` stand in for that
// adapter today.

// ---------------------------------------------------------------------------
// Legend / gutter states (design doc section 00)
// ---------------------------------------------------------------------------

/** Trace Completeness — Slot 1 of the outline's state gutter. */
export type CompletenessState =
  "complete" | "provisional" | "incomplete" | "unverified";

/** Fork Checkpoint Availability — Slot 2 of the outline's state gutter. */
export type CheckpointAvailabilityState =
  "available" | "unknown" | "unavailable";

// ---------------------------------------------------------------------------
// Trace header / lineage
// ---------------------------------------------------------------------------

export interface TraceHeaderVm {
  traceId: string;
  agentName: string;
  completeness: CompletenessState;
  completenessLabel: string;
  outcome: string;
  trust: string;
  sessionLive: boolean;
  runtimeCount: number;
  runtimeName: string;
  storeLocked: boolean;
}

export type TraceLinkRelation = "fork" | "delegates";

export interface TraceLinkVm {
  traceId: string;
  agentName?: string;
  relation?: TraceLinkRelation;
  current: boolean;
  /** Present only for a History Tombstone chip — evidence was deleted. */
  evidenceDeleted?: boolean;
  /** Extra detail shown on hover-expand (design section p07). */
  hoverDetail?: {
    title: string;
    body: string;
    badges: readonly string[];
    footnote?: string;
  };
}

// ---------------------------------------------------------------------------
// Causal outline rows
// ---------------------------------------------------------------------------

export type ProducerKind = "model" | "tool" | "agent";

export interface OutlineRowVm {
  id: string;
  depth: number;
  kind: string;
  label: string;
  detail?: string;
  producer?: { kind: ProducerKind; label: string };
  completeness: CompletenessState;
  checkpointAvailability?: CheckpointAvailabilityState;
  expanded?: boolean;
  hasChildren?: boolean;
  selected?: boolean;
  /** Row-level safety/status words — never color alone (e.g. "FAILED"). */
  badges?: readonly string[];
  /** Flat-form non-tree causal edge annotation, e.g. "caused by op_31". */
  causalAnnotation?: string;
  /** Positioned against the outline's shared, core-supplied time scale. */
  duration?: OutlineDurationVm;
  checkpoint?: {
    id: string;
    operationCount: number;
    availability: CheckpointAvailabilityState;
    evidenceAge: string;
  };
  /** Present on a Delegation row — target trace to follow with `]`. */
  linkedTraceId?: string;
}

export interface OutlineDurationVm {
  /** Fraction [0,1] of the shared scale where this bar begins. */
  startFraction: number;
  /** Fraction [0,1] of the shared scale this bar spans. */
  extentFraction: number;
  seconds: number;
  style: "normal" | "aggregate" | "failed" | "delegation" | "open";
}

export interface OutlineVm {
  rows: readonly OutlineRowVm[];
  scaleSeconds: number;
  operationCount: number;
  agentStepCount: number;
  checkpointCount: number;
}

// ---------------------------------------------------------------------------
// Operation inspector
// ---------------------------------------------------------------------------

export type SensitiveValueVm =
  | { kind: "plain"; value: string }
  | { kind: "protected"; caption: string }
  | { kind: "write-only"; caption: string };

export interface InputFieldVm {
  name: string;
  type: string;
  meta?: string;
  value: SensitiveValueVm;
}

export interface OperationDetailVm {
  eyebrow: string;
  title: string;
  operationId: string;
  causalPosition: string;
  lifecycle: string;
  runtime: string;
  checkpointAvailability?: string;
  completeness: string;
  trust: string;
  protectedStore: string;
  badges: readonly string[];
  inputs?: readonly InputFieldVm[];
  permittedActions: readonly string[];
  /** True when a "Fork from this checkpoint" CTA card should render. */
  forkable?: boolean;
}

// ---------------------------------------------------------------------------
// Command palette (design section p03)
// ---------------------------------------------------------------------------

export interface CommandVm {
  id: string;
  label: string;
  shortcut?: string;
  disabledReason?: string;
  group?: string;
  badge?: string;
}

export interface PaletteVm {
  scopeLabel: string;
  groups: readonly { label?: string; commands: readonly CommandVm[] }[];
}

export interface HoldHintVm {
  eyebrow: string;
  chips: readonly { key: string; label: string; disabled?: boolean }[];
}

// ---------------------------------------------------------------------------
// Fork Checkpoint + Intervention (design sections p04-p05)
// ---------------------------------------------------------------------------

export interface CheckpointVm {
  id: string;
  operationId: string;
  operationLabel: string;
  operationCount: number;
  availability: CheckpointAvailabilityState;
  evidenceAge: string;
  supportedModes: readonly ("sandboxed" | "live")[];
}

export type InterventionFieldVm =
  | {
      kind: "diff";
      name: string;
      type: string;
      changed: true;
      before: string;
      after: string;
    }
  | {
      kind: "numeric";
      name: string;
      type: string;
      changed: true;
      before: number;
      after: number;
      min: number;
      max: number;
    }
  | {
      kind: "omitted";
      name: string;
      type: string;
      changed: false;
      checkpointValue: string;
    }
  | {
      kind: "protected";
      name: string;
      type: string;
      changed: boolean;
    }
  | {
      kind: "write-only";
      name: string;
      type: string;
      changed: boolean;
      draft: string;
    };

export interface ValidationVm {
  blocked: boolean;
  blockedReason?: string;
  invalidField?: string;
  invalidReason?: string;
}

export type ExecutionMode = "sandboxed" | "live";

export interface ForkRequestDraftVm {
  requestId: string;
  checkpoint: CheckpointVm;
  fields: readonly InterventionFieldVm[];
  changedCount: number;
  validation: ValidationVm;
  mode: ExecutionMode;
}

// ---------------------------------------------------------------------------
// Fork Request lifecycle (design section p06) — seven treatments
// ---------------------------------------------------------------------------

export type ForkRequestPhase =
  | "recovering"
  | "preparing"
  | "uncertain"
  | "rejected"
  | "cancelled"
  | "accepted";

export interface ForkRequestVm {
  requestId: string;
  phase: ForkRequestPhase;
  sourceTraceId: string;
  recovering?: {
    runtimeName: string;
    profileName: string;
    attempt: number;
    maxAttempts: number;
    elapsedSeconds: number;
  };
  preparing?: {
    checkpointId: string;
    checklist: readonly { label: string; done: boolean }[];
  };
  uncertain?: { checkpointId: string };
  rejected?: { reason: string };
  cancelled?: { byUser: boolean };
  accepted?: {
    childTraceId: string;
    mode: ExecutionMode;
    childFailure?: { operationLabel: string; reason: string };
  };
}

// ---------------------------------------------------------------------------
// Linked navigation + branch comparison (design section p07)
// ---------------------------------------------------------------------------

export interface ComparisonRowVm {
  id: string;
  label: string;
  duration?: string;
  badges?: readonly string[];
  receiving?: boolean;
  tint?: "none" | "success";
}

export interface ComparisonVm {
  originalTraceId: string;
  forkTraceId: string;
  sharedOperationCount: number;
  sharedSummaryRows: readonly { label: string; note: string }[];
  checkpoint: {
    id: string;
    label: string;
    operationId: string;
    interventionSummary: string;
  };
  original: {
    outcome: string;
    rows: readonly ComparisonRowVm[];
    footnote: string;
  };
  fork: {
    outcome: string;
    badge: string;
    rows: readonly ComparisonRowVm[];
    footnote: string;
  };
  summary: string;
}

// ---------------------------------------------------------------------------
// Live trace deltas
// ---------------------------------------------------------------------------

export interface TraceDeltaVm {
  appendedOperationCount: number;
  updatedRowIds: readonly string[];
}

// ---------------------------------------------------------------------------
// App-level scenes (design section p08)
// ---------------------------------------------------------------------------

export interface DegradedGapVm {
  fromOperationId: string;
  toOperationId: string;
  operationCount: number;
  seconds: number;
}

// ---------------------------------------------------------------------------
// Composite workspace view model
// ---------------------------------------------------------------------------

export interface TraceWorkspaceVm {
  header: TraceHeaderVm;
  links: readonly TraceLinkVm[];
  outline: OutlineVm;
  selectedId: string;
  detailsById: Readonly<Record<string, OperationDetailVm>>;
  degraded?: DegradedGapVm;
}
