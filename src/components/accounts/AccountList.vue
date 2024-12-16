<script setup lang="ts">
import {onMounted, ref, watch} from "vue";
import {Account, getAllAccounts} from "../../composables/Commands.ts";
import AccountItem from "./AccountItem.vue";

  const props = defineProps({
    filter: {
      type: String,
      required: false,
      default: '',
    },
    manage: {
      type: Boolean,
      default: false,
    }
  })

  const emit = defineEmits(['syncRequired']);

  let accountArray: Account[] = [];
  const accounts = ref(accountArray)

  async function getAccounts() {
    const response = await getAllAccounts(props.filter);

    accounts.value = response.accounts;
  }

  function accountRemoved() {
    emit('syncRequired');
    getAccounts();
  }

  watch(() => props.filter, () => getAccounts())

  onMounted(() => getAccounts())
</script>

<template>
  <div
    class="card overflow-auto"
    :class="{'hide-border': !accounts.length}"
  >
    <ul class="list-group list-group-flush">
      <account-item
        v-for="account in accounts"
        :key="account.id"
        :account-id="account.id"
        :account-name="account.name"
        :manage="manage"
        @account-removed="accountRemoved"
      />
    </ul>

    <div
      v-if="accounts.length === 0"
      class="container-fluid mt-5"
    >
      <div class="row">
        <div class="col">
          <h2 class="text-center">
            No accounts found
          </h2>
        </div>
      </div>
    </div>
  </div>
</template>

<style lang="scss">

.hide-border {
  border-bottom: none;
}

</style>