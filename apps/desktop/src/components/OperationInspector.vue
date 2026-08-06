<script setup lang="ts">
// Sections driven entirely by OperationDetailVm; renders redaction
// placeholders exactly as given (design section p09).
import type { OperationDetailVm } from "../view-models";
import SensitiveValue from "./SensitiveValue.vue";

defineProps<{ model: OperationDetailVm }>();

// Only the forkable checkpoint CTA may start a Fork Request. Every other
// permitted action is a generic, core-labeled intent — the inspector does
// not get to decide that "the first action" means fork creation.
const emit = defineEmits<{
  "create-fork-request": [];
  action: [label: string];
}>();
</script>

<template>
  <aside class="inspector" aria-labelledby="inspector-heading">
    <div class="inspector-header">
      <span class="eyebrow">Inspector</span>
      <span class="op-id">{{ model.operationId.split(" ")[0] }}</span>
    </div>
    <div class="inspector-body">
      <span class="section-label visually-hidden">{{ model.eyebrow }}</span>
      <h2 id="inspector-heading" class="inspector-title">{{ model.title }}</h2>
      <p class="inspector-subline">{{ model.operationId }}</p>

      <div v-if="model.badges.length" class="inspector-badges">
        <span
          v-for="badge in model.badges"
          :key="badge"
          class="badge"
          :class="{ 'badge--accent-fill': badge === 'FORK CHECKPOINT' }"
          >{{ badge }}</span
        >
      </div>

      <dl class="facts">
        <div>
          <dt>Lifecycle</dt>
          <dd>{{ model.lifecycle }}</dd>
        </div>
        <div>
          <dt>Causal Cut</dt>
          <dd>{{ model.causalPosition }}</dd>
        </div>
        <div>
          <dt>Owning runtime</dt>
          <dd>{{ model.runtime }}</dd>
        </div>
        <div v-if="model.checkpointAvailability">
          <dt>Availability</dt>
          <dd>{{ model.checkpointAvailability }}</dd>
        </div>
        <div>
          <dt>Completeness</dt>
          <dd>{{ model.completeness }}</dd>
        </div>
        <div>
          <dt>Trust</dt>
          <dd>{{ model.trust }}</dd>
        </div>
        <div>
          <dt>Protected Store</dt>
          <dd>{{ model.protectedStore }}</dd>
        </div>
      </dl>

      <template v-if="model.inputs?.length">
        <h3 class="inspector-section-label">Inputs</h3>
        <div
          v-for="input in model.inputs"
          :key="input.name"
          class="input-field"
        >
          <div class="input-field-head">
            <span class="input-name">{{ input.name }}</span>
            <span class="input-type">{{ input.type }}</span>
          </div>
          <SensitiveValue :model="input.value" />
        </div>
      </template>

      <div v-if="model.forkable" class="fork-cta">
        <h3>Fork from this checkpoint</h3>
        <p>
          Intervention Fields are editable here. Availability is revalidated
          before restoration begins.
        </p>
        <div class="fork-cta-actions">
          <button
            class="btn primary"
            type="button"
            @click="emit('create-fork-request')"
          >
            Create Fork Request
          </button>
          <span class="btn-shortcut">⌘⏎</span>
        </div>
      </div>

      <div v-else class="inspector-actions">
        <button
          v-for="(label, i) in model.permittedActions"
          :key="label"
          :class="i === 0 ? 'btn primary' : 'btn'"
          type="button"
          @click="emit('action', label)"
        >
          {{ label }}
        </button>
      </div>

      <p class="shortcut-note">
        <kbd>N</kbd> next checkpoint · <kbd>⌘K</kbd> commands
      </p>
    </div>
  </aside>
</template>
