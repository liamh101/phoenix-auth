<script setup lang="ts">

import AccountForm from "./AccountForm.vue";
import AccountList from "./AccountList.vue";
import Search from "./AccountSearch.vue";
import {ref} from "vue";
import PageHeader from "../PageHeader.vue";
import PageFooter from "../PageFooter.vue";

const showNewAccountForm = ref(false)
const accountFilter = ref('')

const emit = defineEmits(['showSettings']);

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

function showSettings() {
  emit('showSettings');
}

function reset() {
  showNewAccountForm.value = false;
}

</script>

<template>
  <page-header>
    <div class="row">
      <div class="col-10">
        <Search @updated="filterAccounts" />
      </div>

      <div class="col-2">
        <div class="d-grid gap-2">
          <button
            v-show="showNewAccountForm"
            class="btn btn-primary"
            @click="hideForm"
          >
            <i class="fa-solid fa-arrow-left" />
          </button>
          <button
            v-show="!showNewAccountForm"
            class="btn btn-primary"
            @click="showForm"
          >
            <i class="fa-solid fa-plus" />
          </button>
        </div>
      </div>
    </div>
  </page-header>

  <div class="container-fluid main-content">
    <div class="mt-2">
      <account-form
        v-if="showNewAccountForm"
        @created="newAccountCreated"
      />
    </div>
  </div>

  <account-list
    v-if="!showNewAccountForm"
    class="main-content"
    :filter="accountFilter"
  />

  <page-footer
    @show-settings="showSettings"
    @show-tokens="reset"
  />
</template>