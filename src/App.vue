<script setup lang="ts">
import AccountPage from "./components/accounts/AccountPage.vue";
import {ref, onMounted} from "vue";
import SettingsPage from "./components/settings/SettingsPage.vue";
import {getSettings, THEME_MODES} from "./composables/Commands.ts";
import {getOsTheme} from "./composables/OperatingSystem.ts";

let displayAccountPage = ref(true);
let displaySettingsPage = ref(false);
let override_theme = ref(THEME_MODES.DEFAULT)

function accountPageSelected() {
  displayAccountPage.value = true;
  displaySettingsPage.value = false;
}

function settingsPageSelected() {
  displayAccountPage.value = false;
  displaySettingsPage.value = true;
}

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
  if (settingTheme === THEME_MODES.LIGHT) {
    return 'light'
  }

  return 'dark';
}

function setThemeAttributes(theme: string) {
  document.documentElement.setAttribute("data-theme", theme);
  document.documentElement.setAttribute("data-bs-theme", theme);
}

function init() {
  setupTheme()
}

onMounted(() => init())
</script>

<template>
  <div>
    <theme :override-theme="override_theme" />

    <account-page
      v-if="displayAccountPage"
      @show-settings="settingsPageSelected"
    />

    <settings-page
      v-if="displaySettingsPage"
      @show-tokens="accountPageSelected"
      @change-theme="setupTheme"
    />
  </div>
</template>

<style lang="scss" scoped>

</style>
