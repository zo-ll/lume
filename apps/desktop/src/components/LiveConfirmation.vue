<script setup lang="ts">
// Deliberately no default focus and no Enter-to-submit on the confirmation
// field — friction is the point (design section p05).
import { computed, ref } from "vue";
import type { CheckpointVm } from "../view-models";

const props = defineProps<{ checkpoint: CheckpointVm; requestId: string }>();
const emit = defineEmits<{ submit: []; cancel: []; "switch-sandboxed": [] }>();

const typed = ref("");
const confirmed = computed(() => typed.value === "LIVE");
</script>

<template>
  <div class="live-confirmation">
    <div class="hazard-stripe" aria-hidden="true" />
    <div class="live-confirmation-header">
      <span class="badge badge--danger-fill">LIVE ENVIRONMENT</span>
      <h3>This fork will cause real side effects</h3>
    </div>

    <dl class="confirm-facts facts">
      <div>
        <dt>Runtime</dt>
        <dd>
          rt_support · trusted, launched from profile <code>support-local</code>
        </dd>
      </div>
      <div>
        <dt>Resuming from</dt>
        <dd>
          {{ checkpoint.operationId }} · {{ checkpoint.id }} ·
          {{ checkpoint.operationCount }} Operations
        </dd>
      </div>
      <div>
        <dt>Changing</dt>
        <dd>
          <code>system_prompt</code>, <code>temperature</code> · 3 fields
          omitted
        </dd>
      </div>
      <div>
        <dt>Will re-execute</dt>
        <dd><code>ticket.reply</code> — sends a message to a customer</dd>
      </div>
    </dl>

    <p class="confirm-note">Type <strong>LIVE</strong> to confirm.</p>
    <div class="confirm-input-row">
      <input
        v-model="typed"
        class="confirm-input"
        type="text"
        aria-label="Type LIVE to confirm"
        autocomplete="off"
        @keydown.enter.prevent
      />
    </div>
    <p class="confirm-caption">
      No default focus on this field, and no ⏎ shortcut for this button. The
      hatched header rule, the word, the border, and the typed confirmation are
      four independent signals.
    </p>

    <div class="stage-footer">
      <button
        class="btn danger"
        type="button"
        :disabled="!confirmed"
        @click="emit('submit')"
      >
        Submit live Fork Request
      </button>
      <span class="spacer" />
      <button
        class="link-action"
        type="button"
        @click="emit('switch-sandboxed')"
      >
        Switch to sandboxed
      </button>
      <button class="link-action" type="button" @click="emit('cancel')">
        Cancel
      </button>
    </div>
  </div>
</template>
