<script setup lang="ts">

import {onMounted, Ref, ref, watch} from "vue";
import {AccountAlgorithm, createNewAccount, DraftAccount, ResponseType} from "../../../composables/Commands.ts";

interface displayEditor {
  [key: number]: boolean | undefined
}

const props = defineProps({
  accounts: {
    type: Array<DraftAccount>,
    required: true,
  }
})

const emit = defineEmits(['complete'])

const selectAll = ref(true);
const draftAccounts = ref([]) as Ref<DraftAccount[]>
const displayNameEditor = ref({}) as Ref<displayEditor>

async function confirmAccounts() {
  const approvedAccounts = draftAccounts.value.filter(draftAccount => draftAccount.import)
  const failedImports = [] as DraftAccount[];

  for (const approvedAccount of approvedAccounts) {
    const response = await createNewAccount(approvedAccount.name, approvedAccount.secret, approvedAccount.otp_digits, approvedAccount.totp_step, approvedAccount.algorithm ?? AccountAlgorithm.AUTODETECT)

    if (response.response === ResponseType.FAILURE) {
      failedImports.push(approvedAccount)
    }
  }

  emit('complete', {failed: failedImports.length, attempted: approvedAccounts.length})
}

function updateSelectAll() {
  draftAccounts.value.forEach(draftAccount => draftAccount.import = selectAll.value)
}

function openEditor(index: number) {
  displayNameEditor.value[index] = true
}

function closeEditor(index: number) {
  displayNameEditor.value[index] = false
}

function cloneAccounts() {
  draftAccounts.value = JSON.parse(JSON.stringify(props.accounts));
}

watch(
    () => props.accounts,
    () => cloneAccounts()
)

onMounted(() => cloneAccounts)

</script>

<template>
  <table
    v-if="draftAccounts.length"
    class="table table-striped"
  >
    <thead>
      <tr>
        <th>
          <div class="form-check">
            <input
              v-model="selectAll"
              class="form-check-input"
              type="checkbox"
              @change="updateSelectAll"
            >
            <label
              class="form-check-label"
              for="flexCheckDefault"
            />
          </div>
        </th>
        <th>
          Name
        </th>
      </tr>
    </thead>
    <tbody>
      <tr
        v-for="(draftAccount,index) in draftAccounts"
        :key="index"
      >
        <td>
          <div class="form-check">
            <input
              v-model="draftAccount.import"
              class="form-check-input"
              type="checkbox"
            >
            <label
              class="form-check-label"
              for="flexCheckDefault"
            />
          </div>
        </td>
        <td
          v-if="!displayNameEditor[index]"
          class="selector"
          @click="openEditor(index)"
        >
          <span
            :id="'account-display-' + index"
            v-text="draftAccount.name"
          />
        </td>
        <td v-if="displayNameEditor[index]">
          <div class="input-group">
            <input
              :id="'account-input-' + index"
              v-model="draftAccount.name"
              class="form-control"
              autofocus
            >
            <button
              class="btn btn-primary"
              type="button"
              @click="closeEditor(index)"
            >
              <i class="fa-solid fa-circle-check" />
            </button>
          </div>
        </td>
      </tr>
      <tr>
        <td colspan="2">
          <div class="text-center">
            <div class="d-grid gap-2">
              <button
                class="btn btn-success"
                type="button"
                @click="confirmAccounts"
              >
                <i class="fa-solid fa-circle-down"></i>
              </button>
            </div>
          </div>
        </td>
      </tr>
    </tbody>
  </table>
</template>

<style scoped lang="scss">

.selector:hover {
  background-color: rgba(0,0,0,.075);
}

</style>