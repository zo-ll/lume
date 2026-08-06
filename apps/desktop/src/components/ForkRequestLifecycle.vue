<script setup lang="ts">
// A Fork Request is not a trace. Until Fork Acceptance there is no child,
// and this component shows none — no placeholder row, no optimistic lineage
// entry (design section p06).
import type { ForkRequestVm } from "../view-models";

defineProps<{ model: ForkRequestVm }>();
defineEmits<{
  cancel: [];
  retry: [];
  "choose-another": [];
  discard: [];
  reopen: [];
}>();
</script>

<template>
  <div
    class="lifecycle-card"
    :class="{
      uncertain: model.phase === 'uncertain',
      rejected: model.phase === 'rejected',
      accepted: model.phase === 'accepted',
      'span-2': model.phase === 'accepted' && model.accepted?.childFailure,
    }"
  >
    <!-- RECOVERING -->
    <template v-if="model.phase === 'recovering'">
      <div class="lifecycle-head">
        <span class="badge badge--neutral">RECOVERING</span>
        <span class="spacer" />
        <span class="badge badge--fill-neutral">SANDBOXED</span>
      </div>
      <h4>Reconnecting to {{ model.recovering!.runtimeName }}</h4>
      <p>
        The owning runtime is not connected. Lume is launching it from the
        reviewed Recovery Profile
        <code>{{ model.recovering!.profileName }}</code
        >.
      </p>
      <div class="lifecycle-progress">
        <div class="fill" style="width: 38%" />
      </div>
      <div class="lifecycle-footer">
        <span
          >attempt {{ model.recovering!.attempt }} of
          {{ model.recovering!.maxAttempts }} ·
          {{ model.recovering!.elapsedSeconds }}s elapsed</span
        >
        <button class="link-action" type="button" @click="$emit('cancel')">
          Cancel
        </button>
      </div>
    </template>

    <!-- PREPARING -->
    <template v-else-if="model.phase === 'preparing'">
      <div class="lifecycle-head">
        <span class="badge badge--neutral">PREPARING</span>
        <span class="spacer" />
        <span class="badge badge--fill-neutral">SANDBOXED</span>
      </div>
      <h4>Runtime is restoring {{ model.preparing!.checkpointId }}</h4>
      <ul class="lifecycle-checklist">
        <li
          v-for="item in model.preparing!.checklist"
          :key="item.label"
          :class="{ done: item.done }"
        >
          <span class="mark">{{ item.done ? "✓" : "·" }}</span
          >{{ item.label }}
        </li>
      </ul>
      <div class="lifecycle-footer">
        <span>No child trace exists yet.</span>
      </div>
    </template>

    <!-- UNCERTAIN -->
    <template v-else-if="model.phase === 'uncertain'">
      <div class="lifecycle-head">
        <span class="badge badge--amber-fill">UNCERTAIN</span>
      </div>
      <h4>
        <span
          class="gutter-checkpoint unknown"
          role="img"
          aria-label="Checkpoint availability unknown"
        />
        Checkpoint availability is unknown
      </h4>
      <p>
        The runtime reconnected but has not answered the availability probe.
        Lume will not report the checkpoint as available on stale evidence, and
        will not submit until the runtime answers.
      </p>
      <div class="lifecycle-actions">
        <button class="btn" type="button" @click="$emit('retry')">
          Probe again
        </button>
        <button class="link-action" type="button" @click="$emit('cancel')">
          Cancel request
        </button>
      </div>
    </template>

    <!-- REJECTED -->
    <template v-else-if="model.phase === 'rejected'">
      <div class="lifecycle-head">
        <span class="badge badge--danger-fill">REJECTED</span>
      </div>
      <p class="lifecycle-context">
        {{ model.requestId }} · no Trace Fork was created
      </p>
      <h4>Runtime refused restoration</h4>
      <p>
        Reported: <code>{{ model.rejected!.reason }}</code>
      </p>
      <p>
        The rejection happened before Fork Acceptance, so nothing was executed
        and no ancestry changed. Your Intervention is kept.
      </p>
      <div class="lifecycle-actions">
        <button class="btn" type="button" @click="$emit('choose-another')">
          Choose another checkpoint
        </button>
        <button class="link-action" type="button" @click="$emit('discard')">
          Discard request
        </button>
      </div>
    </template>

    <!-- CANCELLED -->
    <template v-else-if="model.phase === 'cancelled'">
      <div class="lifecycle-head">
        <span class="badge badge--neutral">CANCELLED</span>
      </div>
      <p class="lifecycle-context">
        {{ model.requestId }} · by you, before acceptance
      </p>
      <h4>Request cancelled</h4>
      <p>
        Cancellation was requested during preparation and the runtime confirmed
        it before committing. No Trace Fork exists.
      </p>
      <p>
        Had the runtime already committed, this panel would instead report
        acceptance — cancellation is never claimed retroactively.
      </p>
      <div class="lifecycle-actions">
        <button class="btn" type="button" @click="$emit('reopen')">
          Reopen with the same Intervention
        </button>
      </div>
    </template>

    <!-- ACCEPTED (+ optional immediate child failure) -->
    <template v-else-if="model.phase === 'accepted'">
      <div class="lifecycle-head">
        <span class="badge badge--accent-fill">ACCEPTED</span>
        <span
          v-if="model.accepted!.childFailure"
          class="badge badge--danger-outline"
          >CHILD FAILED IMMEDIATELY</span
        >
      </div>
      <p class="lifecycle-context">
        {{ model.requestId }} → {{ model.accepted!.childTraceId }}
      </p>

      <template v-if="!model.accepted!.childFailure">
        <h4>Trace Fork {{ model.accepted!.childTraceId }} created</h4>
        <p>
          The runtime committed restoration and declared the child. Fork Link
          established. Focus has moved to the child trace.
        </p>
        <div class="lifecycle-mini-lineage">
          <span>{{ model.sourceTraceId }}</span
          ><span class="relation">──fork──▸</span
          ><span>{{ model.accepted!.childTraceId }}</span>
          <span class="badge badge--fill-neutral">SANDBOXED</span>
        </div>
      </template>
      <template v-else>
        <h4>The fork exists and its first Operation failed</h4>
        <p>
          Acceptance already happened, so this is not a rejected request.
          <code>{{ model.accepted!.childTraceId }}</code> is a real Trace Fork
          with real ancestry, and it owns this failure. Deleting the request
          would not remove it.
        </p>
        <div class="lifecycle-mini-outline">
          <div class="mini-head">
            <span
              >{{ model.accepted!.childTraceId }} · support-triage-agent</span
            >
            <span class="badge badge--neutral">COMPLETENESS: INCOMPLETE</span>
          </div>
          <div class="mini-row">
            <span
              class="gutter-completeness incomplete"
              role="img"
              aria-label="Incomplete"
            />
            <span>{{ model.accepted!.childFailure.operationLabel }}</span>
            <span class="badge badge--danger-outline">{{
              model.accepted!.childFailure.reason
            }}</span>
          </div>
        </div>
      </template>
    </template>
  </div>
</template>
