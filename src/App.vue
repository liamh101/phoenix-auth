<script setup lang="ts">
import NewAccount from "./components/NewAccount.vue";
import AccountList from "./components/AccountList.vue";
</script>

<template>
  <div class="container">
    <new-account />

    <account-list />
  </div>
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}
</style>
