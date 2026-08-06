# Define the Intervention and Sensitive Field contract

Parent: [Specify Lume v1 causal debugging and trace forking](../map.md)
Type: `grilling`
Status: `resolved`
Blocked by: 01, 03

## Question

How does a Fork Checkpoint describe its editable inputs, types, constraints, defaults, validation errors, and Sensitive Fields so Lume can render and submit a safe, generic Intervention without understanding agent-specific state?

## Answer

Every Fork Checkpoint contains an immutable Intervention schema that defines the only runtime state a user may change when creating a Trace Fork. Lume renders and locally validates this schema without understanding agent-specific state; the owning runtime remains the authority that validates and applies the resulting Intervention.

### Intervention schema

The schema is versioned and bound immutably to one checkpoint. Its semantic shape is:

```text
schema_version
fields[]

field:
  field_id
  type
  nullable
  label
  help?
  current_value?
  default_value?
  constraints[]
  sensitivity
  rendering_hints?
  children?             # object properties or list item schema
```

Every Intervention Field has an opaque `field_id` stable within the checkpoint schema. Interventions, validation errors, audit records, and runtime responses identify fields by this ID rather than by display labels or fragile structural paths. Structural position still defines object and list shape, but it is not field identity.

Labels and help text are human-facing. Namespaced rendering hints may request presentation such as a multiline editor or masked input, but they cannot change type, validation, sensitivity, or submission semantics. Lume may ignore an unknown hint without changing the meaning of the field.

### Portable type system

The v1 semantic types are deliberately small:

- `string`
- `integer`
- `number`
- `boolean`
- `enum`
- `object`
- `list`

Nullability is explicit and orthogonal to type. An enum declares its finite typed choices. An object declares identified child fields. A list declares its item schema. Runtimes may add namespaced hints, but an unsupported custom type cannot silently degrade into an editable string; Lume must present it as unsupported and prevent submission.

The contract is Lume-owned rather than the full JSON Schema language. This keeps validation and cross-platform rendering portable and prevents runtime-provided executable validation logic.

### Sparse change semantics

An Intervention is a sparse mapping from `field_id` to explicitly supplied value:

```text
checkpoint_id
schema_version
changes:
  field_id -> explicit value, including explicit null
```

- Omission means retain the value in checkpoint state.
- Explicit null means replace the value with null and is valid only for a nullable field.
- An explicit container value replaces that submitted field's value; v1 has no generic path-based patch language.
- A nested field may be changed independently when its schema gives it its own field ID.

`current_value` is the checkpoint value Lume may present before editing when its disclosure policy permits. `default_value` is an optional runtime suggestion for a replacement. A default is never applied merely because a field is omitted, and resetting to a default is an explicit change.

An Intervention must contain at least one effective change. Lume rejects an empty change set locally. During authoritative validation the runtime compares submitted values with restored checkpoint state and rejects a semantically unchanged set with a stable `no_effective_change` code. For a write-only field, the runtime performs this comparison without disclosing either value.

### Declarative constraints

Lume validates a portable bounded constraint vocabulary:

- numeric minimum and maximum, with inclusive or exclusive bounds;
- string minimum and maximum length and a portable regular-expression pattern;
- enum membership;
- list minimum and maximum size; and
- required object-child presence.

Constraints apply only when a field is explicitly changed, except that the runtime validates the resulting restored state as a whole. Cross-field, agent-specific, state-dependent, compatibility, and external-resource rules remain runtime validation. Schemas contain data, not executable expressions or scripts.

### Two-stage validation

Validation occurs twice:

1. Lume validates schema version, field identities, explicit presence versus null, types, portable constraints, and local sensitivity-handling requirements. This provides immediate form feedback and prevents obviously invalid submissions.
2. During atomic fork preparation, the owning runtime restores or inspects the checkpoint state, reapplies all declarative rules, validates cross-field and domain rules, checks that an effective change exists, and decides authoritatively whether the Intervention can be applied.

A successful local validation never guarantees runtime acceptance. Runtime rejection changes neither the checkpoint nor its saved state and creates no Trace Fork.

Every validation issue contains:

```text
stage                 # lume or runtime
code                  # stable, namespaced where runtime-defined
field_ids[]           # one or more affected fields; may be empty for form-level errors
message
details?              # typed and sensitivity-governed
```

Errors are ordered deterministically for stable client presentation. Messages aid humans; automation depends on stage, code, and field IDs. Error content must obey the strongest sensitivity policy of the fields it references and must not echo protected or write-only values.

### Sensitive Fields

Sensitivity is explicit runtime metadata, not a promise of automatic secret detection. Every field declares one policy:

- `ordinary`: normal local presentation and persistence rules apply.
- `protected`: the value may be retained only through protected local storage, is masked by default, and requires an explicit user action to reveal.
- `write_only`: the checkpoint value is never disclosed to Lume. A replacement may exist in Lume memory only while the user edits and submits it; Lume never persists, redisplays, logs, exports, or includes the value in diagnostics.

Nested fields inherit the strongest enclosing policy. A child may strengthen but never weaken its parent's policy. Rendering hints cannot override sensitivity.

Protected reveal is a deliberate UI action and applies only to the selected value; selecting a checkpoint or opening an Intervention form never reveals it automatically. Ticket 08 defines the protected local storage mechanism and retention behavior.

For a write-only field, the schema may disclose whether a checkpoint value exists but never its value. Leaving the field untouched preserves it. Clearing it requires an explicit nullable value or a separate typed replacement permitted by the schema; blank display text never implicitly clears a secret.

### Submission and audit record

Lume sends only explicit changes over the authenticated runtime-control channel. Sensitivity metadata accompanies the values so transport, logs, diagnostics, and runtime responses preserve the field policy. The runtime must not echo sensitive submitted values.

Lume retains a policy-aware Intervention audit record:

- ordinary submitted values are retained normally;
- protected submitted values are retained only through protected local storage and remain hidden by default; and
- write-only submitted values are represented only by `field_id` and a `changed` marker.

The audit record also retains the checkpoint and schema identities, validation outcome, field-level error metadata after policy filtering, and the resulting fork identity once created. It never substitutes for the runtime-owned restored state.

### Acceptance examples

- Omitting an editable temperature field preserves its checkpoint value; submitting `0` explicitly changes it.
- Omitting a nullable system prompt preserves it; submitting null explicitly clears it.
- A list replacement is validated against item type and size constraints before submission.
- Two fields with the same label remain unambiguous because errors and changes use stable field IDs.
- Lume may accept two individually valid numeric fields locally while the runtime rejects their combination through a cross-field rule.
- Opening a form shows a protected value masked and a write-only value as undisclosed; neither is revealed automatically.
- After a write-only credential replacement, history records that the field changed but cannot reveal the submitted credential.
- Submitting no fields, or values the runtime determines are equivalent to checkpoint state, creates no Trace Fork.
