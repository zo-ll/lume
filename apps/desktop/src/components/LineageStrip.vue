<script setup lang="ts">
// Trace Links as hops; emits navigation intents only — it never nests one
// trace's Operations inside another's outline (design section p07).
import { ref } from "vue";
import type { TraceLinkVm } from "../view-models";

const props = defineProps<{
  links: readonly TraceLinkVm[];
  /** Narrow-window responsive fallback only — never user-selected (design
   * section p09: TraceWorkspace owns density as internal responsive
   * state, not a product setting). */
  compact?: boolean;
}>();

const emit = defineEmits<{
  navigate: [traceId: string];
}>();

const hoveredId = ref<string | null>(null);
const hovered = () =>
  props.links.find((l) => l.traceId === hoveredId.value)?.hoverDetail;

const relationText = (relation: TraceLinkVm["relation"]) =>
  props.compact ? `─${relation}─▸` : `──${relation}──▸`;
</script>

<template>
  <nav class="lineage" aria-label="Linked traces">
    <span class="section-label">Lineage</span>
    <div class="lineage-chain">
      <template v-for="(link, index) in links" :key="link.traceId">
        <span v-if="index > 0" class="relation" aria-hidden="true">{{
          relationText(link.relation)
        }}</span>
        <button
          class="lineage-chip"
          :class="{ current: link.current, tombstone: link.evidenceDeleted }"
          :aria-current="link.current ? 'page' : undefined"
          type="button"
          @click="!link.current && emit('navigate', link.traceId)"
          @mouseenter="hoveredId = link.traceId"
          @mouseleave="hoveredId = null"
          @focus="hoveredId = link.traceId"
          @blur="hoveredId = null"
        >
          {{ link.traceId }}
          <span v-if="link.evidenceDeleted" class="badge badge--dark-outline"
            >EVIDENCE DELETED</span
          >
        </button>
      </template>
    </div>

    <div v-if="hovered()" class="lineage-hover-card" role="tooltip">
      <h3>{{ hovered()!.title }}</h3>
      <p>{{ hovered()!.body }}</p>
      <div class="badge-row">
        <span
          v-for="badge in hovered()!.badges"
          :key="badge"
          class="badge"
          :class="{ 'badge--danger-outline': badge === 'UNTRUSTED RUNTIME' }"
          >{{ badge }}</span
        >
      </div>
      <p v-if="hovered()!.footnote" class="footnote">
        {{ hovered()!.footnote }}
      </p>
    </div>
  </nav>
</template>
