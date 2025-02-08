<script setup lang="ts">
import AccountPage from "./components/accounts/AccountPage.vue";
import {ref} from "vue";
import SettingsPage from "./components/settings/SettingsPage.vue";
import Theme from "./components/Theme.vue";
import {THEME_MODES} from "./composables/Commands.ts";

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

function themeChanged(theme: THEME_MODES) {
  override_theme.value = theme;
}

</script>

<template>
  <div>
    <theme :override-theme="override_theme"/>

    <account-page
        v-if="displayAccountPage"
        @show-settings="settingsPageSelected"
    />

    <settings-page
        v-if="displaySettingsPage"
        @show-tokens="accountPageSelected"
        @change-theme="themeChanged"
    />
  </div>
</template>

<style lang="scss" scoped>

</style>
