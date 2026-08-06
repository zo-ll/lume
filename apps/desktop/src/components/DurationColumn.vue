<script setup lang="ts">
// Renders bars against a core-supplied shared time scale; never computes a
// duration itself. In compact density the bar track collapses to a
// right-aligned number (design section p02).
import type { OutlineDurationVm } from "../view-models";

defineProps<{ duration?: OutlineDurationVm }>();

const formatSeconds = (seconds: number) => `${seconds.toFixed(2)}s`;
</script>

<template>
  <span class="duration-cell">
    <span class="duration-track">
      <span
        v-if="duration"
        class="duration-fill"
        :class="duration.style"
        :style="{
          left: duration.startFraction * 100 + '%',
          width: duration.extentFraction * 100 + '%',
        }"
      />
    </span>
    <span class="duration-text">{{
      duration && duration.style !== "aggregate" && duration.style !== "open"
        ? formatSeconds(duration.seconds)
        : ""
    }}</span>
  </span>
</template>
