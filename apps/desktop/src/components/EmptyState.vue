<script setup lang="ts">
// "Empty · first run" (design section p08). Invents no reassurance.
import { emptyStateCopy } from "../fixtures/appStates";
</script>

<template>
  <div class="state-scene">
    <div class="state-card">
      <div class="state-icon" aria-hidden="true" />
      <h2>{{ emptyStateCopy.title }}</h2>
      <p>{{ emptyStateCopy.body }}</p>
      <div class="state-actions">
        <button class="btn primary" type="button">
          Connection instructions
        </button>
        <button class="btn" type="button">Import a Diagnostic Export</button>
      </div>
    </div>
  </div>
</template>
