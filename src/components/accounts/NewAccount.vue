<script setup lang="ts">
import {ref} from "vue";
import {createNewAccount, ResponseType} from "../../composables/Commands.ts";

const name = ref("");
const secret = ref("");
const message = ref("");

const emit = defineEmits(['created']);

async function submitForm() {
  const response = await createNewAccount(name.value, secret.value);

  if (response.response === ResponseType.SUCCESS) {
    emit('created')
  }

  message.value = response.message;
}

function shouldDisable() {
  return name.value.length === 0 || name.value.length > 255 || secret.value.length === 0
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
          for="name"
          class="form-label"
        >Name</label>
        <input
          id="name"
          v-model="name"
          class="form-control"
        >
      </div>

      <div class="mb-3">
        <label
          for="secret"
          class="form-label"
        >Secret</label>
        <input
          id="secret"
          v-model="secret"
          class="form-control"
        >
      </div>

      <div class="mb-3">
        <div class="d-grid gap-2">
          <button
            id="newUserSubmit"
            class="btn btn-primary"
            :disabled="shouldDisable()"
            type="submit"
          >
            Create Account
          </button>
        </div>
      </div>
    </form>

    <p v-text="message" />
  </div>
</template>
