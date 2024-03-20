<script setup lang="ts">

import DeleteAccount from "./DeleteAccount.vue";
import OneTimePassword from "./OneTimePassword.vue";

const props = defineProps({
  accountId: {
    type: Number,
  },
  accountName: {
    type: String,
  },
  manage: {
    type: Boolean,
    default: false,
  }
})

const emit = defineEmits(['accountRemoved']);

let currentToken: string = '';

function accountRemoved() {
  emit('accountRemoved')
}

function copyToClipboard() {
  navigator.clipboard.writeText(currentToken)
}

function setCurrentToken(token: string) {
  currentToken = token;
}

</script>

<template>
  <li class="list-group-item" @click="copyToClipboard">
    <div class="row">
      <div class="col">
        <h2>{{props.accountName}}</h2>
      </div>
      <div class="col">
        <one-time-password v-if="!props.manage" :account-id="props.accountId" @otp="setCurrentToken"/>

        <delete-account v-if="props.manage" :account-id="props.accountId" @success="accountRemoved"/>
      </div>
    </div>
  </li>
</template>

<style scoped lang="scss">
  .list-group-item {
    cursor: pointer;
  }
</style>