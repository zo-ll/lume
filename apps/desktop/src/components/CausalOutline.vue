<script setup lang="ts">
// Virtualized tree over OutlineRowVm[] (design section p09: "Expansion and
// selection are local; ordering and parentage are not"). Implements the
// keyboard model from section p03: arrows move/collapse/expand within a
// single tab stop, n/N jump between Fork Checkpoints.
import { computed, reactive, ref } from "vue";
import type { OutlineRowVm, OutlineVm } from "../view-models";
import StateGutter from "./StateGutter.vue";
import DurationColumn from "./DurationColumn.vue";

const props = defineProps<{
  outline: OutlineVm;
  selectedId: string;
  /** Density is read by CSS via [data-density] on an ancestor; kept as a
   * prop too so compact mode can shorten the checkpoint badge word. */
  compact?: boolean;
}>();

const emit = defineEmits<{
  select: [id: string];
  "follow-link": [traceId: string];
}>();

const expandedOverride = reactive<Record<string, boolean>>({});
const isExpanded = (row: OutlineRowVm) =>
  expandedOverride[row.id] ?? row.expanded ?? true;

const visibleRows = computed(() => {
  const result: OutlineRowVm[] = [];
  let hideUntilDepth: number | null = null;
  for (const row of props.outline.rows) {
    if (hideUntilDepth !== null) {
      if (row.depth > hideUntilDepth) continue;
      hideUntilDepth = null;
    }
    result.push(row);
    if (row.hasChildren && !isExpanded(row)) {
      hideUntilDepth = row.depth;
    }
  }
  return result;
});

const rowEls = new Map<string, HTMLElement>();
const setRowEl = (id: string) => (el: unknown) => {
  if (el instanceof HTMLElement) rowEls.set(id, el);
  else rowEls.delete(id);
};

function focusRow(id: string) {
  emit("select", id);
  requestAnimationFrame(() => rowEls.get(id)?.focus());
}

function moveSelection(delta: 1 | -1) {
  const rows = visibleRows.value;
  const i = rows.findIndex((r) => r.id === props.selectedId);
  const next = rows[Math.min(Math.max(i + delta, 0), rows.length - 1)];
  if (next) focusRow(next.id);
}

function jumpCheckpoint(delta: 1 | -1) {
  const rows = visibleRows.value.filter((r) => r.checkpointAvailability);
  if (rows.length === 0) return;
  const i = rows.findIndex((r) => r.id === props.selectedId);
  const idx = i === -1 ? 0 : (i + delta + rows.length) % rows.length;
  focusRow(rows[idx].id);
}

function toggleExpanded(row: OutlineRowVm, force?: boolean) {
  if (!row.hasChildren) return;
  expandedOverride[row.id] = force ?? !isExpanded(row);
}

function onKeydown(event: KeyboardEvent) {
  const current = visibleRows.value.find((r) => r.id === props.selectedId);
  switch (event.key) {
    case "ArrowDown":
      event.preventDefault();
      moveSelection(1);
      break;
    case "ArrowUp":
      event.preventDefault();
      moveSelection(-1);
      break;
    case "ArrowRight":
      if (current?.hasChildren) {
        event.preventDefault();
        toggleExpanded(current, true);
      }
      break;
    case "ArrowLeft":
      if (current?.hasChildren) {
        event.preventDefault();
        toggleExpanded(current, false);
      }
      break;
    case "n":
      event.preventDefault();
      jumpCheckpoint(1);
      break;
    case "N":
      event.preventDefault();
      jumpCheckpoint(-1);
      break;
  }
}

const checkpointBadgeWord = computed(() =>
  props.compact ? "CHECKPOINT" : "FORK CHECKPOINT",
);
</script>

