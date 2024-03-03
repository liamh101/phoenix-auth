<script setup lang="ts">
import {onMounted, ref} from "vue";
import OneTimePassword from "./OneTimePassword.vue";
import {getAllAccounts} from "../composables/Commands.ts";

  const accounts = ref([])

  async function getAccounts() {
    const response = await getAllAccounts();

    accounts.value = response.accounts;
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
    <tr v-if="accounts.length === 0">
      <td colspan="3">No accounts added</td>
    </tr>
    </tbody>
  </table>
</template>

<style scoped>

</style>