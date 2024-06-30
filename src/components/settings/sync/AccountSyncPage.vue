<script setup lang="ts">
import {ref} from "vue";
import {validateSyncAccount} from "../../../composables/Commands.ts";

const host = ref('');
const username = ref('');
const password = ref('');

const message = ref('');

const loading = ref(false);
const validAccount = ref(false);

async function submitForm() {
  loading.value = true;
  message.value = '';
  const response = await validateSyncAccount(host.value, username.value, password.value)

  loading.value = false;
  message.value = response.message;
}

</script>

<template>
  <div>
    <form
      class="row"
      @submit.prevent="submitForm"
    >
      <div class="mb-3">
        <label
          for="host-url"
          class="form-label"
        >Host URL</label>
        <input
          id="host-url"
          v-model="host"
          class="form-control"
        >
      </div>

      <div class="mb-3">
        <label
          for="username"
          class="form-label"
        >Username</label>
        <input
          id="username"
          v-model="username"
          class="form-control"
        >
      </div>

      <div class="mb-3">
        <label
          for="password"
          class="form-label"
        >Password</label>
        <input
          id="password"
          v-model="password"
          type="password"
          class="form-control"
        >
      </div>

      <div class="mb-3">
        <div class="d-grid gap-2">
          <button
            id="newUserSubmit"
            class="btn btn-primary"
            type="submit"
            :disabled="loading"
          >
            Validate
          </button>
        </div>
      </div>
    </form>

    <p class="text-center" v-text="message" />
  </div>
</template>

<style scoped lang="scss">

</style>