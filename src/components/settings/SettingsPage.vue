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
    <page-header />

    <settings-list
      v-if="!hideSettingsList"
      class="main-content"
      @show-manage-accounts="showManageAccounts"
    />

    <account-list
      v-if="displayManageAccounts"
      class="main-content"
      manage
    />

    <page-footer @show-tokens="showTokens" />
  </div>
</template>