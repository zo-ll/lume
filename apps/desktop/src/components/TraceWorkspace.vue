<script setup lang="ts">
import { computed, ref } from "vue";
import type { TraceWorkspaceVm } from "../view-models";
import CausalOutline from "./CausalOutline.vue";
import LineageStrip from "./LineageStrip.vue";
import OperationInspector from "./OperationInspector.vue";
import StatusRule from "./StatusRule.vue";

const props = defineProps<{ model: TraceWorkspaceVm }>();
const selectedId = ref(props.model.selectedId);
const selectedDetail = computed(
  () => props.model.detailsById[selectedId.value],
);
</script>

<template>
  <main class="trace-workspace">
    <StatusRule :model="model.header" />
    <LineageStrip :links="model.links" />
    <div class="workspace-grid">
      <CausalOutline
        :rows="model.rows"
        :selected-id="selectedId"
        @select="selectedId = $event"
      />
      <OperationInspector :model="selectedDetail" />
    </div>
    <footer class="capture-rule">
      <span
        ><span class="live-dot" /> Receiving Operations from
        {{ model.header.runtime }}</span
      >
      <span>Local History durable · Service independent of this window</span>
    </footer>
  </main>
</template>
