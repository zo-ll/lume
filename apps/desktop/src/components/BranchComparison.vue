<script setup lang="ts">
// Renders one shared prefix and two suffixes from ComparisonVm (design
// section p09). Rows are not force-aligned to each other — the one
// alignment is the checkpoint itself, which spans both columns.
import { onBeforeUnmount, onMounted, ref } from "vue";
import type { ComparisonVm } from "../view-models";

defineProps<{ model: ComparisonVm }>();
const emit = defineEmits<{ close: [] }>();

const sharedExpanded = ref(false);
// ⌘D toggles back out of comparison (the same shortcut that opened it from
// investigation); ⇧⇥ swaps which branch is on which side (design section
// p07's footer hint).
const swapped = ref(false);

function onKeydown(event: KeyboardEvent) {
  if (event.key === "Escape") {
    event.preventDefault();
    emit("close");
  } else if (
    (event.metaKey || event.ctrlKey) &&
    event.key.toLowerCase() === "d"
  ) {
    event.preventDefault();
    emit("close");
  } else if (event.shiftKey && event.key === "Tab") {
    event.preventDefault();
    swapped.value = !swapped.value;
  }
}
onMounted(() => window.addEventListener("keydown", onKeydown));
onBeforeUnmount(() => window.removeEventListener("keydown", onKeydown));
</script>

<template>
  <div class="comparison-screen">
    <div class="comparison-header">
      <span class="wordmark">LUME</span>
      <span class="status-divider" aria-hidden="true" />
      <h2>Compare branches</h2>
      <span class="meta"
        >{{ model.originalTraceId }} ↔ {{ model.forkTraceId }} · shared prefix
        {{ model.sharedOperationCount }} Operations</span
      >
      <span class="spacer" />
      <button class="link-action" type="button" @click="$emit('close')">
        esc to return to investigation
      </button>
    </div>

    <div class="comparison-body">
      <div class="shared-band">
        <div class="shared-band-head">
          <span class="eyebrow"
            >Shared history — identical in both, shown once</span
          >
          <span class="meta"
            >{{ model.sharedOperationCount }} Operations ·
            {{ sharedExpanded ? "expanded" : "collapsed" }}</span
          >
          <span class="spacer" />
          <button
            class="link-action"
            type="button"
            @click="sharedExpanded = !sharedExpanded"
          >
            {{ sharedExpanded ? "Collapse" : "Expand" }}
          </button>
        </div>
        <p
          v-for="row in model.sharedSummaryRows"
          :key="row.label"
          class="shared-row"
        >
          {{ row.label }} · {{ row.note }}
        </p>
      </div>

      <div class="comparison-checkpoint-row">
        <span class="badge badge--accent-fill"
          >FORK CHECKPOINT {{ model.checkpoint.id }}</span
        >
        <span
          >{{ model.checkpoint.operationId }} ·
          {{ model.checkpoint.label }}</span
        >
        <span class="spacer" />
        <span class="mono"
          >Intervention: {{ model.checkpoint.interventionSummary }}</span
        >
      </div>

      <div class="comparison-columns">
        <div class="comparison-column" :style="{ order: swapped ? 2 : 1 }">
          <div class="comparison-column-head">
            <span class="eyebrow">Original</span>
            <span class="trace-id mono">{{ model.originalTraceId }}</span>
            <span class="spacer" />
            <span class="badge badge--neutral"
              >OUTCOME: {{ model.original.outcome }}</span
            >
          </div>
          <div
            v-for="row in model.original.rows"
            :key="row.id"
            class="comparison-row"
          >
            <span>{{ row.label }}</span>
            <span
              v-for="badge in row.badges"
              :key="badge"
              class="badge badge--danger-outline"
              >{{ badge }}</span
            >
            <span v-if="row.duration" class="duration mono">{{
              row.duration
            }}</span>
          </div>
          <p class="comparison-footnote">{{ model.original.footnote }}</p>
        </div>

        <div class="comparison-column" :style="{ order: swapped ? 1 : 2 }">
          <div class="comparison-column-head">
            <span class="eyebrow">Fork</span>
            <span class="trace-id mono">{{ model.forkTraceId }}</span>
            <span class="badge badge--fill-neutral">{{
              model.fork.badge
            }}</span>
            <span class="spacer" />
            <span class="badge badge--accent-outline"
              >OUTCOME: {{ model.fork.outcome }}</span
            >
          </div>
          <div
            v-for="row in model.fork.rows"
            :key="row.id"
            class="comparison-row"
            :class="{
              'tint-success': row.tint === 'success',
              receiving: row.receiving,
            }"
          >
            <span>{{ row.receiving ? "receiving…" : row.label }}</span>
            <span v-if="row.duration" class="duration mono">{{
              row.duration
            }}</span>
          </div>
          <p class="comparison-footnote">{{ model.fork.footnote }}</p>
        </div>
      </div>
    </div>

    <div class="comparison-summary-bar">
      <span>{{ model.summary }}</span>
      <span class="spacer" />
      <span class="hint">⌘D toggles · ⇧⇥ swaps sides</span>
    </div>
  </div>
</template>
