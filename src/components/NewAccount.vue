<script setup lang="ts">
import {ref} from "vue";
import {invoke} from "@tauri-apps/api/tauri";

const name = ref("");
const secret = ref("");

async function createNewAccount() {
  await invoke("create_new_account", { name: name.value, secret: secret.value });
}

</script>

<template>
  <div>
    <form class="row" @submit.prevent="createNewAccount">
      <input id="name" v-model="name" placeholder="Enter name" />
      <input id="secret" v-model="secret" placeholder="Enter secret" />
      <button type="submit">Create Account</button>
    </form>
  </div>
</template>

<style scoped>

</style>