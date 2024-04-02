<script setup lang="ts">
import {ref} from "vue";
import {AccountAlgorithm, createNewAccount, ResponseType} from "../../composables/Commands.ts";

const name = ref("");
const secret = ref("");
const digits = ref(6);
const timestep = ref(30);
const algorithm = ref(AccountAlgorithm.AUTODETECT);
const message = ref("");

const emit = defineEmits(['created']);

async function submitForm() {
  const response = await createNewAccount(name.value, secret.value, digits.value, timestep.value, algorithm.value);

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

      <div id="advanced">
        <div id="" class="pb-3">
          <p class="form-check-label">Number of Digits</p>

          <div class="form-check form-check-inline">
            <input v-model="digits" class="form-check-input" type="radio" name="digits" id="digits6" :value="6" checked>
            <label class="form-check-label" for="digits6">
              6 Digits
            </label>
          </div>
          <div class="form-check form-check-inline">
            <input v-model="digits" class="form-check-input" type="radio" name="digits" id="digits8" :value="8">
            <label class="form-check-label" for="digits8">
              8 Digits
            </label>
          </div>
        </div>

        <div id="timestep_group" class="pb-3">
          <p class="form-check-label">Refresh Rate (timestep)</p>

          <div class="form-check form-check-inline">
            <input v-model="timestep" class="form-check-input" type="radio" name="timestep" id="timeStep30" :value="30" checked>
            <label class="form-check-label" for="timeStep30">
              30 Seconds
            </label>
          </div>
          <div class="form-check form-check-inline">
            <input v-model="timestep" class="form-check-input" type="radio" name="timestep" id="timeStep60" :value="60">
            <label class="form-check-label" for="timeStep60">
              60 Seconds
            </label>
          </div>
          <div class="form-check form-check-inline">
            <input v-model="timestep"class="form-check-input" type="radio" name="timestep" id="timeStep90" :value="90">
            <label class="form-check-label" for="timeStep90">
              90 Seconds
            </label>
          </div>
          <div class="form-check form-check-inline">
            <input v-model="timestep" class="form-check-input" type="radio" name="timestep" id="timeStep120" :value="120">
            <label class="form-check-label" for="timeStep120">
              120 Seconds
            </label>
          </div>
        </div>

        <div class="pb-3">
          <p>2FA Algorithm</p>

          <div class="form-check form-check-inline">
            <input v-model="algorithm" class="form-check-input" type="radio" name="algorithm" id="AutoDetect" :value="AccountAlgorithm.AUTODETECT" checked>
            <label class="form-check-label" for="AutoDetect">
              Auto Detect
            </label>
          </div>
          <div class="form-check form-check-inline">
            <input v-model="algorithm" class="form-check-input" type="radio" name="algorithm" id="SHA1" :value="AccountAlgorithm.SHA1">
            <label class="form-check-label" for="SHA1">
              SHA1
            </label>
          </div>
          <div class="form-check form-check-inline">
            <input v-model="algorithm" class="form-check-input" type="radio" name="algorithm" id="SHA256" :value="AccountAlgorithm.SHA256">
            <label class="form-check-label" for="SHA256">
              SHA256
            </label>
          </div>
          <div class="form-check form-check-inline">
            <input v-model="algorithm"class="form-check-input" type="radio" name="algorithm" id="SHA512" :value="AccountAlgorithm.SHA512">
            <label class="form-check-label" for="SHA512">
              SHA512
            </label>
          </div>
        </div>
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
