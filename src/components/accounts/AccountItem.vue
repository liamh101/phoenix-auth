<script setup lang="ts">

import DeleteAccount from "./DeleteAccount.vue";
import OneTimePassword from "./OneTimePassword.vue";
import {copyOtpToClipboard} from "../../composables/Generics.ts";
import CountdownTimer from "./CountdownTimer.vue";
import {ref} from "vue";

const props = defineProps({
  accountId: {
    type: Number,
    required: true,
  },
  accountName: {
    type: String,
    required: true,
  },
  manage: {
    type: Boolean,
    default: false,
  }
})

const emit = defineEmits(['accountRemoved', 'accountEdit']);

const displayPassword = ref(false)
const copyingCode = ref(false)

function accountRemoved() {
  emit('accountRemoved')
}

function editAccount() {
  emit('accountEdit', props.accountId)
}

async function copyToClipboard() {
  if (props.manage) {
    return;
  }

  copyingCode.value = true;

  await copyOtpToClipboard(props.accountId);

  setTimeout(() => copyingCode.value = false, 100)
}

function togglePassword() {
  displayPassword.value = !displayPassword.value
}

</script>

<template>
  <li
    class="list-group-item"
    :class="{'selector': !manage, 'code-copy': copyingCode}"
    @click.self="copyToClipboard"
  >
    <div class="row">
      <div
        v-if="!manage && displayPassword"
        class="col-2"
        @click="copyToClipboard"
      >
        <countdown-timer :timeout="30" />
      </div>
      <div
        class="account-overflow"
        :class="{'col-10': !manage && !displayPassword, 'col-8': displayPassword || manage}"
        @click="copyToClipboard"
      >
        <span
          v-if="!displayPassword"
          class="list-item-text"
        >{{ props.accountName }}</span>

        <one-time-password
          v-if="!props.manage && displayPassword"
          :account-id="props.accountId"
        />
      </div>
      <div
        v-if="!manage"
        class="col-2"
        @click.self="copyToClipboard"
      >
        <button
          v-if="!manage && !displayPassword"
          class="btn btn-secondary btn-circle btn-lg"
          @click="togglePassword"
        >
          <i class="fa-solid fa-star-of-life icon-size" />
        </button>

        <button
          v-if="!manage && displayPassword"
          class="btn btn-secondary btn-circle btn-lg"
          @click="togglePassword"
        >
          <i class="fa-solid fa-tag icon-size" />
        </button>
      </div>

      <div
        v-if="manage"
        class="col-2 d-grid gap-2"
      >
        <button
          class="btn btn-warning"
          @click="editAccount"
        >
          <i class="fa-solid fa-pen-to-square" />
        </button>
      </div>

      <div
        v-if="manage"
        class="col-2 d-grid gap-2"
      >
        <delete-account
          v-if="props.manage"
          :account-id="props.accountId"
          @success="accountRemoved"
        />
      </div>
    </div>
  </li>
</template>

<style scoped lang="scss">
.btn-circle {
  border-radius: 50%;
}

.code-copy {
  background-color: #d3d3d3;
}

[data-theme="dark"] .code-copy {
  background-color: #363636;
}

.icon-size {
  height: 20px;
  width: 20px;
}
</style>