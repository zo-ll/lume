<script setup lang="ts">
// "Error · service unreachable" (design section p08). The window is a
// client of a separately supervised service; the window failing to reach it
// says nothing about whether trace capture is still running.
import { errorStateCopy } from "../fixtures/appStates";
</script>

<template>
  <div class="state-scene">
    <div class="state-card">
      <div class="state-eyebrow-danger">Error</div>
      <h2>{{ errorStateCopy.title }}</h2>
      <p>{{ errorStateCopy.body }}</p>
      <div class="state-tech-line">{{ errorStateCopy.technicalLine }}</div>
      <div class="state-actions">
        <button class="btn primary" type="button">Retry</button>
        <button class="btn" type="button">Show service status</button>
      </div>
    </div>
  </div>
</template>
