<script setup lang="ts">
import SettingsList from "./SettingsList.vue";
import {computed, ref} from "vue";
import AccountList from "../accounts/AccountList.vue";

const displayManageAccounts = ref(false);

function showManageAccounts() {
  displayManageAccounts.value = true;
}

const hideSettingsList = computed(() => displayManageAccounts.value)
</script>

<template>
  <div>
    <settings-list v-if="!hideSettingsList" @show-manage-accounts="showManageAccounts"></settings-list>

    <account-list v-if="displayManageAccounts" manage></account-list>
  </div>
</template>

<style scoped>

</style>