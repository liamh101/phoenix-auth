<script setup lang="ts">
import {onMounted, watch} from 'vue'
import {getOsTheme} from "../composables/OperatingSystem.ts";
import {getSettings, THEME_MODES} from "../composables/Commands.ts";

const props = defineProps({
    overrideTheme: {
      type: Number,
    }
  })

  watch(() => props.overrideTheme, () => {
    setupTheme();
  })

  async function setupTheme() {
    const settings = await getSettings();

    if (settings.settings.theme === THEME_MODES.DEFAULT) {
      const systemTheme = await getOsTheme();

      setThemeAttributes(systemTheme)
      return;
    }

    const userTheme = getThemeFromMode(settings.settings.theme);

    setThemeAttributes(userTheme)
    return;
  }

  function getThemeFromMode(settingTheme: THEME_MODES) {
    switch (settingTheme) {
      case THEME_MODES.LIGHT:
        return 'light';
      case THEME_MODES.DARK:
        return 'dark';
    }

    return 'dark';
  }

  function setThemeAttributes(theme: string) {
    document.documentElement.setAttribute("data-theme", theme);
    document.documentElement.setAttribute("data-bs-theme", theme);
  }

  onMounted(() => setupTheme())
</script>

<template>

</template>

<style scoped lang="scss">

</style>