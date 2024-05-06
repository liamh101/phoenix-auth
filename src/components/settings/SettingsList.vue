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
        <span class="list-item-text"> <i class="fa-solid fa-users-gear"></i> Manage Accounts</span>
      </li>
      <li
        class="list-group-item selector"
        @click="viewImportAccounts"
      >
        <span class="list-item-text"><i class="fa-solid fa-download"></i> Import Accounts</span>
      </li>
    </ul>
  </div>
</template>