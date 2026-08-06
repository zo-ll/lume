<script setup lang="ts">
// Lists CommandVm including disabled ones with their reasons — impossible
// commands are never hidden, even when filtered by the search query (design
// section p03).
import { computed, nextTick, ref, watch } from "vue";
import type { CommandVm, PaletteVm } from "../view-models";

const props = defineProps<{ model: PaletteVm }>();
const emit = defineEmits<{ close: []; run: [id: string] }>();

const query = ref("");
const inputEl = ref<HTMLInputElement | null>(null);

function matches(command: CommandVm, q: string): boolean {
  if (!q) return true;
  const needle = q.toLowerCase();
  return (
    command.label.toLowerCase().includes(needle) ||
    command.group?.toLowerCase().includes(needle) === true
  );
}

const filteredGroups = computed(() =>
  props.model.groups
    .map((group) => ({
      ...group,
      commands: group.commands.filter((c) => matches(c, query.value)),
    }))
    .filter((group) => group.commands.length > 0),
);

const flatEnabled = computed(() =>
  filteredGroups.value
    .flatMap((g) => g.commands)
    .filter((c) => !c.disabledReason),
);
const activeId = ref(flatEnabled.value[0]?.id ?? "");

watch(flatEnabled, (list) => {
  if (!list.some((c) => c.id === activeId.value)) {
    activeId.value = list[0]?.id ?? "";
  }
});
watch(
  () => props.model,
  () => {
    activeId.value = flatEnabled.value[0]?.id ?? "";
  },
);

function moveActive(delta: 1 | -1) {
  const list = flatEnabled.value;
  if (list.length === 0) return;
  const i = list.findIndex((c) => c.id === activeId.value);
  const next = list[(i + delta + list.length) % list.length];
  if (next) activeId.value = next.id;
}

function onKeydown(event: KeyboardEvent) {
  if (event.key === "Escape") {
    event.preventDefault();
    emit("close");
  } else if (event.key === "ArrowDown") {
    event.preventDefault();
    moveActive(1);
  } else if (event.key === "ArrowUp") {
    event.preventDefault();
    moveActive(-1);
  } else if (event.key === "Enter") {
    event.preventDefault();
    if (activeId.value) emit("run", activeId.value);
  }
}

nextTick(() => inputEl.value?.focus());
</script>

<template>
  <div class="palette-backdrop" @click.self="emit('close')">
    <div
      class="command-palette"
      role="dialog"
      aria-label="Command palette"
      @keydown="onKeydown"
    >
      <div class="palette-search">
        <span class="palette-prompt" aria-hidden="true">›</span>
        <input
          ref="inputEl"
          v-model="query"
          type="text"
          placeholder="Type a command…"
          aria-label="Search commands"
        />
      </div>
      <div class="palette-scope">{{ model.scopeLabel }}</div>
      <div class="palette-body">
        <p
          v-if="filteredGroups.length === 0"
          class="palette-reason"
          style="padding: 12px 16px"
        >
          No commands match "{{ query }}".
        </p>
        <template v-for="(group, gi) in filteredGroups" :key="gi">
          <div v-if="group.label" class="palette-group-label">
            {{ group.label }}
          </div>
          <button
            v-for="command in group.commands"
            :key="command.id"
            class="palette-row"
            :class="{
              selected: command.id === activeId && !command.disabledReason,
              disabled: !!command.disabledReason,
            }"
            type="button"
            :disabled="!!command.disabledReason"
            :aria-disabled="!!command.disabledReason"
            @mouseenter="!command.disabledReason && (activeId = command.id)"
            @click="!command.disabledReason && emit('run', command.id)"
          >
            <span class="label">{{ command.label }}</span>
            <span v-if="command.badge" class="badge badge--accent-outline">{{
              command.badge
            }}</span>
            <span v-if="command.disabledReason" class="palette-reason">{{
              command.disabledReason
            }}</span>
            <span v-else-if="command.shortcut" class="palette-shortcut">{{
              command.shortcut
            }}</span>
          </button>
        </template>
      </div>
      <div class="palette-footer">
        <span>↑↓ move · ⏎ run · esc close</span>
        <span>unavailable commands stay listed with a reason</span>
      </div>
    </div>
  </div>
</template>
