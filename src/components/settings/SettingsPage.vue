<script setup lang="ts">
import SettingsList from "./SettingsList.vue";
import {computed, ref} from "vue";
import AccountList from "../accounts/AccountList.vue";
import PageHeader from "../PageHeader.vue";
import PageFooter from "../PageFooter.vue";
import AccountImportPage from "./imports/AccountImportPage.vue";
import AccountSyncPage from "./sync/AccountSyncPage.vue";
import {attemptSyncAccounts, THEME_MODES} from "../../composables/Commands.ts";
import AccountForm from "../accounts/AccountForm.vue";
import AppearanceSettings from "./AppearanceSettings.vue";

const displayAppearanceSettings = ref(false);
const displayManageAccounts = ref(false);
const displayImportPage = ref(false);
const displaySyncPage = ref(false);
const displayEditAccountPage = ref(false);
const syncRequired = ref(false);

const editAccountId = ref(0);

const emit = defineEmits(['showTokens', 'changeTheme']);

function showAppearanceSettings() {
  displayAppearanceSettings.value = true;
}

function showManageAccounts() {
  displayManageAccounts.value = true;
}

function showImportPage() {
  displayImportPage.value = true;
}

function showSyncPage() {
  displaySyncPage.value = true;
}

function showEditAccountPage(accountId: number) {
  reset();
  editAccountId.value = accountId;
  displayEditAccountPage.value = true;
}

function accountEdited() {
  prepareSync();
  reset();
  editAccountId.value = 0;
  showManageAccounts();
}

function showTokens() {
  emit('showTokens');

  performSyncIfRequired();
}

function reset() {
  displayAppearanceSettings.value = false;
  displayEditAccountPage.value = false;
  displayManageAccounts.value = false;
  displayImportPage.value = false;
  displaySyncPage.value = false;

  performSyncIfRequired();
}

function prepareSync() {
  syncRequired.value = true;
}

function performSyncIfRequired() {
  if (syncRequired.value) {
    attemptSyncAccounts();
  }
}

function themeChanged(theme: THEME_MODES) {
  emit('changeTheme', theme)
}

const hideSettingsList = computed(() => displayAppearanceSettings.value || displayManageAccounts.value || displayImportPage.value || displaySyncPage.value || displayEditAccountPage.value)
</script>

<template>
  <div>
    <page-header v-if="hideSettingsList" />

    <settings-list
      v-if="!hideSettingsList"
      class="main-content no-header"
      @show-appearance-settings="showAppearanceSettings"
      @show-manage-accounts="showManageAccounts"
      @show-import-accounts="showImportPage"
      @show-sync-accounts="showSyncPage"
    />

    <appearance-settings
      v-if="displayAppearanceSettings"
      class="container-fluid main-content"
      @theme-change="themeChanged"
    />

    <account-list
      v-if="displayManageAccounts"
      class="main-content"
      manage
      @sync-required="prepareSync"
      @edit-account="showEditAccountPage"
    />

    <AccountImportPage
      v-if="displayImportPage"
      class="main-content"
      @go-back-to-accounts="showTokens"
    />

    <AccountSyncPage
      v-if="displaySyncPage"
      class="container-fluid main-content"
    />

    <div
      v-if="displayEditAccountPage"
      class="container-fluid main-content"
    >
      <div class="mt-2">
        <account-form
          :account-id="editAccountId"
          @edited="accountEdited"
        />
      </div>
    </div>

    <page-footer
      @show-tokens="showTokens"
      @show-settings="reset"
    />
  </div>
</template>