<script setup lang="ts">
import {onMounted, ref} from "vue";
import {getSettings, ResponseType, saveSettings, THEME_MODES} from "../../composables/Commands.ts";

const themeMode = ref(THEME_MODES.DEFAULT)

const emit = defineEmits(['themeChange'])

async function getDefaultSettings() {
  const response = await getSettings();

  if (response.response === ResponseType.FAILURE) {
    return;
  }

  themeMode.value = response.settings.theme;
}

function saveTheme() {
  saveSettings(themeMode.value)
  emit("themeChange", themeMode.value)
}

onMounted(() => getDefaultSettings())
</script>

<template>
  <div>
    <label
      for="theme-mode"
      class="form-label"
    >Theme</label>

    <select
      id="theme-mode"
      v-model="themeMode"
      class="form-select"
      @change="saveTheme"
    >
      <option
        :value="THEME_MODES.DEFAULT"
        selected
      >
        OS Default
      </option>
      <option :value="THEME_MODES.DARK">
        Dark
      </option>
      <option :value="THEME_MODES.LIGHT">
        Light
      </option>
    </select>
  </div>
</template>

<style scoped lang="scss">

</style>