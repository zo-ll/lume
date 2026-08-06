<script setup lang="ts">
import type { OutlineRowVm } from "../view-models";

defineProps<{
  rows: readonly OutlineRowVm[];
  selectedId: string;
}>();

const emit = defineEmits<{ select: [id: string] }>();

const stateLabel: Record<OutlineRowVm["state"], string> = {
  complete: "Complete evidence",
  failed: "Failed operation",
  unverified: "Unverified evidence",
  receiving: "Operation receiving",
};
</script>

<template>
  <section class="outline-panel" aria-labelledby="outline-heading">
    <div class="panel-heading">
      <div>
        <span class="section-label">Trace</span>
        <h1 id="outline-heading">Causal outline</h1>
      </div>
      <span class="operation-count">46 Operations</span>
    </div>
    <div class="outline" role="tree" aria-label="Trace operations">
      <button
        v-for="row in rows"
        :key="row.id"
        class="outline-row"
        :class="{ selected: row.id === selectedId, checkpoint: row.checkpoint }"
        :style="{ '--depth': row.depth }"
        role="treeitem"
        :aria-level="row.depth + 1"
        :aria-expanded="row.hasChildren ? row.expanded : undefined"
        :aria-selected="row.id === selectedId"
        type="button"
        @click="emit('select', row.id)"
      >
        <span
          class="state-gutter"
          :data-state="row.state"
          :aria-label="stateLabel[row.state]"
        />
        <span v-if="row.hasChildren" class="disclosure" aria-hidden="true">{{
          row.expanded ? "▾" : "▸"
        }}</span>
        <span v-else class="disclosure" />
        <span class="row-copy">
          <span
            ><strong>{{ row.kind }}</strong>
            <span class="row-label">{{ row.label }}</span></span
          >
          <small v-if="row.detail">{{ row.detail }}</small>
        </span>
        <span v-if="row.checkpoint" class="checkpoint-mark">Checkpoint</span>
        <time>{{ row.duration ?? "live" }}</time>
      </button>
    </div>
  </section>
</template>
