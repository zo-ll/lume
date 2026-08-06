<script setup lang="ts">
import type { OperationDetailVm } from "../view-models";

defineProps<{ model: OperationDetailVm }>();
</script>

<template>
  <aside class="inspector" aria-labelledby="inspector-heading">
    <div class="inspector-heading">
      <span class="section-label">{{ model.eyebrow }}</span>
      <h2 id="inspector-heading">{{ model.title }}</h2>
      <code>{{ model.operationId }}</code>
    </div>

    <dl class="facts">
      <div>
        <dt>Causal cut</dt>
        <dd>{{ model.causalPosition }}</dd>
      </div>
      <div>
        <dt>Lifecycle</dt>
        <dd>{{ model.lifecycle }}</dd>
      </div>
      <div>
        <dt>Runtime</dt>
        <dd>{{ model.runtime }}</dd>
      </div>
      <div>
        <dt>Checkpoint</dt>
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

    <div class="inspector-actions">
      <button class="primary-action" type="button">
        {{ model.permittedActions[0] }}
      </button>
      <button class="secondary-action" type="button">
        {{ model.permittedActions[1] }}
      </button>
    </div>
    <p class="shortcut-note">
      <kbd>N</kbd> next checkpoint · <kbd>⌘K</kbd> commands
    </p>
  </aside>
</template>
