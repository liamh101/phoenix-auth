<script setup lang="ts">
import {onMounted, ref} from "vue";
import {invoke} from "@tauri-apps/api/tauri";
import OneTimePassword from "./OneTimePassword.vue";

  const accounts = ref([])

  async function getAccounts() {
    const result = JSON.parse(await invoke("get_all_accounts"));

    if (Array.isArray(result)) {
      accounts.value = result;
    }
  }

  onMounted(() => getAccounts())
</script>

<template>
  <table>
    <thead>
    <tr>
      <th>Id</th>
      <th>Name</th>
      <th>OTP</th>
    </tr>
    </thead>
    <tbody>
    <tr v-for="account in accounts">
      <td v-text="account.id"></td>
      <td v-text="account.name"></td>
      <td><one-time-password :account-id="account.id"/></td>
    </tr>
    </tbody>
  </table>
</template>

<style scoped>

</style>