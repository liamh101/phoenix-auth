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
  <div class="card">
    <ul class="list-group list-group-flush">
      <li v-for="account in accounts" class="list-group-item">
        <div class="row">
          <div class="col">
            <h2>{{account.name}}</h2>
          </div>
          <div class="col">
            <one-time-password :account-id="account.id"/>
          </div>
        </div>
      </li>
      <li v-if="accounts.length === 0">No accounts added</li>
    </ul>
  </div>
</template>

<style scoped>

</style>