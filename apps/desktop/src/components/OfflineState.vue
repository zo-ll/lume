<script setup lang="ts">
// "Offline · no runtime connected" (design section p08). Local History
// stays fully investigable; only fork/live capabilities are gated.
import { offlineStateCopy } from "../fixtures/appStates";
</script>

<template>
  <div class="state-scene">
    <div class="state-card">
      <h2>{{ offlineStateCopy.title }}</h2>
      <p>{{ offlineStateCopy.body }}</p>
      <ul class="capability-list">
        <li
          v-for="cap in offlineStateCopy.capabilities"
          :key="cap.label"
          class="capability-item"
          :class="{ unavailable: !cap.available }"
        >
          <span class="capability-mark" aria-hidden="true" />
          {{ cap.label }}
        </li>
      </ul>
    </div>
  </div>
</template>
