<script setup lang="ts">
import SettingsList from "./SettingsList.vue";
import {computed, ref} from "vue";
import AccountList from "../accounts/AccountList.vue";
import PageHeader from "../PageHeader.vue";
import PageFooter from "../PageFooter.vue";

const displayManageAccounts = ref(false);

const emit = defineEmits(['showTokens']);

function showManageAccounts() {
  displayManageAccounts.value = true;
}

function showTokens() {
  emit('showTokens')
}

const hideSettingsList = computed(() => displayManageAccounts.value)
</script>

<template>
  <div>
    <page-header></page-header>

    <settings-list v-if="!hideSettingsList" class="main-content" @show-manage-accounts="showManageAccounts"></settings-list>

    <account-list v-if="displayManageAccounts" class="main-content" manage></account-list>

    <page-footer @show-tokens="showTokens"></page-footer>
  </div>
</template>