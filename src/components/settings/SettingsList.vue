<script setup lang="ts">
  const emit = defineEmits(['showManageAccounts', 'showImportAccounts']);

  function viewManageAccounts() {
    emit('showManageAccounts')
  }

  function viewImportAccounts() {
    emit('showImportAccounts');
  }
</script>

<template>
  <div class="card">
    <ul class="list-group list-group-flush">
      <li
        class="list-group-item selector"
        @click="viewManageAccounts"
      >
        <h1>Manage Accounts</h1>
      </li>
      <li
        class="list-group-item selector"
        @click="viewImportAccounts"
      >
        <h1>Import Accounts</h1>
      </li>
    </ul>
  </div>
</template>