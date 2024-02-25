 <script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const otp = ref("");
const secret = ref("");

async function getOneTimePassword() {
  otp.value = await invoke("get_one_time_password", { secret: secret.value });
}
</script>

<template>
  <div>
      <form class="row" @submit.prevent="getOneTimePassword">
        <input id="secret-input" v-model="secret" placeholder="Enter secret" />
        <button type="submit">Get OTP</button>
      </form>

      <p>{{ otp }}</p>
  </div>
</template>