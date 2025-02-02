<script setup lang="ts">
import {onMounted, ref, computed} from "vue";
import {
  AccountAlgorithm,
  attemptSyncAccounts,
  createNewAccount, editExistingAccount,
  getEditableAccount,
  ResponseType
} from "../../composables/Commands.ts";

const props = defineProps({
  accountId: {
    type: Number,
    default: null,
  }
})

const accountName = ref("");
const secret = ref("");
const digits = ref(6);
const timestep = ref(30);
const algorithm = ref(AccountAlgorithm.AUTODETECT);
const message = ref("");

const emit = defineEmits(['created', 'edited']);

const submitText = computed(() => props.accountId ? 'Edit Account' : 'Create Account')

async function submitForm() {
  if (props.accountId) {
    await editAccount();
    return;
  }

  await createAccount();
  return;
}

async function createAccount() {
  const response = await createNewAccount(accountName.value, secret.value, digits.value, timestep.value, algorithm.value);

  if (response.response === ResponseType.SUCCESS) {
    emit('created')

    await attemptSyncAccounts();
  }

  message.value = response.message;
}

async function editAccount() {
  if (!props.accountId) {
    return;
  }

  const response = await editExistingAccount(props.accountId, accountName.value, digits.value, timestep.value, algorithm.value);


  if (response.response === ResponseType.SUCCESS) {
    emit('edited')

    await attemptSyncAccounts();
  }

  message.value = response.message;
}

function shouldDisable() {
  if (props.accountId) {
    return accountName.value.length === 0 || accountName.value.length > 255
  }


  return accountName.value.length === 0 || accountName.value.length > 255 || secret.value.length === 0
}

onMounted(async () => {
  if (props.accountId) {
    const response = await getEditableAccount(props.accountId);

    accountName.value = response.account.name;
    digits.value = response.account.otp_digits;
    timestep.value = response.account.totp_step;

    if (response.account.algorithm) {
      algorithm.value = response.account.algorithm;
    }
  }
})
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
          v-model="accountName"
          class="form-control"
        >
      </div>

      <div
        v-if="!accountId"
        class="mb-3"
      >
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
        <div
          id=""
          class="pb-3"
        >
          <p class="form-check-label">
            Number of Digits
          </p>

          <div class="form-check form-check-inline">
            <input
              id="digits6"
              v-model="digits"
              class="form-check-input"
              type="radio"
              name="digits"
              :value="6"
              checked
            >
            <label
              class="form-check-label"
              for="digits6"
            >
              6 Digits
            </label>
          </div>
          <div class="form-check form-check-inline">
            <input
              id="digits8"
              v-model="digits"
              class="form-check-input"
              type="radio"
              name="digits"
              :value="8"
            >
            <label
              class="form-check-label"
              for="digits8"
            >
              8 Digits
            </label>
          </div>
        </div>

        <div
          id="timestep_group"
          class="pb-3"
        >
          <p class="form-check-label">
            Refresh Rate (timestep)
          </p>

          <div class="form-check form-check-inline">
            <input
              id="timeStep30"
              v-model="timestep"
              class="form-check-input"
              type="radio"
              name="timestep"
              :value="30"
              checked
            >
            <label
              class="form-check-label"
              for="timeStep30"
            >
              30 Seconds
            </label>
          </div>
          <div class="form-check form-check-inline">
            <input
              id="timeStep60"
              v-model="timestep"
              class="form-check-input"
              type="radio"
              name="timestep"
              :value="60"
            >
            <label
              class="form-check-label"
              for="timeStep60"
            >
              60 Seconds
            </label>
          </div>
          <div class="form-check form-check-inline">
            <input
              id="timeStep90"
              v-model="timestep"
              class="form-check-input"
              type="radio"
              name="timestep"
              :value="90"
            >
            <label
              class="form-check-label"
              for="timeStep90"
            >
              90 Seconds
            </label>
          </div>
          <div class="form-check form-check-inline">
            <input
              id="timeStep120"
              v-model="timestep"
              class="form-check-input"
              type="radio"
              name="timestep"
              :value="120"
            >
            <label
              class="form-check-label"
              for="timeStep120"
            >
              120 Seconds
            </label>
          </div>
        </div>

        <div class="pb-3">
          <p>2FA Algorithm</p>

          <div class="form-check form-check-inline">
            <input
              id="AutoDetect"
              v-model="algorithm"
              class="form-check-input"
              type="radio"
              name="algorithm"
              :value="AccountAlgorithm.AUTODETECT"
              checked
            >
            <label
              class="form-check-label"
              for="AutoDetect"
            >
              Auto Detect
            </label>
          </div>
          <div class="form-check form-check-inline">
            <input
              id="SHA1"
              v-model="algorithm"
              class="form-check-input"
              type="radio"
              name="algorithm"
              :value="AccountAlgorithm.SHA1"
            >
            <label
              class="form-check-label"
              for="SHA1"
            >
              SHA1
            </label>
          </div>
          <div class="form-check form-check-inline">
            <input
              id="SHA256"
              v-model="algorithm"
              class="form-check-input"
              type="radio"
              name="algorithm"
              :value="AccountAlgorithm.SHA256"
            >
            <label
              class="form-check-label"
              for="SHA256"
            >
              SHA256
            </label>
          </div>
          <div class="form-check form-check-inline">
            <input
              id="SHA512"
              v-model="algorithm"
              class="form-check-input"
              type="radio"
              name="algorithm"
              :value="AccountAlgorithm.SHA512"
            >
            <label
              class="form-check-label"
              for="SHA512"
            >
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
            {{ submitText }}
          </button>
        </div>
      </div>
    </form>

    <p v-text="message" />
  </div>
</template>
