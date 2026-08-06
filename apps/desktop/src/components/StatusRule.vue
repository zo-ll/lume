<script setup lang="ts">
import type { TraceHeaderVm } from "../view-models";

defineProps<{ model: TraceHeaderVm }>();
</script>

<template>
  <header class="status-rule">
    <div class="identity">
      <span class="wordmark">Lume</span>
      <span class="agent-name">{{ model.agentName }}</span>
      <code>{{ model.traceId }}</code>
    </div>
    <dl class="status-items" aria-label="Trace and runtime status">
      <div>
        <dt>Completeness</dt>
        <dd>{{ model.completeness }}</dd>
      </div>
      <div>
        <dt>Trust</dt>
        <dd>{{ model.trust }}</dd>
      </div>
      <div>
        <dt>Session</dt>
        <dd><span class="live-dot" />{{ model.session }}</dd>
      </div>
      <div>
        <dt>Runtime</dt>
        <dd>{{ model.runtime }}</dd>
      </div>
      <div>
        <dt>Store</dt>
        <dd>{{ model.store }}</dd>
      </div>
    </dl>
  </header>
</template>
