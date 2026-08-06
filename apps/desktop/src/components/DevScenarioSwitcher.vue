<script setup lang="ts">
// Not product chrome. The design has no persistent navigation ("command-
// driven navigation, no persistent chrome" — decided in the design doc's
// opening notes), so this panel is deliberately styled apart from it: it
// exists only so every screen in the design is reachable for review without
// a Rust backend to trigger recovery failures, dropped Operations, etc.
//
// It is explicitly opt-in and visually non-structural: App.vue only mounts
// this component at all when a dev build is running AND the page was
// loaded with ?review=1 — the ordinary `tauri dev` / `vite dev` route
// renders none of it, not even a collapsed tab. When it is mounted, it
// still starts collapsed to a single small floating tab, position:fixed so
// it never resizes or reflows the product surface underneath it.
import { ref } from "vue";
import { sceneGroups, type SceneId } from "../fixtures/scenarios";

defineProps<{ scene: SceneId; theme: "system" | "light" | "dark" }>();
defineEmits<{
  "update:scene": [id: SceneId];
  "update:theme": [theme: "system" | "light" | "dark"];
}>();

const open = ref(false);
</script>

<template>
  <button
    class="dev-switcher-tab"
    type="button"
    :aria-expanded="open"
    aria-controls="dev-switcher-panel"
    @click="open = !open"
  >
    {{ open ? "✕ close" : "⚙ dev" }}
  </button>

  <aside
    v-if="open"
    id="dev-switcher-panel"
    class="dev-switcher"
    aria-label="Design review scenario switcher (not part of the product)"
  >
    <div class="dev-switcher-title">Design review · not product UI</div>

    <div class="dev-switcher-group">
      <div class="dev-switcher-group-label">Theme</div>
      <div class="dev-switcher-toggles">
        <button
          v-for="t in ['system', 'light', 'dark'] as const"
          :key="t"
          class="dev-switcher-btn"
          :class="{ active: theme === t }"
          type="button"
          @click="$emit('update:theme', t)"
        >
          {{ t }}
        </button>
      </div>
    </div>

    <div
      v-for="group in sceneGroups"
      :key="group.label"
      class="dev-switcher-group"
    >
      <div class="dev-switcher-group-label">{{ group.label }}</div>
      <button
        v-for="option in group.scenes"
        :key="option.id"
        class="dev-switcher-btn"
        :class="{ active: scene === option.id }"
        type="button"
        @click="$emit('update:scene', option.id)"
      >
        {{ option.label }}
      </button>
    </div>
  </aside>
</template>
