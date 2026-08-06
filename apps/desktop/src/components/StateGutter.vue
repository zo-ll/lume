<script setup lang="ts">
// Pure presentation: renders the outline's two independent legend slots
// (Trace Completeness, Fork Checkpoint Availability) and nothing else. It
// never decides what state a row is in — see docs/design section "00 · State
// encoding" — and every mark carries an aria-label equal to its legend word
// so the state is never conveyed by color or shape alone.
import { computed } from "vue";
import type {
  CheckpointAvailabilityState,
  CompletenessState,
} from "../view-models";

const props = defineProps<{
  completeness: CompletenessState;
  checkpointAvailability?: CheckpointAvailabilityState;
}>();

const completenessLabel: Record<CompletenessState, string> = {
  complete: "Complete",
  provisional: "Provisional",
  incomplete: "Incomplete",
  unverified: "Unverified",
};

const availabilityLabel: Record<CheckpointAvailabilityState, string> = {
  available: "Fork Checkpoint available",
  unknown: "Fork Checkpoint availability unknown",
  unavailable: "Fork Checkpoint unavailable",
};

const availabilityClass = computed(
  () => props.checkpointAvailability ?? "none",
);
</script>

<template>
  <span class="row-state">
    <span
      class="gutter-completeness"
      :class="completeness"
      role="img"
      :aria-label="completenessLabel[completeness]"
    />
    <span
      class="gutter-checkpoint"
      :class="availabilityClass"
      role="img"
      :aria-label="
        checkpointAvailability
          ? availabilityLabel[checkpointAvailability]
          : 'Not a Fork Checkpoint'
      "
      >{{ checkpointAvailability === "unavailable" ? "╳" : "" }}</span
    >
  </span>
</template>
