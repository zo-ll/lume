<script setup lang="ts">
// Owns pane split, responsive density, and (for review purposes) the
// fork-flow overlay's open/closed state. No domain state — every value
// rendered here comes from a prop (design section p09).
//
// Density is not a product setting: comfortable is the only normal
// desktop state, and compact is purely an automatic narrow-window
// fallback (design section p02's "below ~1100px" rule) — there is no
// manual override and no UI to select one.
import { computed, onBeforeUnmount, onMounted, ref, watch } from "vue";
import type {
  CheckpointVm,
  ExecutionMode,
  InterventionFieldVm,
  PaletteVm,
  TraceWorkspaceVm,
  ValidationVm,
} from "../view-models";
import { holdHintFixture, paletteFixture } from "../fixtures/commands";
import CausalOutline from "./CausalOutline.vue";
import LineageStrip from "./LineageStrip.vue";
import OperationInspector from "./OperationInspector.vue";
import StatusRule from "./StatusRule.vue";
import CommandPalette from "./CommandPalette.vue";
import DegradedNotice from "./DegradedNotice.vue";
import ForkRequestPanel from "./ForkRequestPanel.vue";

const props = defineProps<{
  model: TraceWorkspaceVm;
  forkCheckpoint?: CheckpointVm;
  forkFields?: readonly InterventionFieldVm[];
  forkValidation?: ValidationVm;
  /** Dev-switcher only: mount straight into the fork flow at a given stage. */
  startInForkFlow?: boolean;
  forkInitialStage?: 2 | 3 | 4;
  forkInitialMode?: ExecutionMode;
}>();

const emit = defineEmits<{
  navigate: [traceId: string];
  "open-comparison": [];
}>();

const selectedId = ref(props.model.selectedId);
watch(
  () => props.model,
  (m) => (selectedId.value = m.selectedId),
);
const selectedDetail = computed(
  () => props.model.detailsById[selectedId.value],
);

// --- density (responsive only, never user-selected) --------------------
const mql =
  typeof matchMedia === "function" ? matchMedia("(max-width: 1100px)") : null;
const compact = ref(mql?.matches ?? false);
const onMqlChange = (e: MediaQueryListEvent) => (compact.value = e.matches);
const density = computed(() => (compact.value ? "compact" : "comfortable"));

// --- command palette + hold-hint --------------------------------------
const paletteOpen = ref(false);
const holdHintVisible = ref(false);
let holdHintTimer: ReturnType<typeof setTimeout> | null = null;

function isTypingTarget(): boolean {
  const el = document.activeElement;
  return el instanceof HTMLInputElement || el instanceof HTMLTextAreaElement;
}

/** [ / ] follow a Trace Link back or forward along the lineage strip —
 * never nests one trace's Operations inside another's outline (design
 * section p03/p07). Suppressed while typing so it never hijacks a text
 * field, and while the fork flow or palette are open. */
function followLineage(direction: -1 | 1) {
  const links = props.model.links;
  const i = links.findIndex((l) => l.current);
  const target = links[i + direction];
  if (target) emit("navigate", target.traceId);
}

function onWindowKeydown(event: KeyboardEvent) {
  if ((event.metaKey || event.ctrlKey) && event.key.toLowerCase() === "k") {
    event.preventDefault();
    paletteOpen.value = !paletteOpen.value;
    return;
  }
  if (event.key === "Meta") {
    holdHintVisible.value = true;
    return;
  }
  if (event.key === "Escape" && paletteOpen.value) {
    paletteOpen.value = false;
    return;
  }
  if (forkFlowActive.value || paletteOpen.value || isTypingTarget()) return;
  if (event.key === "[") {
    event.preventDefault();
    followLineage(-1);
  } else if (event.key === "]") {
    event.preventDefault();
    followLineage(1);
  }
}
function onWindowKeyup(event: KeyboardEvent) {
  if (event.key === "Meta") {
    if (holdHintTimer) clearTimeout(holdHintTimer);
    holdHintTimer = setTimeout(() => (holdHintVisible.value = false), 250);
  }
}
onMounted(() => {
  window.addEventListener("keydown", onWindowKeydown);
  window.addEventListener("keyup", onWindowKeyup);
  mql?.addEventListener?.("change", onMqlChange);
});
onBeforeUnmount(() => {
  window.removeEventListener("keydown", onWindowKeydown);
  window.removeEventListener("keyup", onWindowKeyup);
  mql?.removeEventListener?.("change", onMqlChange);
});

