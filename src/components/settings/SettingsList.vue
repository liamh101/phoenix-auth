<script setup lang="ts">
  const emit = defineEmits(['showManageAccounts']);

  function viewManageAccounts() {
    emit('showManageAccounts')
  }
</script>

<template>
  <div class="card">
    <ul class="list-group list-group-flush">
      <li class="list-group-item" @click="viewManageAccounts">
        <h1>Manage Accounts</h1>
      </li>
    </ul>

  </div>
</template>

<style scoped>

</style>