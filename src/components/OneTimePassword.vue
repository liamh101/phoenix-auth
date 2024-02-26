 <script setup lang="ts">
 import {onMounted, ref} from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const props = defineProps({
  accountId: {
    type: Number,
  }
});

const otp = ref("");

async function getOneTimePassword() {
  otp.value = await invoke("get_one_time_password_for_account", { account: props.accountId });
}


onMounted(() => {
  getOneTimePassword()
  setInterval(getOneTimePassword, 30000)
})
</script>

<template>
  <p>{{ otp }}</p>
</template>