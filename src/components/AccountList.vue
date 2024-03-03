<script setup lang="ts">
import {onMounted, ref, watch} from "vue";
  import OneTimePassword from "./OneTimePassword.vue";
  import {getAllAccounts} from "../composables/Commands.ts";

  const props = defineProps({
    filter: {
      type: String,
      required: false,
      default: '',
    }
  })

  const accounts = ref([])

  async function getAccounts() {
    const response = await getAllAccounts(props.filter);

    accounts.value = response.accounts;
  }

  watch(() => props.filter, () => getAccounts())

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
      <li v-if="accounts.length === 0">
        <div class="row">
          <div class="col">
            <h2 class="text-center">No accounts found</h2>
          </div>
        </div>
      </li>
    </ul>
  </div>
</template>

<style scoped>

</style>