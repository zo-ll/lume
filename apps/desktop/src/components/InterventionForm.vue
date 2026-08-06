<script setup lang="ts">
// Field editors keyed by structural type. Holds draft text and omission
// flags only — this is local, transient presentation state; it is never
// written back into the field props, and it never recomputes `validation`
// (blocked/invalid/changed-count stay exactly what the core supplied,
// consistent with "the frontend does not decide what is valid"). A real
// adapter would receive `update:draft` and send a sparse patch, never a
// full state object (design section p09).
import { reactive } from "vue";
import type { InterventionFieldVm, ValidationVm } from "../view-models";
import SensitiveValue from "./SensitiveValue.vue";

const props = defineProps<{
  fields: readonly InterventionFieldVm[];
  validation: ValidationVm;
}>();

const emit = defineEmits<{
  /** value === null means the field was reverted to omitted. */
  "update:draft": [name: string, value: string | number | null];
}>();

// Seed local draft state from the initial fixture. Editing never mutates
// `props.fields` — only these local copies.
const stringDrafts = reactive<Record<string, string>>({});
const numericDrafts = reactive<Record<string, number>>({});
const writeOnlyDrafts = reactive<Record<string, string>>({});
const omittedOverride = reactive<Record<string, boolean>>({});

for (const field of props.fields) {
  if (field.kind === "diff") stringDrafts[field.name] = field.after;
  else if (field.kind === "numeric") numericDrafts[field.name] = field.after;
  else if (field.kind === "write-only")
    writeOnlyDrafts[field.name] = field.draft;
}

function isOmitted(field: InterventionFieldVm): boolean {
  if (field.kind === "omitted") return true;
  if (field.kind === "diff" || field.kind === "numeric")
    return !!omittedOverride[field.name];
  return false;
}

function editString(
  field: Extract<InterventionFieldVm, { kind: "diff" }>,
  value: string,
) {
  stringDrafts[field.name] = value;
  omittedOverride[field.name] = false;
  emit("update:draft", field.name, value);
}

function editNumber(
  field: Extract<InterventionFieldVm, { kind: "numeric" }>,
  value: number,
) {
  numericDrafts[field.name] = value;
  omittedOverride[field.name] = false;
  emit("update:draft", field.name, value);
}

function editWriteOnly(
  field: Extract<InterventionFieldVm, { kind: "write-only" }>,
  value: string,
) {
  writeOnlyDrafts[field.name] = value;
  emit("update:draft", field.name, value.length ? value : null);
}

function revert(field: InterventionFieldVm) {
  if (field.kind !== "diff" && field.kind !== "numeric") return;
  omittedOverride[field.name] = true;
  if (field.kind === "diff") stringDrafts[field.name] = field.before;
  else numericDrafts[field.name] = field.before;
  emit("update:draft", field.name, null);
}

function writeOnlyChanged(
  field: Extract<InterventionFieldVm, { kind: "write-only" }>,
): boolean {
  return writeOnlyDrafts[field.name]?.length > 0;
}
</script>

