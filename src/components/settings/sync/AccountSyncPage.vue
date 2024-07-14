<script setup lang="ts">
import {computed, onMounted, ref} from "vue";
import {ResponseType, saveSyncAccount, validateSyncAccount, getExistingAccount} from "../../../composables/Commands.ts";

const host = ref('');
const username = ref('');
const password = ref('');

const message = ref('');

const loading = ref(false);
const lockdownForm = ref(false);

async function init() {
 const response = await getExistingAccount();

 if (response.response === ResponseType.SUCCESS && response.syncAccount) {
   host.value = response.syncAccount.url;
   username.value = response.syncAccount.username;
   password.value = response.syncAccount.password;
   lockdownForm.value = true;
 }
}

function submitForm() {
  if (lockdownForm.value) {
    resetForm();
    return;
  }

  submitValidationForm();
}

async function submitValidationForm() {
  loading.value = true;
  message.value = '';
  const response = await validateSyncAccount(host.value, username.value, password.value)
  const validAccount = response.response === ResponseType.SUCCESS
  message.value = response.message;

  if (validAccount) {
    const savedResponse = await saveSyncAccount({
      id: null,
      username: username.value,
      password: password.value,
      url: host.value,
    });

    if (savedResponse.response === ResponseType.SUCCESS) {
      lockdownForm.value = true;
    }
  }

  loading.value = false;
}

function resetForm() {
  lockdownForm.value = false;
}

const submitButtonMessage = computed(() => lockdownForm.value ? 'Change Details' : 'Validate & Save')

onMounted(() => init())

</script>

<template>
  <div>
    <form
      @submit.prevent="submitForm"
    >
      <fieldset
          class="row"
          :disabled="lockdownForm || loading"
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

        <div v-if="!lockdownForm" class="mb-3">
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
      </fieldset>

      <fieldset
          class="row"
          :disabled="loading"
      >
        <div class="mb-3">
          <div class="d-grid gap-2">
            <button
                id="newUserSubmit"
                class="btn btn-primary"
                type="submit"
                v-text="submitButtonMessage"
            >
            </button>
          </div>
        </div>
      </fieldset>
    </form>

    <p class="text-center" v-text="message" />
  </div>
</template>

<style scoped lang="scss">

</style>