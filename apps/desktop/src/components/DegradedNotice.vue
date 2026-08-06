<script setup lang="ts">
// A trace can be degraded while still fully shown — this is a banner, not a
// replacement screen (design section p08, "Degraded · evidence loss").
import type { DegradedGapVm } from "../view-models";

defineProps<{ gap: DegradedGapVm }>();
</script>

<template>
  <div class="degraded-notice" role="status">
    <span class="gap-mark" aria-hidden="true" />
    <span>
      This trace has a gap. The runtime acknowledged dropping
      {{ gap.operationCount }} Operations between {{ gap.fromOperationId }} and
      {{ gap.toOperationId }} under backpressure. Marked
      <strong>incomplete</strong>, not repaired; causal claims across the gap
      are withheld rather than inferred.
    </span>
    <span class="gap-meta"
      >gap · {{ gap.operationCount }} Operations · {{ gap.seconds.toFixed(1) }}s
      unaccounted</span
    >
  </div>
</template>
