<script setup lang="ts">

import DeleteAccount from "./DeleteAccount.vue";
import OneTimePassword from "./OneTimePassword.vue";
import {copyOtpToClipboard} from "../../composables/Generics.ts";
import CountdownTimer from "./CountdownTimer.vue";
import {computed, ref} from "vue";
import AccountButton from "./AccountButton.vue";

enum Display {
  LABEL,
  PASSWORD
}

const props = defineProps({
  accountId: {
    type: Number,
    required: true,
  },
  accountName: {
    type: String,
    required: true,
  },
  accountColour: {
    type: String,
    required: true,
  },
  manage: {
    type: Boolean,
    default: false,
  }
})

const emit = defineEmits(['accountRemoved', 'accountEdit']);

const currentDisplay = ref(Display.LABEL)
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

function displayPassword() {
  currentDisplay.value = Display.PASSWORD;
}

function displayLabel() {
  currentDisplay.value = Display.LABEL
}

const passwordDisplayed = computed(() => currentDisplay.value === Display.PASSWORD)
const labelDisplayed = computed(() => currentDisplay.value === Display.LABEL)

</script>

<template>
  <li
    class="list-group-item"
    :class="{'selector': !manage, 'code-copy': copyingCode}"
    @click.self="copyToClipboard"
  >
    <div class="row">
      <div
        v-if="!manage && passwordDisplayed"
        class="col-2"
        @click="copyToClipboard"
      >
        <countdown-timer :timeout="30" />
      </div>
      <div
        class="account-overflow"
        :class="{'col-10': !manage && labelDisplayed, 'col-8': passwordDisplayed || manage}"
        @click="copyToClipboard"
      >
        <span
          v-if="labelDisplayed"
          class="list-item-text"
        >{{ props.accountName }}</span>

        <one-time-password
          v-if="!props.manage && passwordDisplayed"
          :account-id="props.accountId"
        />
      </div>
      <div
        v-if="!manage"
        class="col-2"
        @click.self="copyToClipboard"
      >
        <account-button
          :button-colour="accountColour"
          @display-label="displayLabel"
          @display-password="displayPassword"
        />
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

.code-copy {
  background-color: #d3d3d3;
}

[data-theme="dark"] .code-copy {
  background-color: #363636;
}

</style>