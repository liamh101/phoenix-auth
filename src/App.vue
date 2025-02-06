<script setup lang="ts">
import AccountPage from "./components/accounts/AccountPage.vue";
import {ref} from "vue";
import SettingsPage from "./components/settings/SettingsPage.vue";
import Theme from "./components/Theme.vue";

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
  <div>
    <theme/>

    <account-page
        v-if="displayAccountPage"
        @show-settings="settingsPageSelected"
    />

    <settings-page
        v-if="displaySettingsPage"
        @show-tokens="accountPageSelected"
    />
  </div>
</template>

<style lang="scss" scoped>

</style>
