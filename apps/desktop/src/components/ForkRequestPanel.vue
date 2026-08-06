<script setup lang="ts">
// Stage host for Fork Request creation. Stage index is local UI state; which
// stages are reachable and whether Continue is enabled comes from the core
// (ValidationVm), never derived here (design section p09).
import { computed, onBeforeUnmount, onMounted, ref } from "vue";
import type {
  CheckpointVm,
  ExecutionMode,
  InterventionFieldVm,
  OutlineRowVm,
  ValidationVm,
} from "../view-models";
import StateGutter from "./StateGutter.vue";
import InterventionForm from "./InterventionForm.vue";
import ExecutionModeChoice from "./ExecutionModeChoice.vue";
import LiveConfirmation from "./LiveConfirmation.vue";

const props = defineProps<{
  requestId: string;
  checkpoint: CheckpointVm;
  fields: readonly InterventionFieldVm[];
  validation: ValidationVm;
  outlineRows?: readonly OutlineRowVm[];
  initialStage?: 2 | 3 | 4;
  initialMode?: ExecutionMode;
}>();

const emit = defineEmits<{ cancel: []; submit: [mode: ExecutionMode] }>();

const stage = ref<2 | 3 | 4>(props.initialStage ?? 2);
const mode = ref<ExecutionMode>(props.initialMode ?? "sandboxed");

const checkpointIndex = computed(() =>
  (props.outlineRows ?? []).findIndex(
    (r) => r.id === props.checkpoint.operationId,
  ),
);
const beforeRows = computed(() =>
  checkpointIndex.value >= 0
    ? (props.outlineRows ?? []).slice(0, checkpointIndex.value)
    : [],
);
const afterRows = computed(() =>
  checkpointIndex.value >= 0
    ? (props.outlineRows ?? []).slice(checkpointIndex.value + 1)
    : [],
);

const changedCount = computed(
  () => props.fields.filter((f) => f.changed).length,
);

const interventionBlocked = computed(
  () => props.validation.blocked || !!props.validation.invalidField,
);

function continueFromIntervention() {
  if (interventionBlocked.value) return;
  stage.value = 3;
}
function continueFromMode() {
  if (mode.value === "sandboxed") {
    emit("submit", "sandboxed");
  } else {
    stage.value = 4;
  }
}

// Esc leaves the current stage; from stage one (the checkpoint row already
// selected, before this panel opens) it cancels the Fork Request with
// confirmation — here, at the first stage this panel actually renders,
// that same rule applies (design section p03's keyboard model).
function cancelWithConfirmation() {
  if (
    window.confirm(
      "Cancel this Fork Request? Your Intervention will be discarded.",
    )
  ) {
    emit("cancel");
  }
}
function onKeydown(event: KeyboardEvent) {
  if (event.key !== "Escape") return;
  event.preventDefault();
  if (stage.value === 2) cancelWithConfirmation();
  else if (stage.value === 3) stage.value = 2;
  else if (stage.value === 4) stage.value = 3;
}
onMounted(() => window.addEventListener("keydown", onKeydown));
onBeforeUnmount(() => window.removeEventListener("keydown", onKeydown));
</script>

