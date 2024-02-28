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

const otp = ref("");

async function getOneTimePassword() {
  otp.value = (await generateToken(props.accountId)).token
}


onMounted(() => {
  getOneTimePassword()
  setInterval(getOneTimePassword, 30000)
})
</script>

<template>
  <p>{{ otp }}</p>
</template>