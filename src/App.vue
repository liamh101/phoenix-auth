<script setup lang="ts">
import NewAccount from "./components/NewAccount.vue";
import AccountList from "./components/AccountList.vue";
import {ref} from "vue";

const showNewAccountForm = ref(false)

function showForm() {
  showNewAccountForm.value = true
}

function hideForm() {
  showNewAccountForm.value = false
}


function newAccountCreated() {
  showNewAccountForm.value = false
}
</script>

<template>
  <div class="container">
    <div>


      <button v-show="showNewAccountForm" @click="hideForm"><-</button>
      <button v-show="!showNewAccountForm" @click="showForm">+</button>

    </div>


    <new-account v-if="showNewAccountForm" @created="newAccountCreated"/>

    <account-list v-if="!showNewAccountForm" />
  </div>
</template>
