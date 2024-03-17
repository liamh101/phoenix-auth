<script setup lang="ts">
import PageFooter from "./components/PageFooter.vue";
import AccountPage from "./components/accounts/AccountPage.vue";
import SettingsList from "./components/settings/SettingsList.vue";
import {ref} from "vue";

let displayAccountPage = ref(true);
let displaySettingsPage = ref(false);

function accountPageSelected() {
  displayAccountPage.value = true;
  displaySettingsPage.value = false;
}

function settingsPageSelected() {
  displayAccountPage.value = false;
  displaySettingsPage.value = true;
}


</script>

<template>
  <account-page v-if="displayAccountPage"></account-page>

  <settings-list v-if="displaySettingsPage"></settings-list>

  <page-footer @show-tokens="accountPageSelected" @show-settings="settingsPageSelected"></page-footer>
</template>

<style lang="scss" scoped>

</style>
