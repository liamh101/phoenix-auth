<script setup lang="ts">
import SettingsList from "./SettingsList.vue";
import {computed, ref} from "vue";
import AccountList from "../accounts/AccountList.vue";
import PageHeader from "../PageHeader.vue";
import PageFooter from "../PageFooter.vue";
import AccountImportPage from "./imports/AccountImportPage.vue";

const displayManageAccounts = ref(false);
const displayImportPage = ref(false);

const emit = defineEmits(['showTokens']);

function showManageAccounts() {
  displayManageAccounts.value = true;
}

function showImportPage() {
  displayImportPage.value = true;
}

function showTokens() {
  emit('showTokens')
}

const hideSettingsList = computed(() => displayManageAccounts.value || displayImportPage.value)
</script>

<template>
  <div>
    <page-header />

    <settings-list
      v-if="!hideSettingsList"
      class="main-content"
      @show-manage-accounts="showManageAccounts"
      @show-import-accounts="showImportPage"
    />

    <account-list
      v-if="displayManageAccounts"
      class="main-content"
      manage
    />

    <AccountImportPage
      v-if="displayImportPage"
      class="main-content"
      @go-back-to-accounts="showTokens"
    />

    <page-footer @show-tokens="showTokens" />
  </div>
</template>