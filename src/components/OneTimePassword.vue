 <script setup lang="ts">
 import {onMounted, ref} from "vue";
import { invoke } from "@tauri-apps/api/tauri";
 import {generateToken} from "../composables/Commands.ts";

const props = defineProps({
  accountId: {
    type: Number,
    required: true,
  }
});

const DEFAULT_TEXT = '------'

const otp = ref(DEFAULT_TEXT);

async function getOneTimePassword() {
  console.log('Hello world')
  otp.value = (await generateToken(props.accountId)).token
}

function onExit() {
  otp.value = DEFAULT_TEXT;
}

function copyToClipboard() {
  navigator.clipboard.writeText(otp.value)
}

</script>

<template>
  <button  @click="copyToClipboard" @mouseover="getOneTimePassword" @mouseleave="onExit">{{ otp }}</button>
</template>