<template>
  <div>
    <p class="stage-explainer">
      An omitted field keeps checkpoint state. Omission, an explicit value, and
      an explicit null are three different things and are labelled as three
      different things.
    </p>

    <div class="intervention-list">
      <div
        v-for="field in fields"
        :key="field.name"
        class="intervention-field"
        :class="{
          changed:
            field.kind === 'write-only'
              ? writeOnlyChanged(field)
              : field.changed && !isOmitted(field),
          'write-only': field.kind === 'write-only',
        }"
      >
        <div class="field-header">
          <span class="field-name">{{ field.name }}</span>
          <span class="field-type">{{ field.type }}</span>
          <span class="spacer" />
          <span
            v-if="
              field.kind === 'write-only'
                ? writeOnlyChanged(field)
                : field.changed && !isOmitted(field)
            "
            class="badge badge--accent-fill"
            >CHANGED</span
          >
          <span
            v-else-if="field.kind === 'omitted' || isOmitted(field)"
            class="badge badge--neutral"
            >OMITTED — RETAINS CHECKPOINT STATE</span
          >
          <span
            v-else-if="field.kind === 'protected'"
            class="badge badge--dark-outline"
            >PROTECTED — STORE LOCKED</span
          >
          <span
            v-else-if="field.kind === 'write-only'"
            class="badge badge--dark-outline"
            >WRITE-ONLY — NEVER RETAINED</span
          >
        </div>

        <!-- string diff, editable -->
        <template v-if="field.kind === 'diff'">
          <template v-if="!isOmitted(field)">
            <div class="diff-block">
              <div class="diff-line removed">− {{ field.before }}</div>
              <div class="diff-line added">
                +
                {{
                  field.before === stringDrafts[field.name]
                    ? "(unchanged text, still an explicit value)"
                    : stringDrafts[field.name]
                }}
              </div>
            </div>
            <textarea
              class="editable-text-input"
              rows="2"
              :value="stringDrafts[field.name]"
              :aria-label="`Edit ${field.name}`"
              @input="
                editString(field, ($event.target as HTMLTextAreaElement).value)
              "
              @keydown.meta.z.prevent="revert(field)"
            />
            <p class="field-caption">
              Diff against checkpoint state · <kbd>⌘Z</kbd> or
              <button class="link-action" type="button" @click="revert(field)">
                revert
              </button>
              restores the checkpoint value and returns this field to omitted
            </p>
          </template>
          <template v-else>
            <p class="omitted-value">
              {{ field.before }}
              <span class="from-checkpoint">from checkpoint</span>
            </p>
            <p class="field-caption">
              <button
                class="link-action"
                type="button"
                @click="editString(field, field.after)"
              >
                Edit
              </button>
            </p>
          </template>
        </template>

        <!-- numeric, editable range -->
        <template v-else-if="field.kind === 'numeric'">
          <template v-if="!isOmitted(field)">
            <div class="slider-row">
              <span class="value-old">{{ field.before }}</span>
              <input
                class="slider-input"
                type="range"
                :min="field.min"
                :max="field.max"
                step="0.1"
                :value="numericDrafts[field.name]"
                :aria-label="`Edit ${field.name}`"
                @input="
                  editNumber(
                    field,
                    Number(($event.target as HTMLInputElement).value),
                  )
                "
              />
              <span class="value-new">{{
                numericDrafts[field.name].toFixed(1)
              }}</span>
            </div>
            <p class="field-caption">
              <button class="link-action" type="button" @click="revert(field)">
                Revert to checkpoint value
              </button>
            </p>
          </template>
          <template v-else>
            <p class="omitted-value">
              {{ field.before }}
              <span class="from-checkpoint">from checkpoint</span>
            </p>
            <p class="field-caption">
              <button
                class="link-action"
                type="button"
                @click="editNumber(field, field.after)"
              >
                Edit
              </button>
            </p>
          </template>
        </template>

        <template v-else-if="field.kind === 'omitted'">
          <p class="omitted-value">
            {{ field.checkpointValue.split(" from ")[0] }}
            <span class="from-checkpoint">from checkpoint</span>
          </p>
        </template>

        <template v-else-if="field.kind === 'protected'">
          <SensitiveValue
            :model="{
              kind: 'protected',
              caption:
                'Left omitted, the runtime restores its own protected value from the checkpoint. Lume never substitutes plaintext.',
            }"
          />
          <p class="field-caption">
            <button class="btn" type="button">
              Unlock Protected Store to edit
            </button>
          </p>
        </template>

        <!-- write-only, editable draft that is never displayed back -->
        <template v-else-if="field.kind === 'write-only'">
          <input
            class="write-only-input"
            type="text"
            :value="writeOnlyDrafts[field.name]"
            :aria-label="`Enter a new value for ${field.name}`"
            placeholder="enter a new value to send · nothing is shown back"
            @input="
              editWriteOnly(field, ($event.target as HTMLInputElement).value)
            "
          />
          <p class="field-caption">
            Lume holds no prior value for this field and will not retain what
            you type past submission. Leaving it empty omits it; the runtime
            supplies its own.
          </p>
        </template>
      </div>
    </div>

    <div
      v-if="validation.blocked"
      id="intervention-block-reason"
      class="blocking-card blocked"
    >
      <h4>
        <span class="badge badge--danger-fill">BLOCKED</span> No-op Intervention
      </h4>
      <p>
        {{ validation.blockedReason }} The Continue action stays visible and
        disabled, with this reason attached to it.
      </p>
    </div>
    <div
      v-else-if="validation.invalidField"
      id="intervention-block-reason"
      class="blocking-card invalid"
    >
      <h4>
        <span class="badge badge--amber-fill">INVALID</span> Constraint violated
      </h4>
      <p>{{ validation.invalidReason }}</p>
    </div>
  </div>
</template>
