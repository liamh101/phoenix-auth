<script setup lang="ts">

import DeleteAccount from "./DeleteAccount.vue";
import OneTimePassword from "./OneTimePassword.vue";
import {copyTextToClipboard} from "../../composables/Generics.ts";
import CountdownTimer from "./CountdownTimer.vue";
import {computed} from "vue";

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

async function copyToClipboard() {
  await copyTextToClipboard(currentToken);
}

function setCurrentToken(token: string) {
  currentToken = token;
}

</script>

<template>
  <li class="list-group-item" :class="{'token-selector': !manage}" @click="copyToClipboard">
    <div class="row">
      <div v-if="!manage" class="col-2">
        <countdown-timer :timeout="30"></countdown-timer>
      </div>
      <div :class="{'col-7': manage, 'col-5': !manage}">
        <h2>{{props.accountName}}</h2>
      </div>
      <div class="col-5">
        <one-time-password v-if="!props.manage" :account-id="props.accountId" @otp="setCurrentToken"/>

        <delete-account v-if="props.manage" :account-id="props.accountId" @success="accountRemoved"/>
      </div>
    </div>
  </li>
</template>

<style scoped lang="scss">
  .token-selector {
    cursor: pointer;
  }
</style>