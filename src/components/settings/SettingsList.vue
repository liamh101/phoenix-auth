<script setup lang="ts">
  const emit = defineEmits(['showAppearanceSettings', 'showManageAccounts', 'showImportAccounts', 'showSyncAccounts']);
  import {exportAccounts} from "../../composables/Commands.ts";

  function viewAppearanceSettings() {
    emit('showAppearanceSettings');
  }

  function viewManageAccounts() {
    emit('showManageAccounts')
  }

  function viewImportAccounts() {
    emit('showImportAccounts');
  }

  async function exportAllAccounts() {
    await exportAccounts();
  }

  function viewSyncAccounts() {
    emit('showSyncAccounts');
  }
</script>

<template>
  <div class="card">
    <ul class="list-group list-group-flush">
      <li
        class="list-group-item selector"
        @click="viewAppearanceSettings"
      >
        <div class="row">
          <div class="col-1">
            <div class="text-center pt-1">
              <i class="fa-solid fa-brush fa-2xl" />
            </div>
          </div>
          <div class="col-11">
            <span class="list-item-text">Appearance</span>
          </div>
        </div>
      </li>
      <li
        class="list-group-item selector"
        @click="viewManageAccounts"
      >
        <div class="row">
          <div class="col-1">
            <div class="text-center pt-1">
              <i class="fa-solid fa-users-gear fa-2xl" />
            </div>
          </div>
          <div class="col-11">
            <span class="list-item-text">Manage Accounts</span>
          </div>
        </div>
      </li>
      <li
        class="list-group-item selector"
        @click="viewImportAccounts"
      >
        <div class="row">
          <div class="col-1">
            <div class="text-center pt-1">
              <i class="fa-solid fa-download fa-2xl" />
            </div>
          </div>
          <div class="col-11">
            <span class="list-item-text">Import Accounts</span>
          </div>
        </div>
      </li>
      <li
        class="list-group-item selector"
        @click="exportAllAccounts"
      >
        <div class="row">
          <div class="col-1">
            <div class="text-center pt-1">
              <i class="fa-solid fa-upload fa-2xl" />
            </div>
          </div>
          <div class="col-11">
            <span class="list-item-text">Export Accounts</span>
          </div>
        </div>
      </li>
      <li
        class="list-group-item selector"
        @click="viewSyncAccounts"
      >
        <div class="row">
          <div class="col-1">
            <div class="text-center pt-1">
              <i class="fa-solid fa-rotate fa-2xl" />
            </div>
          </div>
          <div class="col-11">
            <span class="list-item-text">Sync Accounts</span>
          </div>
        </div>
      </li>
    </ul>
  </div>
</template>