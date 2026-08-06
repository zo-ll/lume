<script setup lang="ts">
// Offers only checkpoint-supported modes; live requires LiveConfirmation
// (design section p09).
import type { CheckpointVm, ExecutionMode } from "../view-models";

defineProps<{ checkpoint: CheckpointVm; mode: ExecutionMode }>();
const emit = defineEmits<{ "update:mode": [mode: ExecutionMode] }>();
</script>

<template>
  <div>
    <h3>Fork Execution Mode</h3>
    <p class="stage-counter">
      Declared by the checkpoint. Both modes are supported here.
    </p>

    <div
      class="mode-options"
      role="radiogroup"
      aria-label="Fork Execution Mode"
    >
      <button
        v-if="checkpoint.supportedModes.includes('sandboxed')"
        class="mode-option"
        :class="{ selected: mode === 'sandboxed' }"
        type="button"
        role="radio"
        :aria-checked="mode === 'sandboxed'"
        @click="emit('update:mode', 'sandboxed')"
      >
        <span class="mode-radio"><span class="dot" /></span>
        <span class="mode-body">
          <span class="mode-title-row">
            <span class="mode-title">Sandboxed environment</span>
            <span class="badge badge--fill-neutral">SANDBOXED</span>
          </span>
          <p>
            Tool calls are routed to the runtime's sandbox. No external side
            effects. The default for every checkpoint that supports it.
          </p>
        </span>
      </button>

      <button
        v-if="checkpoint.supportedModes.includes('live')"
        class="mode-option live"
        :class="{ selected: mode === 'live' }"
        type="button"
        role="radio"
        :aria-checked="mode === 'live'"
        @click="emit('update:mode', 'live')"
      >
        <span class="mode-radio"><span class="dot" /></span>
        <span class="mode-body">
          <span class="mode-title-row">
            <span class="mode-title">Live environment</span>
            <span class="badge badge--danger-fill">LIVE ENVIRONMENT</span>
          </span>
          <p>
            Tool calls execute for real. Messages send, records change, payments
            move. Requires a separate confirmation before submission.
          </p>
        </span>
      </button>
    </div>
  </div>
</template>
