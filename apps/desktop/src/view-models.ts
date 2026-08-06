export type EvidenceState = "complete" | "failed" | "unverified" | "receiving";

export interface TraceHeaderVm {
  traceId: string;
  agentName: string;
  completeness: string;
  outcome: string;
  trust: string;
  session: string;
  runtime: string;
  store: string;
}

export interface TraceLinkVm {
  traceId: string;
  relation?: "fork" | "delegates";
  current: boolean;
}

export interface OutlineRowVm {
  id: string;
  depth: number;
  kind: string;
  label: string;
  detail?: string;
  duration?: string;
  state: EvidenceState;
  expanded?: boolean;
  hasChildren?: boolean;
  checkpoint?: {
    id: string;
    operationCount: number;
    availability: string;
    evidenceAge: string;
  };
}

export interface OperationDetailVm {
  eyebrow: string;
  title: string;
  operationId: string;
  causalPosition: string;
  lifecycle: string;
  runtime: string;
  checkpointAvailability: string;
  completeness: string;
  trust: string;
  protectedStore: string;
  permittedActions: readonly string[];
}

export interface TraceWorkspaceVm {
  header: TraceHeaderVm;
  links: readonly TraceLinkVm[];
  rows: readonly OutlineRowVm[];
  selectedId: string;
  detailsById: Readonly<Record<string, OperationDetailVm>>;
}
