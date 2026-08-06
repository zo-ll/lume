import type {
  CheckpointVm,
  InterventionFieldVm,
  ValidationVm,
} from "../view-models";

// Transcribed from the approved design's Intervention editing and Execution
// Mode stages (sections p04-p05).

export const forkCheckpoint: CheckpointVm = {
  id: "cut_0f2",
  operationId: "op_44",
  operationLabel: "Model Call · draft.compose",
  operationCount: 43,
  availability: "available",
  evidenceAge: "4s old",
  supportedModes: ["sandboxed", "live"],
};

const systemPromptBefore = "Resolve entitlement before drafting a reply.";
const systemPromptAfter =
  "Resolve entitlement before drafting a reply. If verification fails twice, escalate rather than retrying.";

/** The happy-path field set: two changed fields, three left as-is. */
export const interventionFields: readonly InterventionFieldVm[] = [
  {
    kind: "diff",
    name: "system_prompt",
    type: "string · maxLength 4096",
    changed: true,
    before: systemPromptBefore,
    after: systemPromptAfter,
  },
  {
    kind: "numeric",
    name: "temperature",
    type: "float · 0.0–2.0",
    changed: true,
    before: 0.7,
    after: 0.2,
    min: 0,
    max: 2,
  },
  {
    kind: "omitted",
    name: "max_tokens",
    type: "int · 1–8192",
    changed: false,
    checkpointValue: "2048 from checkpoint",
  },
  {
    kind: "protected",
    name: "customer_record",
    type: "object · protected",
    changed: false,
  },
  {
    kind: "write-only",
    name: "upstream_api_token",
    type: "string · write-only",
    changed: false,
    draft: "",
  },
];

export const interventionValidationOk: ValidationVm = { blocked: false };

/** Every field left omitted — a no-op Intervention. */
export const interventionFieldsBlocked: readonly InterventionFieldVm[] = [
  {
    kind: "omitted",
    name: "system_prompt",
    type: "string · maxLength 4096",
    changed: false,
    checkpointValue: "unchanged from checkpoint",
  },
  {
    kind: "omitted",
    name: "temperature",
    type: "float · 0.0–2.0",
    changed: false,
    checkpointValue: "0.7 from checkpoint",
  },
  {
    kind: "omitted",
    name: "max_tokens",
    type: "int · 1–8192",
    changed: false,
    checkpointValue: "2048 from checkpoint",
  },
  {
    kind: "protected",
    name: "customer_record",
    type: "object · protected",
    changed: false,
  },
  {
    kind: "write-only",
    name: "upstream_api_token",
    type: "string · write-only",
    changed: false,
    draft: "",
  },
];

export const interventionValidationBlocked: ValidationVm = {
  blocked: true,
  blockedReason:
    "Every field is omitted, so this request would produce a Trace Fork identical to its source. Change at least one Intervention Field to continue.",
};

export const interventionValidationInvalid: ValidationVm = {
  blocked: false,
  invalidField: "temperature",
  invalidReason:
    "temperature must be between 0.0 and 2.0. Validation is the Rust core's answer, echoed here verbatim; the frontend does not decide what is valid.",
};

export const invalidTemperatureField: InterventionFieldVm = {
  kind: "numeric",
  name: "temperature",
  type: "float · 0.0–2.0",
  changed: true,
  before: 0.7,
  after: 2.6,
  min: 0,
  max: 2,
};

/** Same field set as the happy path, but temperature actually violates the
 * 0.0-2.0 constraint the invalid-state fixture describes. */
export const interventionFieldsInvalid: readonly InterventionFieldVm[] =
  interventionFields.map((field) =>
    field.kind === "numeric" && field.name === "temperature"
      ? invalidTemperatureField
      : field,
  );
