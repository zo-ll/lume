<script setup lang="ts">
// Protected and write-only presentation. This component has no code path
// that can render a value the core did not send — a redaction placeholder is
// literal text, never a value it derived or cached.
import type { SensitiveValueVm } from "../view-models";

defineProps<{ model: SensitiveValueVm }>();
</script>

<template>
  <span v-if="model.kind === 'plain'" class="sensitive-value plain">{{
    model.value
  }}</span>
  <span
    v-else-if="model.kind === 'protected'"
    class="sensitive-value protected"
  >
    <span class="badge badge--dark-outline">PROTECTED — STORE LOCKED</span>
    <span class="redaction-block" aria-hidden="true">▒▒▒▒▒▒▒▒▒▒▒▒▒▒</span>
    <span class="sensitive-caption">{{ model.caption }}</span>
  </span>
  <span v-else class="sensitive-value write-only">
    <span class="badge badge--dark-outline">WRITE-ONLY — NEVER RETAINED</span>
    <span class="sensitive-caption">{{ model.caption }}</span>
  </span>
</template>
