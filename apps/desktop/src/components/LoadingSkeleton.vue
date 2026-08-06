<script setup lang="ts">
// "Loading · outline skeleton" (design section p08) — explicitly no
// spinner; progress is shown as a rising row count.
import { loadingStateCopy } from "../fixtures/appStates";

const widths = [62, 44, 71, 38];
</script>

<template>
  <div class="state-scene">
    <div class="state-card" role="status" aria-live="polite">
      <div class="skeleton-rows">
        <div
          v-for="(w, i) in widths"
          :key="i"
          class="skeleton-row"
          :style="{ '--depth': i % 2 }"
        >
          <span class="skeleton-gutter" />
          <span class="skeleton-bar" :style="{ width: w + '%' }" />
        </div>
      </div>
      <p class="skeleton-caption">{{ loadingStateCopy.caption }}</p>
      <p class="skeleton-caption">{{ loadingStateCopy.note }}</p>
    </div>
  </div>
</template>
