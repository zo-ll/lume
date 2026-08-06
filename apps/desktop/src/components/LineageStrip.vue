<script setup lang="ts">
import type { TraceLinkVm } from "../view-models";

defineProps<{ links: readonly TraceLinkVm[] }>();
</script>

<template>
  <nav class="lineage" aria-label="Linked traces">
    <span class="section-label">Lineage</span>
    <template v-for="(link, index) in links" :key="link.traceId">
      <span v-if="index > 0" class="relation" aria-hidden="true">
        —{{ link.relation }}→
      </span>
      <button
        class="trace-link"
        :class="{ current: link.current }"
        :aria-current="link.current ? 'page' : undefined"
        type="button"
      >
        {{ link.traceId }}
      </button>
    </template>
  </nav>
</template>
