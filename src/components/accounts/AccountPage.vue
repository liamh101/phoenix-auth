<script setup lang="ts">

import NewAccount from "../NewAccount.vue";
import AccountList from "../AccountList.vue";
import Search from "../Search.vue";
import {ref} from "vue";

const showNewAccountForm = ref(false)
const accountFilter = ref('')

function showForm() {
  showNewAccountForm.value = true
}

function hideForm() {
  showNewAccountForm.value = false
}

function newAccountCreated() {
  showNewAccountForm.value = false
}

function filterAccounts(filter: string) {
  accountFilter.value = filter;
}

</script>

<template>
  <div class="nav-fill sticky-top mt-1">
    <div class="container-fluid">
      <div class="row">
        <div class="col-10">
          <Search @updated="filterAccounts"/>
        </div>

        <div class="col-2">
          <div class="d-grid gap-2">
            <button v-show="showNewAccountForm" @click="hideForm" class="btn btn-primary"><-</button>
            <button v-show="!showNewAccountForm" @click="showForm" class="btn btn-primary">+</button>
          </div>
        </div>
      </div>
    </div>
  </div>

  <div class="container-fluid">
    <div class="mt-2">
      <new-account v-if="showNewAccountForm" @created="newAccountCreated"/>

      <account-list v-if="!showNewAccountForm" :filter="accountFilter"/>
    </div>
  </div>
</template>

<style scoped>

</style>