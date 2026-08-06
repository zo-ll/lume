<script setup lang="ts">
// Trace identity, completeness, session and store state (design section
// p09). No domain decisions — every value here is given.
import type { TraceHeaderVm } from "../view-models";

defineProps<{ model: TraceHeaderVm; compact?: boolean }>();
</script>

<template>
  <header class="status-rule">
    <span class="wordmark">LUME</span>
    <span class="status-divider" aria-hidden="true" />

    <div class="identity">
      <span
        class="gutter-completeness completeness-dash"
        :class="model.completeness"
        role="img"
        :aria-label="model.completenessLabel"
      />
      <span class="agent-name">{{ model.agentName }}</span>
      <code class="trace-id">{{ model.traceId }}</code>
      <span v-if="!compact" class="badge badge--neutral">{{
        model.completenessLabel
      }}</span>
    </div>

    <span class="spacer" />

    <div class="status-items">
      <template v-if="!compact">
        <span class="status-item">
          <span v-if="model.sessionLive" class="live-dot" aria-hidden="true" />
          {{ model.sessionLive ? "session live" : "session ended" }} ·
          {{ model.runtimeCount }} runtime{{
            model.runtimeCount === 1 ? "" : "s"
          }}
        </span>
        <span v-if="model.storeLocked" class="status-item">
          <span class="badge badge--neutral">STORE LOCKED</span>
        </span>
      </template>
      <span v-else class="status-item">
        <span v-if="model.sessionLive" class="live-dot" aria-hidden="true" />
        live
      </span>
      <span class="status-item kbd-hint">⌘K</span>
    </div>
  </header>
</template>
