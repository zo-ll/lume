<script setup lang="ts">
import TraceWorkspace from "./components/TraceWorkspace.vue";
import { investigationFixture } from "./fixtures/investigation";
</script>

<template>
  <TraceWorkspace :model="investigationFixture" />
</template>