<template>
  <section class="outline-panel" aria-labelledby="outline-heading">
    <h1 id="outline-heading" class="visually-hidden">Causal outline</h1>
    <div class="outline-header">
      <span class="outline-header-cell col-state">State</span>
      <span class="outline-header-cell col-operation">Operation</span>
      <span class="outline-header-cell col-producer">Producer</span>
      <div class="duration-header" aria-hidden="true">
        <template v-if="!compact">
          <span class="ticks"
            ><span class="tick" /><span class="tick" /><span
              class="tick" /><span class="tick"
          /></span>
          <span class="scale-start outline-header-cell">0s</span>
          <span class="scale-end outline-header-cell"
            >{{ outline.scaleSeconds }}s</span
          >
        </template>
        <span v-else class="outline-header-cell">DUR</span>
      </div>
    </div>

    <!-- No tabindex here: the tree is a single tab stop realized entirely
         by the roving tabindex on its rows below (design section p08's
         accessibility rule), not by the container itself. Keydown from a
         focused row still bubbles up to this handler. -->
    <div
      class="outline"
      role="tree"
      aria-label="Trace operations"
      @keydown="onKeydown"
    >
      <div
        v-for="row in visibleRows"
        :key="row.id"
        :ref="setRowEl(row.id)"
        class="outline-row"
        :class="{
          step: row.kind === 'Agent Step',
          selected: row.id === selectedId,
          receiving: row.duration?.style === 'open',
        }"
        :style="{ '--depth': row.depth }"
        role="treeitem"
        :aria-level="row.depth + 1"
        :aria-expanded="row.hasChildren ? isExpanded(row) : undefined"
        :aria-selected="row.id === selectedId"
        :tabindex="row.id === selectedId ? 0 : -1"
        @click="emit('select', row.id)"
        @dblclick="row.linkedTraceId && emit('follow-link', row.linkedTraceId)"
      >
        <StateGutter
          :completeness="row.completeness"
          :checkpoint-availability="row.checkpointAvailability"
        />
        <span class="row-indent" aria-hidden="true" />
        <button
          v-if="row.hasChildren"
          class="disclosure"
          type="button"
          tabindex="-1"
          aria-hidden="true"
          @click.stop="toggleExpanded(row)"
        >
          {{ isExpanded(row) ? "▾" : "▸" }}
        </button>
        <span v-else class="disclosure" aria-hidden="true" />

        <span class="row-copy">
          <template v-if="row.duration?.style === 'open'">
            <span class="row-receiving-label">receiving…</span>
            <span class="row-annotation">{{ row.detail }}</span>
          </template>
          <template v-else>
            <span class="row-kind"
              >{{ row.kind
              }}<template v-if="row.kind !== 'Agent Step'"
                >&nbsp;</template
              ></span
            >
            <span v-if="row.kind !== 'Agent Step'" class="row-detail">{{
              row.label
            }}</span>
            <span v-else class="row-detail">{{ row.label }}</span>
            <span v-if="row.causalAnnotation" class="row-annotation">{{
              row.causalAnnotation
            }}</span>
          </template>
          <template v-for="badge in row.badges" :key="badge">
            <span
              class="badge"
              :class="{
                'badge--danger-outline': badge === 'FAILED',
                'badge--accent-fill': badge === 'FORK CHECKPOINT',
                'badge--dark-outline':
                  badge === 'UNVERIFIED' || badge === 'WRITE-ONLY FIELD',
              }"
              >{{
                badge === "FORK CHECKPOINT" ? checkpointBadgeWord : badge
              }}</span
            >
          </template>
        </span>

        <span class="row-producer">{{
          row.producer?.label ?? (row.kind === "Agent Step" ? "—" : "")
        }}</span>

        <DurationColumn :duration="row.duration" />
        <span class="row-tail" />
      </div>
    </div>

    <div class="outline-footer">
      <span class="count"
        >{{ outline.operationCount }} Operations ·
        {{ outline.agentStepCount }} Agent Steps ·
        {{ outline.checkpointCount }} Fork Checkpoints</span
      >
      <span class="spacer" />
      <span class="count">hold ⌘ for commands</span>
    </div>

    <!-- Live Operations append via a polite live region announcing counts,
         never a per-row announcement (design section p08). -->
    <div class="visually-hidden" aria-live="polite" role="status">
      {{ outline.operationCount }} Operations in this trace.
    </div>
  </section>
</template>