<template>
  <div class="fork-flow">
    <div class="fork-outline-panel">
      <div class="outline-header">
        <span class="outline-header-cell col-operation"
          >Trace · context retained</span
        >
      </div>
      <div
        v-for="row in beforeRows"
        :key="row.id"
        class="outline-row fork-outline-row before-cut"
        :style="{ '--depth': row.depth }"
      >
        <StateGutter
          :completeness="row.completeness"
          :checkpoint-availability="row.checkpointAvailability"
        />
        <span class="row-indent" />
        <span class="row-copy"
          ><span class="row-kind">{{ row.kind }}</span>
          <span class="row-detail">{{ row.label }}</span></span
        >
      </div>

      <div class="fork-pin">
        <div class="eyebrow">Forking from</div>
        <div class="title">{{ checkpoint.operationLabel }}</div>
        <div class="subline">
          {{ checkpoint.operationId }} · {{ checkpoint.id }} ·
          {{ checkpoint.operationCount }} Operations included
        </div>
        <div class="badges">
          <span class="badge badge--accent-outline"
            >AVAILABILITY: AVAILABLE</span
          >
          <span class="badge badge--accent-outline"
            >EVIDENCE {{ checkpoint.evidenceAge.toUpperCase() }}</span
          >
        </div>
      </div>

      <div
        v-for="row in afterRows"
        :key="row.id"
        class="outline-row fork-outline-row after-cut"
        :style="{ '--depth': row.depth }"
      >
        <StateGutter
          :completeness="row.completeness"
          :checkpoint-availability="row.checkpointAvailability"
        />
        <span class="row-indent" />
        <span class="row-copy"
          ><span class="row-kind">{{ row.kind }}</span>
          <span class="row-detail">{{ row.label }}</span></span
        >
      </div>

      <p class="fork-outline-footnote">
        Operations after the cut are not part of the fork.
      </p>
    </div>

    <div class="stage-host">
      <div class="stepper">
        <h2>Fork Request</h2>
        <div class="step complete">
          <span class="step-circle">✓</span
          ><span class="step-label">Checkpoint</span>
        </div>
        <span class="step-connector" />
        <div class="step" :class="stage === 2 ? 'current' : 'complete'">
          <span class="step-circle">{{ stage === 2 ? "2" : "✓" }}</span
          ><span class="step-label">Intervention</span>
        </div>
        <span class="step-connector" />
        <div
          class="step"
          :class="{ current: stage === 3, complete: stage === 4 }"
        >
          <span class="step-circle">{{ stage === 4 ? "✓" : "3" }}</span
          ><span class="step-label">Execution Mode</span>
        </div>
        <span class="step-connector" />
        <div class="step" :class="{ current: stage === 4 }">
          <span class="step-circle">4</span
          ><span class="step-label">Submit</span>
        </div>
        <span class="spacer" />
        <span class="request-id">{{ requestId }} · not yet a Trace Fork</span>
      </div>

      <div class="stage-body">
        <template v-if="stage === 2">
          <h3>Intervention Fields</h3>
          <p class="stage-counter">
            {{ fields.length }} declared by the checkpoint ·
            {{ changedCount }} changed
          </p>
          <InterventionForm :fields="fields" :validation="validation" />
        </template>

        <template v-else-if="stage === 3">
          <ExecutionModeChoice
            :checkpoint="checkpoint"
            :mode="mode"
            @update:mode="mode = $event"
          />
        </template>

        <LiveConfirmation
          v-if="stage === 4"
          :checkpoint="checkpoint"
          :request-id="requestId"
          @submit="emit('submit', 'live')"
          @switch-sandboxed="((mode = 'sandboxed'), (stage = 3))"
          @cancel="stage = 3"
        />
      </div>

      <div v-if="stage === 2" class="stage-footer">
        <button
          class="btn primary"
          type="button"
          :disabled="interventionBlocked"
          :aria-describedby="
            interventionBlocked ? 'intervention-block-reason' : undefined
          "
          @click="continueFromIntervention"
        >
          Continue to Execution Mode
        </button>
        <span class="btn-shortcut">⌘⏎</span>
        <span class="spacer" />
        <button class="link-action" type="button" @click="emit('cancel')">
          Cancel Fork Request <kbd>esc</kbd>
        </button>
      </div>
      <div v-else-if="stage === 3" class="stage-footer">
        <button class="btn primary" type="button" @click="continueFromMode">
          Continue
        </button>
        <span class="btn-shortcut">⌘⏎</span>
        <span class="spacer" />
        <button class="link-action" type="button" @click="stage = 2">
          Back to Intervention <kbd>esc</kbd>
        </button>
      </div>
    </div>
  </div>
</template>
