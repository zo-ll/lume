<script setup lang="ts">
import { computed, ref, watch } from "vue";
import type { SceneId } from "./fixtures/scenarios";
import { investigationFixture } from "./fixtures/investigation";
import { traceRegistry } from "./fixtures/linkedTraces";
import {
  forkCheckpoint,
  interventionFields,
  interventionFieldsBlocked,
  interventionFieldsInvalid,
  interventionValidationBlocked,
  interventionValidationInvalid,
  interventionValidationOk,
} from "./fixtures/forkRequest";
import { lifecycleFixtures } from "./fixtures/lifecycle";
import { comparisonFixture } from "./fixtures/comparison";
import { degradedGapFixture } from "./fixtures/appStates";
import type { TraceWorkspaceVm } from "./view-models";

import TraceWorkspace from "./components/TraceWorkspace.vue";
import BranchComparison from "./components/BranchComparison.vue";
import ForkRequestLifecycle from "./components/ForkRequestLifecycle.vue";
import EmptyState from "./components/EmptyState.vue";
import LoadingSkeleton from "./components/LoadingSkeleton.vue";
import OfflineState from "./components/OfflineState.vue";
import ServiceErrorState from "./components/ServiceErrorState.vue";
import DevScenarioSwitcher from "./components/DevScenarioSwitcher.vue";

// The dev scenario switcher is a review aid, not product chrome. It must
// be absent from the ordinary `tauri dev` route, not merely collapsed —
// so it needs two things to be true, not one:
//   1. import.meta.env.DEV — Vite replaces this with a literal at build
//      time, so in production this is always false and the switcher never
//      renders (and is dead-code-eligible), regardless of the query param.
//   2. An explicit, documented opt-in query param (?review=1) — without
//      it, the default development URL renders zero review controls, same
//      as production.
const isDev = import.meta.env.DEV;
const reviewOptIn =
  typeof window !== "undefined" &&
  new URLSearchParams(window.location.search).get("review") === "1";
const showDevSwitcher = isDev && reviewOptIn;

const scene = ref<SceneId>("investigation");
const theme = ref<"system" | "light" | "dark">("system");

watch(
  theme,
  (t) => {
    const root = document.documentElement;
    if (t === "system") root.removeAttribute("data-theme");
    else root.dataset.theme = t;
  },
  { immediate: true },
);

// Fixture-backed linked navigation: the investigation scene's lineage links
// (tr_1a09, tr_9f2b) are real, independently-modeled traces, not dead
// buttons. Navigating swaps the whole model — never nests one trace's
// Operations inside another's outline.
const allTraces: Record<string, TraceWorkspaceVm> = {
  tr_4c81e0: investigationFixture,
  ...traceRegistry,
};
const currentTraceId = ref("tr_4c81e0");
const activeInvestigationModel = computed(
  () => allTraces[currentTraceId.value],
);
function onNavigate(traceId: string) {
  if (allTraces[traceId]) currentTraceId.value = traceId;
}

const degradedModel = computed(() => ({
  ...activeInvestigationModel.value,
  // The evidence gap belongs to tr_4c81e0 specifically; navigating to a
  // linked trace shows that trace's own (ungapped) state honestly rather
  // than carrying the banner along with it.
  degraded:
    currentTraceId.value === "tr_4c81e0" ? degradedGapFixture : undefined,
}));

function goToComparison() {
  scene.value = "comparison";
}
function goToInvestigation() {
  scene.value = "investigation";
}
</script>

<template>
  <div class="app-shell">
    <TraceWorkspace
      v-if="scene === 'investigation'"
      :model="activeInvestigationModel"
      :fork-checkpoint="forkCheckpoint"
      :fork-fields="interventionFields"
      :fork-validation="interventionValidationOk"
      @open-comparison="goToComparison"
      @navigate="onNavigate"
    />

    <TraceWorkspace
      v-else-if="scene === 'state-degraded'"
      :model="degradedModel"
      :fork-checkpoint="forkCheckpoint"
      :fork-fields="interventionFields"
      :fork-validation="interventionValidationOk"
      @open-comparison="goToComparison"
      @navigate="onNavigate"
    />

    <TraceWorkspace
      v-else-if="scene === 'fork-intervention'"
      :model="investigationFixture"
      :fork-checkpoint="forkCheckpoint"
      :fork-fields="interventionFields"
      :fork-validation="interventionValidationOk"
      start-in-fork-flow
      :fork-initial-stage="2"
    />
    <TraceWorkspace
      v-else-if="scene === 'fork-blocked'"
      :model="investigationFixture"
      :fork-checkpoint="forkCheckpoint"
      :fork-fields="interventionFieldsBlocked"
      :fork-validation="interventionValidationBlocked"
      start-in-fork-flow
      :fork-initial-stage="2"
    />
    <TraceWorkspace
      v-else-if="scene === 'fork-invalid'"
      :model="investigationFixture"
      :fork-checkpoint="forkCheckpoint"
      :fork-fields="interventionFieldsInvalid"
      :fork-validation="interventionValidationInvalid"
      start-in-fork-flow
      :fork-initial-stage="2"
    />
    <TraceWorkspace
      v-else-if="scene === 'fork-mode'"
      :model="investigationFixture"
      :fork-checkpoint="forkCheckpoint"
      :fork-fields="interventionFields"
      :fork-validation="interventionValidationOk"
      start-in-fork-flow
      :fork-initial-stage="3"
    />
    <TraceWorkspace
      v-else-if="scene === 'fork-live-confirm'"
      :model="investigationFixture"
      :fork-checkpoint="forkCheckpoint"
      :fork-fields="interventionFields"
      :fork-validation="interventionValidationOk"
      start-in-fork-flow
      :fork-initial-stage="4"
      fork-initial-mode="live"
    />

    <div
      v-else-if="scene === 'lifecycle-gallery'"
      class="trace-workspace lifecycle-scene"
    >
      <div class="lifecycle-grid">
        <ForkRequestLifecycle
          v-for="(fixture, i) in lifecycleFixtures"
          :key="i"
          :model="fixture"
        />
      </div>
    </div>

    <div v-else-if="scene === 'comparison'" class="trace-workspace">
      <BranchComparison :model="comparisonFixture" @close="goToInvestigation" />
    </div>

    <EmptyState v-else-if="scene === 'state-empty'" />
    <LoadingSkeleton v-else-if="scene === 'state-loading'" />
    <OfflineState v-else-if="scene === 'state-offline'" />
    <ServiceErrorState v-else-if="scene === 'state-error'" />

    <DevScenarioSwitcher
      v-if="showDevSwitcher"
      v-model:scene="scene"
      v-model:theme="theme"
    />
  </div>
</template>
