<script setup lang="ts">
import {ref} from "vue";
import {createNewAccount} from "../composables/Commands.ts";

const name = ref("");
const secret = ref("");
const message = ref("");

async function submitForm() {
  const response = await createNewAccount(name.value, secret.value);

  message.value = response.message;
}

function shouldDisable() {
  return name.value.length === 0 || secret.value.length === 0
}

</script>

<template>
  <div>
    <form class="row" @submit.prevent="submitForm">
      <input id="name" v-model="name" placeholder="Enter name" />
      <input id="secret" v-model="secret" placeholder="Enter secret" />
      <button id="newUserSubmit" :disabled="shouldDisable()" type="submit">Create Account</button>
    </form>

    <p v-text="message"></p>
  </div>
</template>

<style scoped>

</style>