function runCommand(id: string) {
  paletteOpen.value = false;
  if (id === "create-fork-request") forkFlowActive.value = true;
  else if (id === "compare-source-branch") emit("open-comparison");
  else if (id === "go-to-delegated-trace") emit("navigate", "tr_9f2b");
}

/** Every other permitted action is a labeled intent, not an assumption.
 * Only known labels do anything; an unrecognized label is a no-op rather
 * than silently opening the fork flow. */
function onInspectorAction(label: string) {
  if (label === "Open command palette") paletteOpen.value = true;
  else if (label === "Compare linked traces") emit("open-comparison");
}

// --- fork request flow (real interaction path) -------------------------
const forkFlowActive = ref(props.startInForkFlow ?? false);
function openForkFlow() {
  forkFlowActive.value = true;
}
function closeForkFlow() {
  forkFlowActive.value = false;
}
function submitForkFlow(_mode: ExecutionMode) {
  // No backend yet: a real submission would hand off to
  // ForkRequestLifecycle. See the dev scenario switcher's lifecycle
  // gallery for every phase this could enter next.
  forkFlowActive.value = false;
}
</script>

<template>
  <main class="trace-workspace" :data-density="density">
    <StatusRule :model="model.header" :compact="compact" />

    <template v-if="!forkFlowActive">
      <LineageStrip
        :links="model.links"
        :compact="compact"
        @navigate="emit('navigate', $event)"
      />
      <DegradedNotice v-if="model.degraded" :gap="model.degraded" />
      <div class="workspace-grid">
        <CausalOutline
          :outline="model.outline"
          :selected-id="selectedId"
          :compact="compact"
          @select="selectedId = $event"
          @follow-link="emit('navigate', $event)"
        />
        <OperationInspector
          v-if="selectedDetail"
          :model="selectedDetail"
          @create-fork-request="openForkFlow"
          @action="onInspectorAction"
        />
      </div>
    </template>

    <ForkRequestPanel
      v-else-if="forkCheckpoint && forkFields && forkValidation"
      request-id="req_7d1"
      :checkpoint="forkCheckpoint"
      :fields="forkFields"
      :validation="forkValidation"
      :outline-rows="model.outline.rows"
      :initial-stage="forkInitialStage"
      :initial-mode="forkInitialMode"
      @cancel="closeForkFlow"
      @submit="submitForkFlow"
    />

    <footer class="capture-rule">
      <span class="capture-live"
        ><span v-if="model.header.sessionLive" class="live-dot" /> Receiving
        Operations from {{ model.header.runtimeName }}</span
      >
      <span>Local History durable · Service independent of this window</span>
    </footer>

    <CommandPalette
      v-if="paletteOpen"
      :model="paletteFixture"
      @close="paletteOpen = false"
      @run="runCommand"
    />
    <div v-if="holdHintVisible && !paletteOpen" class="hold-hint-overlay">
      <div class="hold-hint-bar">
        <span class="eyebrow">{{ holdHintFixture.eyebrow }}</span>
        <span
          v-for="chip in holdHintFixture.chips"
          :key="chip.key"
          class="hold-hint-chip"
          :class="{ disabled: chip.disabled }"
        >
          <kbd>{{ chip.key }}</kbd> {{ chip.label }}
        </span>
      </div>
    </div>
  </main>
</template>
